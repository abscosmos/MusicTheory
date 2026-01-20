mod note;
pub mod enharmonic;
pub mod interval;
mod semitone;
pub mod pitch;
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

// -- REEXPORTS --
pub use crate::interval::Interval;
pub use crate::pitch::{Pitch, PitchClass, AccidentalSign, Letter};
pub use crate::note::Note;
pub use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
pub use crate::semitone::Semitones;
