use std::num::NonZeroU8;
use crate::letter::Letter;
use crate::note::Note;
use crate::pitch::Pitch;

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
}