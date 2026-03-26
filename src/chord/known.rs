use crate::Interval;
use Interval as I;

struct KnownChordData {
    name: &'static str,
    intervals: &'static [Interval],
}

macro_rules! define_known_chords {
    ($( $variant:ident { name: $name:expr, intervals: [$($ivl:expr),+ $(,)?] } ),* $(,)?) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
        #[non_exhaustive]
        #[repr(u16)]
        pub enum KnownChord { $($variant),* }

        static CHORD_DATA: &[KnownChordData] = &[
            $( KnownChordData { name: $name, intervals: &[$($ivl),+] } ),*
        ];
    };
}

define_known_chords! {
    // Triads
    MajorTriad { name: "major triad", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH] },
    MinorTriad { name: "minor triad", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH] },
    AugmentedTriad { name: "augmented triad", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::AUGMENTED_FIFTH] },
    DiminishedTriad { name: "diminished triad", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::DIMINISHED_FIFTH] },
    SuspendedSecond { name: "suspended 2nd", intervals: [I::PERFECT_UNISON, I::MAJOR_SECOND, I::PERFECT_FIFTH] },
    SuspendedFourth { name: "suspended 4th", intervals: [I::PERFECT_UNISON, I::PERFECT_FOURTH, I::PERFECT_FIFTH] },

    // Seventh chords
    MajorSeventh { name: "major seventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH] },
    DominantSeventh { name: "dominant seventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH] },
    MinorSeventh { name: "minor seventh", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH] },
    MinorMajorSeventh { name: "minor-major seventh", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH] },
    HalfDiminishedSeventh { name: "half-diminished seventh", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::DIMINISHED_FIFTH, I::MINOR_SEVENTH] },
    DiminishedSeventh { name: "diminished seventh", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::DIMINISHED_FIFTH, I::DIMINISHED_SEVENTH] },
    AugmentedSeventh { name: "augmented seventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::AUGMENTED_FIFTH, I::MINOR_SEVENTH] },
    AugmentedMajorSeventh { name: "augmented major seventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::AUGMENTED_FIFTH, I::MAJOR_SEVENTH] },
    DominantSeventhSuspendedFourth { name: "dominant seventh suspended 4th", intervals: [I::PERFECT_UNISON, I::PERFECT_FOURTH, I::PERFECT_FIFTH, I::MINOR_SEVENTH] },

    // Added tone chords
    MajorSixth { name: "major sixth", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SIXTH] },
    MinorSixth { name: "minor sixth", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SIXTH] },
    AddedNinth { name: "added ninth", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_NINTH] },
    SixNine { name: "six-nine", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SIXTH, I::MAJOR_NINTH] },

    // Ninth chords
    MajorNinth { name: "major ninth", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH, I::MAJOR_NINTH] },
    DominantNinth { name: "dominant ninth", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH] },
    MinorNinth { name: "minor ninth", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH] },
    MinorMajorNinth { name: "minor-major ninth", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH, I::MAJOR_NINTH] },
    DominantFlatNinth { name: "dominant flat ninth", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MINOR_NINTH] },
    DominantSharpNinth { name: "dominant sharp ninth", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::AUGMENTED_NINTH] },

    // Eleventh chords
    MajorEleventh { name: "major eleventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH] },
    DominantEleventh { name: "dominant eleventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH] },
    MinorEleventh { name: "minor eleventh", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH] },
    DominantSharpEleventh { name: "dominant sharp eleventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::AUGMENTED_ELEVENTH] },
    MajorSharpEleventh { name: "major sharp eleventh", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH, I::MAJOR_NINTH, I::AUGMENTED_ELEVENTH] },

    // Thirteenth chords
    MajorThirteenth { name: "major thirteenth", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MAJOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH, I::MAJOR_THIRTEENTH] },
    DominantThirteenth { name: "dominant thirteenth", intervals: [I::PERFECT_UNISON, I::MAJOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH, I::MAJOR_THIRTEENTH] },
    MinorThirteenth { name: "minor thirteenth", intervals: [I::PERFECT_UNISON, I::MINOR_THIRD, I::PERFECT_FIFTH, I::MINOR_SEVENTH, I::MAJOR_NINTH, I::PERFECT_ELEVENTH, I::MAJOR_THIRTEENTH] },
}

impl KnownChord {
    const fn data(self) -> &'static KnownChordData {
        &CHORD_DATA[self as usize]
    }

    pub(super) const fn name(self) -> &'static str {
        self.data().name
    }

    pub(super) const fn intervals(self) -> &'static [Interval] {
        self.data().intervals
    }
}
