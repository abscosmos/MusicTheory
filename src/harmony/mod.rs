//! Harmony-related types including keys, modes, and scale degrees.
//!
//! The [`Key`] type represents a musical key, combining a [`tonic`](Key::tonic) pitch
//! with a [`mode`](Key::mode) (major, minor, etc.). It provides functionality for
//! working with key signatures, relative and parallel keys, and key transposition.
//!
//! # Examples
//! ```
//! # use music_theory::prelude::*;
//! // Create a key using common constructors
//! let c_major = Key::major(Pitch::C);
//! let e_minor = Key::minor(Pitch::E);
//!
//! // E minor has three flats
//! assert_eq!(e_minor.sharps(), -3)
//!
//! // Scale degree V in G major is D
//! let g_major = Key::major(Pitch::G);
//! assert_eq!(g_major.relative_pitch(ScaleDegree::V), Pitch::D);
//!
//! // Relative minor of E major is C# minor
//! let e_minor = Key::major(Pitch::E).relative(DiatonicMode::NATURAL_MINOR);
//! assert_eq!(e_minor, Key::minor(Pitch::C_SHARP));
//! ```

mod key;
pub use key::*;

mod mode;
pub use mode::*;

mod scale_degree;
pub use scale_degree::*;