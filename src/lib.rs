pub mod note;
pub mod enharmonic;
pub mod interval;
pub mod semitone;
pub mod pitch;
pub mod scales;
pub mod prelude;
pub mod set;
pub mod notation;
pub mod harmony;


// experimental features:

#[cfg(feature = "experimental-chords")]
pub mod chord;
// no need to even compile since it's not used anywhere else
// #[cfg(not(feature = "experimental-chords"))]
// mod chord;

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