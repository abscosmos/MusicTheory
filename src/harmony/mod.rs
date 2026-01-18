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
//! let f_minor = Key::minor(Pitch::F);
//!
//! // F minor has four flats
//! assert_eq!(f_minor.sharps(), -4);
//!
//! // Scale degree VI in F minor is Db
//! assert_eq!(f_minor.relative_pitch(ScaleDegree::VI), Pitch::D_FLAT);
//!
//! // Relative major of F minor is Ab major
//! let ab_major = f_minor.relative(DiatonicMode::MAJOR);
//! assert_eq!(ab_major, Key::major(Pitch::A_FLAT));
//! ```

mod key;
pub use key::*;

mod mode;
pub use mode::*;

mod scale_degree;
pub use scale_degree::*;