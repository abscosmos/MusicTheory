//! Set class identification and Forte number classification.
//!
//! This module provides [`SetClass`] and [`SetClassForm`], which identify pitch class sets
//! by their equivalence class under transposition and inversion (TnI), using the
//! [Forte numbering system](https://en.wikipedia.org/wiki/Forte_number).
//!
//! - [`SetClass`]: identifies a set class without inversion distinction (e.g. `3-11`)
//! - [`SetClassForm`]: distinguishes inversion forms A and B (e.g. `3-11A`, `3-11B`)
//!
//! # Examples
//!
//! ```
//! # use music_theory::PitchClass;
//! # use music_theory::set::{PitchClassSet, SetClass, SetClassForm};
//! use PitchClass as PC;
//! // C major and C minor share the same set class (3-11)
//! let c_major = PitchClassSet::from_iter([PC::C, PC::E, PC::G]);
//! let c_minor = PitchClassSet::from_iter([PC::C, PC::Ds, PC::G]);
//!
//! assert_eq!(c_major.set_class(), c_minor.set_class());
//! assert_eq!(c_major.set_class().to_string(), "3-11");
//!
//! // ... but they have different inversion forms
//! assert_ne!(c_major.set_class_form(), c_minor.set_class_form());
//! assert_eq!(c_minor.set_class_form().to_string(), "3-11A");
//! assert_eq!(c_major.set_class_form().to_string(), "3-11B");
//! ```

mod set_class;
pub use set_class::*;

mod set_class_form;
pub use set_class_form::*;

mod tables;