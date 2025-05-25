use std::num::NonZeroU8;
use std::ops::RangeInclusive;
use crate::letter::Letter;
use crate::note::Note;
use crate::pitch::Pitch;

#[derive(Copy, Clone, Debug)]
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
}

#[derive(Copy, Clone, Debug)]
pub struct PercussionClef;

// should this be represented as a clef?
#[derive(Copy, Clone, Debug)]
pub struct TablatureClef;

struct WrappingRange<T: Ord>(RangeInclusive<T>);

impl<T: Ord> WrappingRange<T> {
    pub fn new(domain: RangeInclusive<T>) -> Self {
        match domain.into_inner() {
            (start, end) if start <= end => Self(start..=end),
            (start, end) => Self(end..=start),
        }
    }
    
    pub fn contains(&self, range: RangeInclusive<T>, val: &T) -> bool {
        let start = range.start();
        let end = range.end();
        
        assert!(self.0.contains(val));
        assert!(self.0.contains(start));
        assert!(self.0.contains(end));
        
        if start <= end {
            range.contains(val)
        } else {
            val >= start || val <= end
        }
    }
}

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