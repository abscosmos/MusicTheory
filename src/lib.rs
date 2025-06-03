pub mod note;
pub mod enharmonic;
pub mod interval;
pub mod semitone;
pub mod chord;
pub mod pitch_class;
pub mod accidental;
pub mod pitch;
pub mod letter;
pub mod scales;
pub mod key;
pub mod analyze;
pub mod prelude;
pub mod clef;
pub mod octave_letter;
pub mod stem_direction;
pub mod pcset;
pub mod duration;
pub mod containers;
pub mod export;
/*
TODO:
    - add prelude
    - derive Copy where applicable and change &Self -> Self
    - full documentation
    - full tests
    - easy conversion using From for everything applicable
    - score stuff
    - tonal parity:
        - abc notation
        - scales & scale types
        - chords
        - pcsets
        - keys & modes
        - progressions & roman numerals
        - voicings
        - rhythm, time signatures, duration
    - music21 parity?
        - harmony
        - ...
*/