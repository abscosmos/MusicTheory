use arrayvec::ArrayVec;
use crate::Pitch;
use crate::pitch::Letter;
use crate::set::PitchClassSet;

/// A 7-bit set of [`Letter`] values (C through B).
#[derive(Copy, Clone, Eq, PartialEq, Default)]
struct LetterSet(u8);

impl LetterSet {
    const EMPTY: Self = Self(0);

    pub const fn is_set(self, letter: Letter) -> bool {
        self.0 & Self::index(letter) != 0
    }

    pub const fn with_set(self, letter: Letter) -> Self {
        Self(self.0 | Self::index(letter))
    }

    #[inline(always)]
    const fn index(letter: Letter) -> u8 {
        1 << letter.step()
    }
}

/// Interval-number patterns to help identify chord root.
/// Each entry is a bitmask, bit N corresponds to a specific
/// [`interval::Number`](crate::interval::Number) present.
const PATTERNS: [u8; 6] = [
    0b0000_0010, // {1} - single note
    0b0000_1010, // {1, 3} - incomplete triad
    0b0010_1010, // {1, 3, 5} - triad
    0b1010_1010, // {1, 3, 5, 7} - seventh chord
    0b0011_0010, // {1, 4, 5} - sus4
    0b0010_0110, // {1, 2, 5} - sus2
];

/// Finds the root of a chord given its constituent pitches.
///
/// Uses a two-phase algorithm:
/// 1. Checks if any pitch produces a common interval pattern by letter distance.
///    Catches all standard triads, seventh chords, and sus chords.
/// 2. If no common pattern found, scores each candidate by psychoacoustic
///    root-support weights based on semitone intervals. ([Parncutt 1988](https://doi.org/10.2307/40285416))
///
/// Returns `None` if the input is empty.
pub fn find_root(pitches: impl Iterator<Item=Pitch>) -> Option<Pitch> {
    // deduplication
    let mut letter_seen = LetterSet::EMPTY;
    let mut by_letter = ArrayVec::<Pitch, 7>::new();

    let mut chroma_seen = PitchClassSet::EMPTY;
    let mut by_chroma = ArrayVec::<Pitch, 12>::new();

    for p in pitches {
        let letter = p.letter();
        if !letter_seen.is_set(letter) {
            letter_seen = letter_seen.with_set(letter);
            by_letter.push(p);
        }

        let pc = p.as_pitch_class();
        if !chroma_seen.is_set(pc) {
            chroma_seen = chroma_seen.with_set(pc);
            by_chroma.push(p);
        }

        if chroma_seen == PitchClassSet::CHROMATIC_AGGREGATE {
            break;
        }
    }

    match by_chroma.as_slice() {
        [] => return None,
        &[pitch] => return Some(pitch),
        _ => {}
    }

    // recognize based on shape, mostly stacked thirds
    for &candidate in &by_letter {
        let mask = by_letter.iter().fold(0u8, |mask, &p| {
            let number = candidate.letter().offset_between(p.letter()) + 1;
            mask | (1 << number)
        });

        if PATTERNS.contains(&mask) {
            return Some(candidate);
        }
    }

    parncutt_1988(by_chroma.iter().copied())
}

/// [Parncutt 1988](https://doi.org/10.2307/40285416) root-support scoring
///
/// For correctness, `pitches` must contain at most one of each [pitch class](crate::PitchClass).
fn parncutt_1988(pitches: impl Iterator<Item=Pitch> + Clone) -> Option<Pitch> {
    /// root-support weights, indexed by semitone distance
    /// scaled by 60 to avoid floating point arithmetic
    const PARNCUTT_WEIGHTS: [u16; 12] = [
        60, // P1
        0, // m2
        12, // M2
        6, // m3
        20, // M3
        0, // P4
        0, // TT
        30, // P5
        0, // m6
        0, // M6
        15, // m7
        0, // M7
    ];

    pitches.clone()
        .max_by_key(|&candidate|
            pitches.clone()
                .map(|p|
                    PARNCUTT_WEIGHTS[candidate.semitones_to(p).0 as usize]
                )
                .sum::<u16>()
        )
}