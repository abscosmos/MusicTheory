use std::num::NonZeroU8;
use std::ops::RangeInclusive;
use crate::letter::Letter;
use crate::note::Note;
use crate::octave_letter::OctaveLetter;
use crate::util::WrappingRange;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PitchClef {
    // assuming there are only G, C, and F clefs
    letter: Letter,
    // instead of storing the note that the middle line passes through, 
    // only store the octave, since the pitch is just the letter.
    octave: i16,
    // are staffs always placed on the line?
    staff_line: NonZeroU8,
}

impl PitchClef {
    pub const TREBLE: Self = Self::new(Letter::G, 4, 2).expect("should be valid clef");
    pub const TREBLE_8VA: Self = Self::new(Letter::G, 5, 2).expect("should be valid clef");
    pub const TREBLE_8VB: Self = Self::new(Letter::G, 3, 2).expect("should be valid clef");
    pub const FRENCH_VIOLIN: Self = Self::new(Letter::G, 4, 1).expect("should be valid clef");
    pub const BASS: Self = Self::new(Letter::F, 3, 4).expect("should be valid clef");
    pub const BASS_8VA: Self = Self::new(Letter::F, 4, 4).expect("should be valid clef");
    pub const BASS_8VB: Self = Self::new(Letter::F, 2, 4).expect("should be valid clef");
    pub const SUB_BASS: Self = Self::new(Letter::F, 3, 5).expect("should be valid clef");
    pub const F_BARITONE: Self = Self::new(Letter::F, 3, 3).expect("should be valid clef");
    pub const SOPRANO: Self = Self::movable_c(1).expect("should be valid clef");
    pub const MEZZO_SOPRANO: Self = Self::movable_c(2).expect("should be valid clef");
    pub const ALTO: Self = Self::movable_c(3).expect("should be valid clef");
    pub const TENOR: Self = Self::movable_c(4).expect("should be valid clef");
    pub const C_BARITONE: Self = Self::movable_c(5).expect("should be valid clef");

    pub const fn new(letter: Letter, octave: i16, staff_line: u8) -> Option<Self> {
        // {range}.contains() isn't const
        let valid_line = 1 <= staff_line && staff_line <= 5;
        
        if matches!(letter, Letter::C | Letter::F | Letter::G) && valid_line {
            let staff_line = NonZeroU8::new(staff_line).expect("should be nonzero, already checked.");
            
            Some(Self{ letter, octave, staff_line })
        } else {
            None
        }
    }

    const fn movable_c(staff_line: u8) -> Option<Self> {
        Self::new(Letter::C, 4, staff_line)
    }
    
    // FIXME: this should really return a Letter with an octave
    pub fn get_note(self, position: StaffPosition) -> OctaveLetter {
        match position {
            StaffPosition::Line(line) => self.line(line),
            StaffPosition::Space(space) => self.space(space),
        }
    }
    
    pub fn range(self) -> RangeInclusive<OctaveLetter> {
        self.get_note(StaffPosition::BOTTOM_LINE)..=self.get_note(StaffPosition::TOP_LINE)
    }
    
    pub fn contains(self, note: OctaveLetter) -> bool {
        self.range().contains(&note)
    }
    
    fn line(self, line: i8) -> OctaveLetter {
        let letter_delta =  2 * (line - self.staff_line.get() as i8);

        let res_letter = (self.letter.step() as i8 + letter_delta).rem_euclid(7) as _;

        let res_letter = Letter::from_step(res_letter).expect("% 7 is in range [0,6]");

        let oct_1 = letter_delta / 7;

        let letter_range = WrappingRange::new(Letter::C..=Letter::B);

        let range = if letter_delta > 0 {
            self.letter..=res_letter
        } else {
            res_letter..=self.letter
        };
        
        let oct_2 = if *range.start() != Letter::C && letter_range.contains(range, &Letter::C) {
            letter_delta.signum()
        } else {
            0
        };

        OctaveLetter::new(res_letter, self.octave + (oct_1 + oct_2) as i16)
    }
    
    fn space(self, space: i8) -> OctaveLetter {
        let line_above = self.line(space + 1);
        
        let new_step = line_above.letter.step() + 6; // +6 == -1 (mod 7)
        
        let letter = Letter::from_step(new_step % 7)
            .expect("% 7 is in range [0,6]");
        
        let octave = if letter != Letter::B {
            line_above.octave
        } else {
            line_above.octave - 1
        };

        OctaveLetter::new(letter, octave)
    }
    
    pub fn get_position(note: Note) -> StaffPosition {
        todo!()
    }

    // TODO: for a note or multiple notes, return if the stem should point up or down
    // https://www.music21.org/music21docs/moduleReference/moduleClef.html#music21.clef.Clef.getStemDirectionForPitches
    pub fn stem_direction(notes: impl IntoIterator<Item = Note>) -> () {
        todo!()
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
    use super::{PitchClef as Clef, StaffPosition as Pos};

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
}