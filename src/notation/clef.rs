use std::fmt;
use std::num::NonZeroU8;
use std::ops::RangeInclusive;
use crate::letter::Letter;
use crate::notation::StemDirection;
use crate::octave_letter::OctaveLetter;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PitchClef {
    // assuming there are only G, C, and F clefs
    // TODO: better field name?
    anchor: OctaveLetter,
    // are staffs always placed on the line?
    staff_line: NonZeroU8,
}

impl PitchClef {
    pub const TREBLE: Self = Self::new(OctaveLetter::new(Letter::G, 4), 2).expect("should be valid clef");
    pub const TREBLE_8VA: Self = Self::new(OctaveLetter::new(Letter::G, 5), 2).expect("should be valid clef");
    pub const TREBLE_8VB: Self = Self::new(OctaveLetter::new(Letter::G, 3), 2).expect("should be valid clef");
    pub const FRENCH_VIOLIN: Self = Self::new(OctaveLetter::new(Letter::G, 4), 1).expect("should be valid clef");
    pub const BASS: Self = Self::new(OctaveLetter::new(Letter::F, 3), 4).expect("should be valid clef");
    pub const BASS_8VA: Self = Self::new(OctaveLetter::new(Letter::F, 4), 4).expect("should be valid clef");
    pub const BASS_8VB: Self = Self::new(OctaveLetter::new(Letter::F, 2), 4).expect("should be valid clef");
    pub const SUB_BASS: Self = Self::new(OctaveLetter::new(Letter::F, 3), 5).expect("should be valid clef");
    pub const F_BARITONE: Self = Self::new(OctaveLetter::new(Letter::F, 3), 3).expect("should be valid clef");
    pub const SOPRANO: Self = Self::new(OctaveLetter::C4, 1).expect("should be valid clef");
    pub const MEZZO_SOPRANO: Self = Self::new(OctaveLetter::C4, 2).expect("should be valid clef");
    pub const ALTO: Self = Self::new(OctaveLetter::C4, 3).expect("should be valid clef");
    pub const TENOR: Self = Self::new(OctaveLetter::C4, 4).expect("should be valid clef");
    pub const C_BARITONE: Self = Self::new(OctaveLetter::C4, 5).expect("should be valid clef");

    pub const fn new(anchor: OctaveLetter, staff_line: u8) -> Option<Self> {
        // {range}.contains() isn't const
        let valid_line = 1 <= staff_line && staff_line <= 5;
        
        if matches!(anchor.letter, Letter::C | Letter::F | Letter::G) && valid_line {
            let staff_line = NonZeroU8::new(staff_line).expect("should be nonzero, already checked.");
            
            Some(Self{ anchor, staff_line })
        } else {
            None
        }
    }
    
    pub fn anchor(self) -> OctaveLetter {
        self.anchor
    }
    
    pub fn get_note(self, position: StaffPosition) -> OctaveLetter {
        match position {
            StaffPosition::Line(line) => {
                let line_offset = line as i16 - self.staff_line.get() as i16;

                self.anchor.with_offset(line_offset * 2)
            },
            StaffPosition::Space(space) => self.get_note(StaffPosition::Line(space)).with_offset(1),
        }
    }
    
    pub fn range(self) -> RangeInclusive<OctaveLetter> {
        self.get_note(StaffPosition::BOTTOM_LINE)..=self.get_note(StaffPosition::TOP_LINE)
    }
    
    pub fn contains(self, note: OctaveLetter) -> bool {
        self.range().contains(&note)
    }
    
    pub fn get_position(self, note: OctaveLetter) -> StaffPosition {
        let diff = self.anchor.offset_to(note);

        let val = self.staff_line.get() as i8 + diff.div_euclid(2) as i8;

        if diff % 2 == 0 {
            StaffPosition::Line(val)
        } else {
            StaffPosition::Space(val)
        }
    }
    
    pub fn stem_direction(self, notes: &[OctaveLetter], params: GetStemDirectionParams) -> Option<StemDirection> { 
        // adapted from https://www.music21.org/music21docs/moduleReference/moduleClef.html#music21.clef.Clef.getStemDirectionForPitches
        
        use GetStemDirectionParams as P;

        let mid_line = self.get_note(StaffPosition::Line(3));

        match notes {
            &[] => None,
            &[ol] => {
                if ol >= mid_line {
                    Some(StemDirection::Down)
                } else {
                    Some(StemDirection::Up)
                }
            }
            notes => {
                let slice = match params {
                    P::EndsOnly => {
                        let &[first, .. , last] = notes else {
                            unreachable!("branch ensures at least two elements")
                        };

                        &[first, last]
                    }
                    P::ExtremesOnly => {
                        let min = *notes.iter()
                            .min()
                            .expect("should be at least two elements");

                        let max = *notes.iter()
                            .max()
                            .expect("should be at least two elements");

                        &[min, max]
                    }
                    P::AllNotes => notes,
                };

                let sum = slice.iter()
                    .map(|ol|
                        mid_line.offset_to(*ol)
                    )
                    .sum::<i16>();

                if sum >= 0 {
                    Some(StemDirection::Down)
                } else {
                    Some(StemDirection::Up)
                }
            }
        }


    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum GetStemDirectionParams {
    /// Only the first note and last note are considered (Default)
    #[default]
    EndsOnly,
    /// Only the note furthest above the middle line and furthest below the middle line are considered.
    ExtremesOnly,
    /// All notes are considered
    AllNotes,
}

impl fmt::Display for PitchClef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TREBLE => write!(f, "Treble"),
            Self::TREBLE_8VA => write!(f, "Treble (8va)"),
            Self::TREBLE_8VB => write!(f, "Treble (8vb)"),
            Self::FRENCH_VIOLIN => write!(f, "French Violin"),
            Self::BASS => write!(f, "Bass"),
            Self::BASS_8VA => write!(f, "Bass (8va)"),
            Self::BASS_8VB => write!(f, "Bass (8vb)"),
            Self::SUB_BASS => write!(f, "Sub-bass"),
            Self::F_BARITONE => write!(f, "F Baritone"),
            Self::SOPRANO => write!(f, "Soprano"),
            Self::MEZZO_SOPRANO => write!(f, "Mezzo-soprano"),
            Self::ALTO => write!(f, "Alto"),
            Self::TENOR => write!(f, "Tenor"),
            Self::C_BARITONE => write!(f, "C Baritone"),
            _ => write!(f, "Custom ({} on line {})", self.anchor, self.staff_line),
        }
    }
}

/// The first line of the staff is Line(1), and the space above it is Space(1)
/// Line (0) would correspond to the first ledger line underneath the staff
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StaffPosition {
    Line(i8),
    Space(i8),
}

impl StaffPosition {
    pub const BOTTOM_LINE: Self = Self::Line(1);
    pub const TOP_LINE: Self = Self::Line(5);
}

#[derive(Copy, Clone, Debug)]
pub struct PercussionClef;

// should this be represented as a clef?
#[derive(Copy, Clone, Debug)]
pub struct TablatureClef;

#[cfg(test)]
mod tests {
    use crate::letter::Letter;
    use crate::octave_letter::OctaveLetter;
    use super::{PitchClef as Clef, StaffPosition as Pos, *};

    const ALL_CONSTS: [PitchClef; 14] = [
        Clef::TREBLE,
        Clef::TREBLE_8VA,
        Clef::TREBLE_8VB,
        Clef::FRENCH_VIOLIN,
        Clef::BASS,
        Clef::BASS_8VA,
        Clef::BASS_8VB,
        Clef::SUB_BASS,
        Clef::F_BARITONE,
        Clef::SOPRANO,
        Clef::MEZZO_SOPRANO,
        Clef::ALTO,
        Clef::TENOR,
        Clef::C_BARITONE,
    ];

    #[test]
    fn top_line() {
        assert_eq!(Clef::TREBLE.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::E, 4));
        assert_eq!(Clef::TREBLE_8VA.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::E, 5));
        assert_eq!(Clef::TREBLE_8VB.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::E, 3));
        assert_eq!(Clef::FRENCH_VIOLIN.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::G, 4));
        assert_eq!(Clef::BASS.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::G, 2));
        assert_eq!(Clef::BASS_8VA.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::G, 3));
        assert_eq!(Clef::BASS_8VB.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::G, 1));
        assert_eq!(Clef::SUB_BASS.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::E, 2));
        assert_eq!(Clef::F_BARITONE.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::B, 2));
        assert_eq!(Clef::SOPRANO.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::C, 4));
        assert_eq!(Clef::MEZZO_SOPRANO.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::A, 3));
        assert_eq!(Clef::ALTO.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::F, 3));
        assert_eq!(Clef::TENOR.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::D, 3));
        assert_eq!(Clef::C_BARITONE.get_note(Pos::BOTTOM_LINE), OctaveLetter::new(Letter::B, 2));
    }
    
    #[test]
    fn bottom_line() {
        assert_eq!(Clef::TREBLE.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::F, 5));
        assert_eq!(Clef::TREBLE_8VA.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::F, 6));
        assert_eq!(Clef::TREBLE_8VB.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::F, 4));
        assert_eq!(Clef::FRENCH_VIOLIN.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::A, 5));
        assert_eq!(Clef::BASS.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::A, 3));
        assert_eq!(Clef::BASS_8VA.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::A, 4));
        assert_eq!(Clef::BASS_8VB.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::A, 2));
        assert_eq!(Clef::SUB_BASS.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::F, 3));
        assert_eq!(Clef::F_BARITONE.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::C, 4));
        assert_eq!(Clef::SOPRANO.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::D, 5));
        assert_eq!(Clef::MEZZO_SOPRANO.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::B, 4));
        assert_eq!(Clef::ALTO.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::G, 4));
        assert_eq!(Clef::TENOR.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::E, 4));
        assert_eq!(Clef::C_BARITONE.get_note(Pos::TOP_LINE), OctaveLetter::new(Letter::C, 4));
    }

    #[test]
    fn test_get_position() {
        for clef in ALL_CONSTS {
            for v in -8..=8 {
                let line = StaffPosition::Line(v);

                assert_eq!(line, clef.get_position(clef.get_note(line)));

                let space = StaffPosition::Space(v);

                assert_eq!(space, clef.get_position(clef.get_note(space)));
            }
        }
    }
    
    #[test]
    fn test_stem_direction() {
        use GetStemDirectionParams as P;
        
        let c3 = OctaveLetter::new(Letter::C, 3);
        let b3 = OctaveLetter::new(Letter::B, 3);
        
        assert_eq!(P::default(), P::EndsOnly);
        
        assert_eq!(Clef::BASS.stem_direction(&[], P::default()), None);
        
        assert_eq!(
            Clef::BASS.stem_direction(&[c3], P::default()),
            Some(StemDirection::Up),
        );

        assert_eq!(
            Clef::BASS.stem_direction(&[c3, b3], P::default()),
            Some(StemDirection::Down),
        );

        assert_eq!(
            Clef::BASS.stem_direction(&[c3, b3, c3], P::default()),
            Some(StemDirection::Up),
        );

        assert_eq!(
            Clef::BASS.stem_direction(&[c3, b3, c3], P::AllNotes),
            Some(StemDirection::Down),
        );

        assert_eq!(
            Clef::BASS.stem_direction(&[c3, b3, c3], P::ExtremesOnly),
            Some(StemDirection::Down),
        );

        let c2 = OctaveLetter::new(Letter::C, 2);
        
        assert_eq!(
            Clef::BASS.stem_direction(&[c3, b3, c3, c2], P::ExtremesOnly),
            Some(StemDirection::Up),
        );
    }

}