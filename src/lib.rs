pub mod note;
pub mod enharmonic;
pub mod interval;
pub mod semitone;
pub mod pitch;
pub mod prelude;
pub mod set;
pub mod harmony;


// experimental features:

#[cfg(feature = "experimental-chords")]
pub mod chord;
// no need to compile it otherwise, since it's not used anywhere

#[cfg(feature = "experimental-scales")]
pub mod scales;
#[cfg(not(feature = "experimental-scales"))]
mod scales;

#[cfg(feature = "experimental-notation")]
pub mod notation;
// no need to compile it otherwise, since it's not used anywhere

#[cfg(feature = "experimental-note-gen")]
pub mod generator;
#[cfg(not(feature = "experimental-note-gen"))]
mod generator;

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