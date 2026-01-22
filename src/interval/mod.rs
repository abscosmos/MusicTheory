//! Musical intervals and their components.
//!
//! An [`Interval`] represents the distance between two pitches, combining a
//! [`quality`](Quality) (major, minor, perfect, etc.) with a
//! [`number`](Number) (unison, second, third, etc.).
//!
//! # Examples
//!
//! ```
//! # use music_theory::Interval;
//! # use music_theory::interval::Stability;
//!
//! // Create intervals using constants
//! let major_third = Interval::MAJOR_THIRD;
//! let perfect_fifth = Interval::PERFECT_FIFTH;
//!
//! // Intervals can be combined
//! assert_eq!(
//!     Interval::MAJOR_THIRD + Interval::MINOR_THIRD,
//!     Interval::PERFECT_FIFTH
//! );
//!
//! // Check harmonic stability
//! assert_eq!(
//!     Interval::MAJOR_THIRD.stability(),
//!     Some(Stability::ImperfectConsonance)
//! );
//! ```

use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::num::{NonZeroI16, NonZeroU16};
use std::ops::{Add, Mul, Neg, Sub};
use std::str::FromStr;
use crate::enharmonic::WithoutSpelling;
use crate::{Note, Pitch, Semitones, EnharmonicEq, EnharmonicOrd};
use crate::enharmonic;

mod quality;
pub use quality::*;

mod number;
pub use number::*;

mod stability;
pub use stability::*;

mod consts;

#[cfg(test)]
mod tests;

/// A musical interval representing the distance between two musical objects.
///
/// An interval combines a [`quality`](Quality) (major, minor, perfect, etc.)
/// with a [`number`](Number) (unison, second, third, etc.). Intervals can either
/// be ascending or descending.
///
/// Two intervals may be similar, but spelled differently. For example, a [diminished fifth](Self::DIMINISHED_FIFTH)
/// and an [augmented fourth](Self::AUGMENTED_FOURTH) are different ways to spell a tritone, but
/// are both 6 semitones wide. To compare intervals in a spelling agnostic way, use the
/// [`EnharmonicOrd`] and [`EnharmonicEq`] traits.
///
/// For convenience, constants such as [`Interval::MAJOR_SIXTH`] are provided,
/// of all qualities up to number 15.
///
/// # Examples
///
/// ```
/// # use music_theory::{Pitch, Interval, Semitones};
/// # use music_theory::interval::{Quality, Number};
/// // Create using constants
/// let major_third = Interval::MAJOR_THIRD;
/// assert_eq!(major_third.semitones(), Semitones(4));
///
/// // Create from quality and number
/// let minor_sixth = Interval::new(
///     Quality::Minor,
///     Number::SIXTH
/// ).unwrap();
///
/// // Parse from shorthand notation
/// let perfect_fifth: Interval = "P5".parse().unwrap();
/// assert_eq!(perfect_fifth, Interval::PERFECT_FIFTH);
///
/// // Calculate intervals between pitches
/// let interval = Interval::between_pitches(Pitch::C, Pitch::E);
/// assert_eq!(interval, Interval::MAJOR_THIRD);
///
/// // Intervals can be added and subtracted
/// assert_eq!(
///     Interval::MAJOR_THIRD + Interval::MINOR_THIRD,
///     Interval::PERFECT_FIFTH
/// );
///
/// // ... or scaled, equivalent to repeated addition
/// assert_eq!(
///     Interval::MINOR_THIRD * 3,
///     Interval::DIMINISHED_SEVENTH,
/// );
///
/// // Intervals can be inverted
/// assert_eq!(Interval::MAJOR_THIRD.inverted(), Interval::MINOR_SIXTH);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Interval {
    /// The quality of the interval.
    quality: Quality,
    /// The diatonic size of the interval.
    number: Number,
}

impl Interval {
    /// Creates a new interval from a quality and number.
    ///
    /// Returns `None` if the quality-number combination is invalid:
    /// - Perfect quality requires a perfect interval number (1, 4, 5, 8)
    /// - Major/minor qualities require a major/minor interval number (2, 3, 6, 7)
    /// - Augmented/diminished can be used with any interval number
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::{Quality, Number};
    /// assert_eq!(
    ///     Interval::new(Quality::Perfect, Number::FIFTH),
    ///     Some(Interval::PERFECT_FIFTH),
    /// );
    ///
    /// assert_eq!(
    ///     Interval::new(Quality::Major, Number::FIFTH),
    ///     None,
    /// );
    /// ```
    pub fn new(quality: Quality, number: Number) -> Option<Self> {
        let unchecked = Some(Self { quality, number });

        match quality {
            Quality::Diminished(_) | Quality::Augmented(_) => unchecked,
            Quality::Perfect if number.is_perfect() => unchecked,
            Quality::Minor | Quality::Major if !number.is_perfect() => unchecked,
            _ => None,
        }
    }
    
    /// Creates a new interval with major or perfect quality.
    ///
    /// Returns a [perfect](Quality::Perfect) interval for perfect interval numbers (1, 4, 5, 8),
    /// and a [major](Quality::Major) interval for major/minor interval numbers (2, 3, 6, 7).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::Number;
    /// assert_eq!(
    ///     Interval::new_maj_or_perfect(Number::FIFTH),
    ///     Interval::PERFECT_FIFTH
    /// );
    ///
    /// assert_eq!(
    ///     Interval::new_maj_or_perfect(Number::THIRD),
    ///     Interval::MAJOR_THIRD
    /// );
    /// ```
    pub fn new_maj_or_perfect(number: Number) -> Self {
        let quality = if number.is_perfect() {
            Quality::Perfect
        } else {
            Quality::Major
        };
        
        Self { quality, number }
    }

    /// Creates a new interval, returning `None` if it's subzero.
    ///
    /// A subzero interval is either an ascending interval whose quality makes it span a descending
    /// semitone size, or a descending interval whose quality makes it span an ascending semitone size.
    /// For more information, see [`Self::is_subzero`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::{Quality, Number};
    /// assert_eq!(
    ///     Interval::strict_non_subzero(Quality::Perfect, Number::FIFTH),
    ///     Some(Interval::PERFECT_FIFTH),
    /// );
    ///
    /// // Subzero interval (diminished unison is -1 semitones)
    /// assert_eq!(
    ///     Interval::strict_non_subzero(Quality::DIMINISHED, Number::UNISON),
    ///     None,
    /// );
    /// ```
    pub fn strict_non_subzero(quality: Quality, number: Number) -> Option<Self> {
         Self::new(quality, number).filter(|ivl| !ivl.is_subzero())
    }

    /// Calculates the interval between two notes.
    ///
    /// If `lhs` is lower than `rhs`, the result is an ascending interval. If `lhs` is greater,
    /// the result is descending. The interval between identical notes is always a
    /// positive [perfect unison](Self::PERFECT_UNISON).
    ///
    /// # Examples
    /// ```
    /// # use music_theory::{Pitch, Note, Interval};
    /// let c4 = Note::new(Pitch::C, 4);
    /// let e4 = Note::new(Pitch::E, 4);
    /// let a5 = Note::new(Pitch::A, 5);
    ///
    /// assert_eq!(
    ///     Interval::between_notes(c4, e4),
    ///     Interval::MAJOR_THIRD
    /// );
    ///
    /// assert_eq!(
    ///     Interval::between_notes(e4, c4),
    ///     -Interval::MAJOR_THIRD
    /// );
    ///
    /// assert_eq!(
    ///     Interval::between_notes(c4, a5),
    ///     Interval::MAJOR_THIRTEENTH
    /// );
    /// ```
    // TODO: test if this is correct for subzero intervals
    pub fn between_notes(lhs: Note, rhs: Note) -> Interval {
        let (lhs, rhs, descending) = match lhs.cmp(&rhs) {
            Ordering::Equal => return Self::PERFECT_UNISON,
            Ordering::Less => (lhs, rhs, false),
            Ordering::Greater => (rhs, lhs, true),
        };
        
        let base_interval = Self::between_pitches(lhs.pitch, rhs.pitch);

        let diff = lhs.semitones_to(rhs) - base_interval.semitones();

        assert!(diff.0 >= 0, "after reordering, the difference should be positive or zero");
        
        const OCTAVE_SEMITONES: i16 = 12;

        assert_eq!(diff.0 % OCTAVE_SEMITONES, 0, "should just be off by multiples of an octave");

        let octaves = diff.0 / 12;

        let new_number = NonZeroI16::new(base_interval.number().get() + 7 * octaves)
            .expect("nonzero shouldn't become zero if adding away from zero; shouldn't overflow either");
        
        let signed_number = if descending { -new_number } else { new_number };

        Interval::new(base_interval.quality(), Number(signed_number))
            .expect("quality should still be valid")
    }
    
    /// Calculates the ascending interval between two pitches.
    ///
    /// Always returns an ascending, simple interval.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::{Pitch, Interval};
    /// assert_eq!(
    ///     Interval::between_pitches(Pitch::C, Pitch::E),
    ///     Interval::MAJOR_THIRD
    /// );
    ///
    /// assert_eq!(
    ///     Interval::between_pitches(Pitch::E, Pitch::C),
    ///     Interval::MINOR_SIXTH
    /// );
    /// ```
    // TODO: test with intervals where quality makes it more than 2 octaves
    pub fn between_pitches(lhs: Pitch, rhs: Pitch) -> Self {
        let lhs_letter = lhs.letter();
        let rhs_letter = rhs.letter();

        let number = lhs_letter.offset_between(rhs_letter) + 1;
        
        let number = Number::new(number as _)
            .expect("can't be zero since offset_between returns [0, 6], and adding one");

        let number = if number == Number::UNISON && lhs > rhs { Number::OCTAVE } else { number };

        let base_semitones = lhs.semitones_to(rhs).0;
        
        let semitones = if lhs_letter.offset_between(rhs_letter) == 6 && base_semitones == 0 {
            base_semitones + 12
        } else {
            base_semitones
        };
        
        let quality = match semitones - number.base_semitones_with_octave_unsigned() {
            -1 if number.is_perfect() => Quality::DIMINISHED,
            -1 => Quality::Minor,
            0 if number.is_perfect() => Quality::Perfect,
            0 => Quality::Major,
            n @ 1.. => Quality::Augmented((n as u16).try_into().expect("can't be zero")),
            n @ ..-1 if number.is_perfect() => Quality::Diminished(NonZeroU16::new(-n as u16).expect("shouldn't be zero, as the first arm should've caught that")),
            n @ ..-1 => Quality::Diminished(NonZeroU16::new(-n as u16 - 1).expect("shouldn't be zero, as the first arm should've caught that")),
        };
        
        Interval::new(quality, number).expect("should be valid")
    }

    /// Returns the harmonic stability of the interval.
    ///
    /// Intervals are either perfect or imperfect consonances, or dissonances
    /// based on traditional music theory principles of harmonic stability.
    ///
    /// # Returns
    ///
    /// Returns `None` for perfect fourths, which are consonant melodically but dissonant
    /// harmonically. All other intervals return `Some(Stability)`.
    /// See [Stability] for what intervals fall in each category.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use music_theory::Interval;
    /// # use music_theory::interval::Stability;
    /// // Perfect consonances
    /// assert_eq!(Interval::PERFECT_FIFTH.stability(), Some(Stability::PerfectConsonance));
    ///
    /// // Imperfect consonances
    /// assert_eq!(Interval::MAJOR_THIRD.stability(), Some(Stability::ImperfectConsonance));
    ///
    /// // Dissonances
    /// assert_eq!(Interval::MAJOR_SECOND.stability(), Some(Stability::Dissonance));
    /// assert_eq!(Interval::AUGMENTED_FOURTH.stability(), Some(Stability::Dissonance));
    ///
    /// // Perfect fourth is ambiguous
    /// assert_eq!(Interval::PERFECT_FOURTH.stability(), None);
    /// ```
    pub fn stability(self) -> Option<Stability> {
        match self.quality {
            Quality::Diminished(_) | Quality::Augmented(_) => Some(Stability::Dissonance),
            _ => match self.number.as_simple() {
                Number::UNISON | Number::FIFTH | Number::OCTAVE => Some(Stability::PerfectConsonance),
                Number::THIRD | Number::SIXTH => Some(Stability::ImperfectConsonance),
                Number::SECOND | Number::SEVENTH => Some(Stability::Dissonance),
                Number::FOURTH => None,
                _ => unreachable!("as_simple should return number in [1,8]"),
            }
        }
    }

    /// Returns `true` if this is a subzero interval.
    ///
    /// A subzero interval is either an ascending interval whose quality makes it span a descending
    /// semitone size, or a descending interval whose quality makes it span an ascending semitone size.
    ///
    /// For example, a doubly-diminished second is ascending, but spans -1 semitones. As such,
    /// transposing a note by it will result in a note lower than the starting note, despite the
    /// interval being ascending.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::{Quality, Number};
    /// assert!(!Interval::PERFECT_UNISON.is_subzero());
    /// assert!(!Interval::MAJOR_THIRD.is_subzero());
    ///
    /// // Diminished unison is subzero (-1 semitones)
    /// let dim_unison = Interval::new(Quality::DIMINISHED, Number::UNISON)
    ///     .expect("valid number & quality combination");
    ///
    /// assert!(dim_unison.is_subzero());
    /// assert!(dim_unison.is_ascending() && dim_unison.semitones().is_negative());
    /// ```
    // TODO: does this work for descending intervals?
    pub fn is_subzero(self) -> bool {
        self.quality.is_diminished()
            && self.semitones() != Semitones::UNISON
            && self.semitones().is_positive() != self.is_ascending()
    }
    
    /// Expands a subzero interval into an equivalent non-subzero compound interval.
    ///
    /// Adds octaves to a subzero interval until it spans in the expected direction.
    /// For non-subzero intervals, returns the interval unchanged.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::{Quality, Number};
    /// // Non-subzero intervals remain unchanged
    /// assert_eq!(
    ///     Interval::PERFECT_FIFTH.expand_subzero(),
    ///     Interval::PERFECT_FIFTH
    /// );
    ///
    /// // Diminished unison is subzero (-1 semitones)
    /// let dim_unison = Interval::new(Quality::DIMINISHED, Number::UNISON)
    ///     .expect("valid number & quality combination");
    ///
    /// let expanded = dim_unison.expand_subzero();
    /// assert!(!expanded.is_subzero());
    /// assert_eq!(expanded, Interval::DIMINISHED_OCTAVE);
    /// ```
    // TODO: add tests for this function
    // TODO: ensure this works for descending intervals
    pub fn expand_subzero(self) -> Self {
        if !self.is_subzero() {
            return self;
        }

        const OCTAVE_SEMITONES: i16 = 12;

        let semitones = self.semitones().0;

        let octaves = -semitones.div_euclid(OCTAVE_SEMITONES);
        
        let new_number = Number::new(self.number().get() + octaves * 7)
            .expect("shouldn't be zero to begin with");
        
        let expanded = Self::strict_non_subzero(self.quality, new_number)
            .expect("should be valid quality and not subzero");
        
        debug_assert!(expanded.semitones().0 < OCTAVE_SEMITONES, "expanded shouldn't be more than an octave");
        
        expanded
    }

    /// Returns the quality of the interval.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::Quality;
    /// assert_eq!(Interval::MAJOR_THIRD.quality(), Quality::Major);
    /// assert_eq!(Interval::PERFECT_FIFTH.quality(), Quality::Perfect);
    /// ```
    pub fn quality(self) -> Quality {
        self.quality
    }

    /// Returns the number (diatonic size) of the interval.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::Number;
    /// assert_eq!(Interval::MAJOR_THIRD.number(), Number::THIRD);
    /// assert_eq!(Interval::PERFECT_FIFTH.number(), Number::FIFTH);
    /// ```
    pub fn number(self) -> Number {
        self.number
    }

    /// Returns the number of semitones spanned by this interval.
    ///
    /// Positive for ascending intervals, negative for descending intervals.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::{Interval, Semitones};
    /// assert_eq!(Interval::PERFECT_FIFTH.semitones(), Semitones(7));
    /// // Descending intervals are negative
    /// assert_eq!((-Interval::MAJOR_THIRD).semitones(), Semitones(-4));
    /// ```
    pub fn semitones(self) -> Semitones {
        let base_oct_semitones = self.number.base_semitones_with_octave_unsigned();

        let quality_adjust = match self.quality {
            Quality::Perfect | Quality::Major => 0,
            Quality::Minor => -1,

            Quality::Diminished(n) => if self.number.is_perfect() {
                -(n.get() as i16)
            } else {
                -(n.get() as i16 + 1)
            }

            Quality::Augmented(n) => n.get() as i16,
        };

        let unsigned = base_oct_semitones + quality_adjust;

        Semitones(unsigned * self.number.get().signum())
    }

    /// Returns the shorthand notation for the interval.
    ///
    /// Combines quality shorthand (P, M, m, d, A) with the interval number.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// assert_eq!(Interval::PERFECT_FIFTH.shorthand(), "P5");
    /// assert_eq!(Interval::MINOR_SEVENTH.shorthand(), "m7");
    /// assert_eq!(Interval::DIMINISHED_FIFTH.shorthand(), "d5");
    /// ```
    pub fn shorthand(self) -> String {
        format!("{}{}", self.quality.shorthand(), self.number.get())
    }

    /// Returns the inverted interval.
    ///
    /// Flips the quality and the number of the interval. See [`Quality::inverted`] and
    /// [`Number::inverted`] for more information.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// assert_eq!(
    ///     Interval::MAJOR_THIRD.inverted(),
    ///     Interval::MINOR_SIXTH
    /// );
    ///
    /// assert_eq!(
    ///     Interval::PERFECT_FIFTH.inverted(),
    ///     Interval::PERFECT_FOURTH
    /// );
    ///
    /// assert_eq!(
    ///     Interval::AUGMENTED_FOURTH.inverted(),
    ///     Interval::DIMINISHED_FIFTH
    /// );
    /// ```
    pub fn inverted(self) -> Self {
        Self::new(self.quality.inverted(), self.number.inverted())
            .expect("valid quality")
    }

    /// Returns the inverted interval if it's not subzero.
    ///
    /// For information on what a subzero interval is, see [`Self::is_subzero`].
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::{Quality, Number};
    /// // Normal intervals invert successfully
    /// assert!(Interval::MAJOR_THIRD.inverted_strict_non_subzero().is_some());
    ///
    /// let doubly_augmented_second = Interval::new(
    ///     Quality::Augmented(2.try_into().unwrap()),
    ///     Number::SEVENTH
    /// )
    /// .expect("valid quality & number combination");
    ///
    /// // the inverted interval is a dd2, which is subzero
    /// assert_eq!(
    ///     doubly_augmented_second.inverted_strict_non_subzero(),
    ///     None,
    /// )
    /// ```
    pub fn inverted_strict_non_subzero(self) -> Option<Self> {
        match self.inverted() {
            ivl if !ivl.is_subzero() => Some(ivl),
            _ => None,
        }
    }

    /// Creates an interval from a number of semitones using preferred spellings.
    ///
    /// Spells intervals with major/minor/perfect, except for a [diminished fifth](Self::DIMINISHED_FIFTH)
    /// (tritone, 6 semitones), which cannot be spelled as a major/minor/perfect interval.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::{Interval, Semitones};
    /// assert_eq!(
    ///     Interval::from_semitones_preferred(Semitones(4)),
    ///     Interval::MAJOR_THIRD
    /// );
    ///
    /// // Tritone prefers diminished fifth
    /// assert_eq!(
    ///     Interval::from_semitones_preferred(Semitones(6)),
    ///     Interval::DIMINISHED_FIFTH
    /// );
    /// ```
    pub fn from_semitones_preferred(semitones: Semitones) -> Self {
        let semi = semitones.0;

        if semi == 0 {
            return Self::PERFECT_UNISON;
        }

        let semi_adj = (semi.abs() - 1) % 12;

        let (quality, base_num) = match semi_adj + 1 {
            1 => (Quality::Minor, 2),
            2 => (Quality::Major, 2),
            3 => (Quality::Minor, 3),
            4 => (Quality::Major, 3),
            5 => (Quality::Perfect, 4),
            6 => (Quality::DIMINISHED, 5),
            7 => (Quality::Perfect, 5),
            8 => (Quality::Minor, 6),
            9 => (Quality::Major, 6),
            10 => (Quality::Minor, 7),
            11 => (Quality::Major, 7),
            12 => (Quality::Perfect, 8),
            _ => unreachable!("should be in range [1,12]"),
        };

        let oct = (semi.abs() - 1) / 12;

        let unsigned = base_num + oct * 7;

        let number = Number::new(unsigned * semi.signum())
            .expect("non zero");

        Self::new(quality, number)
            .expect("valid quality")
    }
    
    /// Returns `true` if the interval is ascending.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// assert!(Interval::MAJOR_THIRD.is_ascending());
    /// assert!(!(-Interval::MAJOR_THIRD).is_ascending());
    /// ```
    pub fn is_ascending(self) -> bool {
        self.number.is_ascending()
    }

    /// Returns the interval with the specified direction.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// let m3 = Interval::MAJOR_THIRD;
    ///
    /// assert_eq!(m3.with_direction(true), m3);
    /// assert_eq!(m3.with_direction(false), -m3);
    /// ```
    pub fn with_direction(self, ascending: bool) -> Self {
        Self {
            number: self.number.with_direction(ascending),
            .. self
        }
    }

    /// Reduces a compound interval to its simple form.
    ///
    /// Simple intervals are in `[1, 8]`, and compound intervals (9ths, 10ths, etc.)
    /// are reduced by removing complete octaves. The direction is preserved.
    /// Octaves and multiples of octaves reduce to an octave, *not a unison!*
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// assert_eq!(Interval::MAJOR_TENTH.as_simple(), Interval::MAJOR_THIRD);
    /// assert_eq!(Interval::PERFECT_FIFTEENTH.as_simple(), Interval::PERFECT_OCTAVE);
    /// assert_eq!((-Interval::MAJOR_THIRD).as_simple(), -Interval::MAJOR_THIRD);
    /// ```
    pub fn as_simple(self) -> Self {
        Self {
            quality: self.quality,
            number: self.number.as_simple(),
        }
    }

    /// Returns the ascending form of the interval.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// assert_eq!(Interval::MAJOR_THIRD.abs(), Interval::MAJOR_THIRD);
    /// assert_eq!((-Interval::MAJOR_THIRD).abs(), Interval::MAJOR_THIRD);
    /// ```
    pub fn abs(self) -> Self {
        Self {
            number: self.number.abs(),
            .. self
        }
    }
    
    /// Negates the interval, but preserves perfect unisons.
    ///
    /// This ensures that perfect unisons remain positive, useful for musical operations where
    /// unisons should not become descending.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// // Perfect unison is preserved
    /// assert_eq!(
    ///     Interval::PERFECT_UNISON.neg_preserve_perfect_unison(),
    ///     Interval::PERFECT_UNISON
    /// );
    ///
    /// // Other intervals are negated normally
    /// assert_eq!(
    ///     Interval::MAJOR_THIRD.neg_preserve_perfect_unison(),
    ///     -Interval::MAJOR_THIRD
    /// );
    /// ```
    pub fn neg_preserve_perfect_unison(self) -> Self {
        if self.abs() == Self::PERFECT_UNISON {
            self
        } else {
            -self
        }
    }

    /// Creates an interval from a [number](Number) and [semitone](Semitones) count.
    ///
    /// Automatically determines the appropriate quality based on the
    /// number and semitones. This is useful when you know both the
    /// diatonic size and chromatic distance of an interval.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::interval::Number;
    /// # use music_theory::Semitones;
    /// // A third spanning 4 semitones is a major third
    /// assert_eq!(
    ///     Interval::from_number_and_semitones(Number::THIRD, Semitones(4)),
    ///     Interval::MAJOR_THIRD,
    /// );
    ///
    /// // A third spanning 5 semitones is an augmented third
    /// assert_eq!(
    ///     Interval::from_number_and_semitones(Number::THIRD, Semitones(5)),
    ///     Interval::AUGMENTED_THIRD,
    /// );
    /// ```
    pub fn from_number_and_semitones(num: Number, intended_semitones: Semitones) -> Self {
        let perfect = num.is_perfect();
        let num_sign = num.get().signum();

        let quality = match intended_semitones.0 - num.base_semitones_with_octave_unsigned() * num_sign {
            0 if perfect => Quality::Perfect,
            0 if !perfect => Quality::Major,
            -1 if !perfect && num_sign == 1 => Quality::Minor,
            -1 if !perfect && num_sign == -1 => Quality::AUGMENTED,
            diff => match diff * num_sign {
                -1 if !perfect => Quality::Minor,
                n if n > 0 => Quality::Augmented(NonZeroU16::new(n as u16).expect("zero was handled already")),
                n if n < 0 && perfect => Quality::Diminished(NonZeroU16::new(-n as u16).expect("nonzero")),
                n if n < 0 && !perfect => Quality::Diminished(NonZeroU16::new(-(n + 1) as _).expect("nonzero")),
                _ => unreachable!("all cases covered"),
            }
        };

        Self::new(quality, num).expect("valid quality")
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_number_and_semitones(
            self.number + rhs.number,
            self.semitones() + rhs.semitones(),
        )
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}

impl Mul<i16> for Interval {
    type Output = Self;

    /// Multiplies an interval by a scalar, equivalent to repeatedly adding the interval to itself.
    ///
    /// Multiplying by a negative value negates the result. Multiplying by zero
    /// returns a perfect unison in the same direction as the original interval.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// assert_eq!(
    ///     Interval::PERFECT_FOURTH * 3,
    ///     Interval::MINOR_TENTH,
    /// );
    ///
    /// let m6 = Interval::MINOR_SIXTH;
    /// assert_eq!(
    ///     m6 + m6 + m6 + m6 + m6,
    ///     m6 * 5, // == ddd26
    /// );
    ///
    /// assert_eq!(
    ///     Interval::MINOR_THIRD * -4,
    ///     -Interval::DIMINISHED_NINTH,
    /// );
    ///
    /// assert_eq!(Interval::MAJOR_SEVENTH * 0, Interval::PERFECT_UNISON);
    /// ```
    fn mul(self, rhs: i16) -> Self::Output {
        Self::from_number_and_semitones(
            self.number * rhs,
            self.semitones() * rhs,
        )
    }
}

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            number: -self.number,
            .. self
        }
    }
}

impl PartialOrd for Interval {
    /// Compares intervals by their number and quality (spelling-aware).
    ///
    /// See [`Ord::ord`](Ord) implementation for more information.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    /// Compares intervals by their number and quality (spelling-aware).
    ///
    /// Intervals are ordered by number first (descending < ascending), then by quality.
    /// For descending intervals, quality order is reversed, so a descending minor third is
    /// *greater* than a descending major third.
    ///
    /// # Examples
    /// ```
    /// # use music_theory::Interval;
    /// # use music_theory::EnharmonicEq as _;
    /// assert!(-Interval::MAJOR_SIXTH < -Interval::MAJOR_THIRD);
    /// // Descending intervals are less than ascending
    /// assert!(-Interval::MAJOR_THIRD < Interval::MAJOR_THIRD);
    ///
    /// // If ascending, smaller quality < larger quality
    /// assert!(Interval::MINOR_THIRD < Interval::MAJOR_THIRD);
    ///
    /// // If descending, it's the opposite
    /// assert!(-Interval::MINOR_THIRD > -Interval::MAJOR_THIRD);
    ///
    /// // Enharmonic intervals ordered by spelling
    /// assert!(Interval::AUGMENTED_FOURTH < Interval::DIMINISHED_FIFTH);
    /// assert!(Interval::AUGMENTED_FOURTH.eq_enharmonic(&Interval::DIMINISHED_FIFTH));
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.number
            .cmp(&other.number)
            .then_with(|| {
                let quality_cmp = self.quality.total_cmp(&other.quality);
                if self.number.is_ascending() {
                    quality_cmp
                } else {
                    quality_cmp.reverse()
                }
            })
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::PERFECT_UNISON
    }
}

impl Sum for Interval {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(Add::add).unwrap_or_default()
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shorthand())
    }
}

impl WithoutSpelling for Interval {
    type Unspelled = Semitones;

    fn without_spelling(self) -> Self::Unspelled {
        self.semitones()
    }
}

impl EnharmonicEq for Interval {
    fn eq_enharmonic(&self, other: &Self) -> bool {
        enharmonic::defer_without_spelling::eq(self, other)
    }
}

impl EnharmonicOrd for Interval {
    fn cmp_enharmonic(&self, other: &Self) -> Ordering {
        enharmonic::defer_without_spelling::cmp(self, other)
    }
}

/// Error returned when parsing an [`Interval`] from a [`&str`](prim@str) fails.
///
/// # Examples
/// ```
/// # use music_theory::Interval;
/// # use music_theory::interval::ParseIntervalError;
/// assert_eq!(
///     "P5".parse::<Interval>(),
///     Ok(Interval::PERFECT_FIFTH),
/// );
///
/// assert_eq!(
///     "XYZ".parse::<Interval>(),
///     Err(ParseIntervalError::InvalidFormat),
/// );
///
/// // Perfect third doesn't exist
/// assert_eq!(
///     "P3".parse::<Interval>(),
///     Err(ParseIntervalError::InvalidInterval),
/// );
/// ```
#[derive(Debug, thiserror::Error, Eq, PartialEq, Clone)]
pub enum ParseIntervalError {
    /// The input string was in an invalid format.
    #[error("The input was in an invalid format")]
    InvalidFormat,
    /// The quality-number combination is not a valid interval.
    #[error("The interval wasn't a valid interval")]
    InvalidInterval,
}

impl FromStr for Interval {
    type Err = ParseIntervalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseIntervalError as ParseErr;

        if s.chars()
            .last()
            .ok_or(ParseErr::InvalidFormat)?
            .is_ascii_digit()
        {
            let leading_negative = s.starts_with('-');

            let start = leading_negative as usize;

            let s = &s[start..];

            let split = s.find(|c: char| c.is_ascii_digit() || c == '-')
                .ok_or(ParseErr::InvalidFormat)?;

            let (quality_str, num_str) = s.split_at(split);

            let quality = quality_str.parse()
                .map_err(|_| ParseErr::InvalidFormat)?;

            let number = num_str.parse()
                .map_err(|_| ParseErr::InvalidFormat)?;

            let ivl = Self::new(quality, number).ok_or(ParseErr::InvalidInterval)?;

            if !ivl.is_ascending() && leading_negative {
                Err(ParseIntervalError::InvalidFormat)
            } else if leading_negative {
                Ok(-ivl)
            } else {
                Ok(ivl)
            }
        } else {
            let split = s.find(|c: char| !c.is_ascii_digit() && c != '-')
                .ok_or(ParseIntervalError::InvalidFormat)?;

            let (num_str, quality_str) = s.split_at(split);

            let quality = quality_str.parse()
                .map_err(|_| ParseErr::InvalidFormat)?;

            let number = num_str.parse()
                .map_err(|_| ParseErr::InvalidFormat)?;

            Ok(
                Self::new(quality, number).ok_or(ParseIntervalError::InvalidInterval)?
            )
        }
    }
}