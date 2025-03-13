pub mod note;
pub mod enharmonic;
pub mod interval;
pub mod semitone;
pub mod chord;
pub mod placed;
pub mod pitch_class;
pub mod accidental;
pub mod pitch;
pub mod letter;

/*
TODO:
    - add prelude
    - add pcset
    - derive Copy where applicable and change &Self -> Self
    - full documentation
    - full tests
    - easy conversion using From for everything applicable
*/