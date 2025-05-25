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
}