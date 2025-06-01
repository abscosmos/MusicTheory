use std::num::NonZeroU8;
use std::ops::RangeInclusive;
use crate::letter::Letter;
use crate::note::Note;
use crate::pitch::Pitch;
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

    pub fn bottom_line(self) -> Note {
        self.line(1).expect("1 is a valid line")
    }

    pub fn top_line(self) -> Note {
        self.line(5).expect("5 is a valid line")
    }
    
    pub fn range(self) -> RangeInclusive<Note> {
        self.bottom_line()..=self.top_line()
    }
    
    pub fn contains(self, note: Note) -> bool {
        self.range().contains(&note)
    }
    
    pub fn line(self, line: u8) -> Option<Note> {
        if !(1..=5).contains(&line) {
            return None;
        }
        
        let letter_delta =  2 * (line as i8 - self.staff_line.get() as i8);

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

        let oct_adj = oct_1 + oct_2;

        Some(Note::new(res_letter.into(), self.octave + oct_adj as i16))
    }
    
    pub fn space(self, space: u8) -> Option<Note> {
        if !(1..=4).contains(&space) {
            return None;
        }
        
        let line_above = self.line(space + 1).expect("x in [1,4] + 1 should be in [1,5]");
        
        let new_step = line_above.letter().step() + 6; // +6 == -1 (mod 7)
        
        let letter = Letter::from_step(new_step % 7)
            .expect("% 7 is in range [0,6]");
        
        let octave = if letter != Letter::B {
            line_above.octave
        } else {
            line_above.octave - 1
        };

        Some(Note::new(letter.into(), octave))
    }

    // TODO: for a note or multiple notes, return if the stem should point up or down
    // https://www.music21.org/music21docs/moduleReference/moduleClef.html#music21.clef.Clef.getStemDirectionForPitches
    pub fn stem_direction() -> () {
        todo!()
    }
}

/// The first line of the staff is Line(1), and the space above it is Space(1)
/// Line (0) would correspond to the first ledger line underneath the staff
pub enum StaffPosition {
    Line(i8),
    Space(i8),
}

#[derive(Copy, Clone, Debug)]
pub struct PercussionClef;

// should this be represented as a clef?
#[derive(Copy, Clone, Debug)]
pub struct TablatureClef;

#[cfg(test)]
mod tests {
    use crate::note::Note;
    use crate::pitch::Pitch;
    use super::PitchClef as Clef;

    #[test]
    fn lines() {
        assert_eq!(Clef::TREBLE.bottom_line(), Note::new(Pitch::E, 4));
        assert_eq!(Clef::TREBLE_8VA.bottom_line(), Note::new(Pitch::E, 5));
        assert_eq!(Clef::TREBLE_8VB.bottom_line(), Note::new(Pitch::E, 3));
        assert_eq!(Clef::FRENCH_VIOLIN.bottom_line(), Note::new(Pitch::G, 4));
        assert_eq!(Clef::BASS.bottom_line(), Note::new(Pitch::G, 2));
        assert_eq!(Clef::BASS_8VA.bottom_line(), Note::new(Pitch::G, 3));
        assert_eq!(Clef::BASS_8VB.bottom_line(), Note::new(Pitch::G, 1));
        assert_eq!(Clef::SUB_BASS.bottom_line(), Note::new(Pitch::E, 2));
        assert_eq!(Clef::F_BARITONE.bottom_line(), Note::new(Pitch::B, 2));
        assert_eq!(Clef::SOPRANO.bottom_line(), Note::new(Pitch::C, 4));
        assert_eq!(Clef::MEZZO_SOPRANO.bottom_line(), Note::new(Pitch::A, 3));
        assert_eq!(Clef::ALTO.bottom_line(), Note::new(Pitch::F, 3));
        assert_eq!(Clef::TENOR.bottom_line(), Note::new(Pitch::D, 3));
        assert_eq!(Clef::C_BARITONE.bottom_line(), Note::new(Pitch::B, 2));

        assert_eq!(Clef::TREBLE.top_line(), Note::new(Pitch::F, 5));
        assert_eq!(Clef::TREBLE_8VA.top_line(), Note::new(Pitch::F, 6));
        assert_eq!(Clef::TREBLE_8VB.top_line(), Note::new(Pitch::F, 4));
        assert_eq!(Clef::FRENCH_VIOLIN.top_line(), Note::new(Pitch::A, 5));
        assert_eq!(Clef::BASS.top_line(), Note::new(Pitch::A, 3));
        assert_eq!(Clef::BASS_8VA.top_line(), Note::new(Pitch::A, 4));
        assert_eq!(Clef::BASS_8VB.top_line(), Note::new(Pitch::A, 2));
        assert_eq!(Clef::SUB_BASS.top_line(), Note::new(Pitch::F, 3));
        assert_eq!(Clef::F_BARITONE.top_line(), Note::new(Pitch::C, 4));
        assert_eq!(Clef::SOPRANO.top_line(), Note::new(Pitch::D, 5));
        assert_eq!(Clef::MEZZO_SOPRANO.top_line(), Note::new(Pitch::B, 4));
        assert_eq!(Clef::ALTO.top_line(), Note::new(Pitch::G, 4));
        assert_eq!(Clef::TENOR.top_line(), Note::new(Pitch::E, 4));
        assert_eq!(Clef::C_BARITONE.top_line(), Note::new(Pitch::C, 4));
    }
}