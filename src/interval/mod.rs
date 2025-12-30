use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::num::{NonZeroI16, NonZeroU16, ParseIntError};
use std::ops::{Add, Neg, Sub};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::enharmonic::{EnharmonicEq, EnharmonicOrd};
use crate::note::Note;
use crate::pitch::Pitch;
use crate::semitone::Semitone;

mod quality;
pub use quality::*;

mod number;
pub use number::*;

mod stability;
pub use stability::*;

mod consts;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Interval {
    quality: IntervalQuality,
    number: IntervalNumber,
}

impl Interval {
    pub fn new(quality: IntervalQuality, number: IntervalNumber) -> Option<Self> {
        use IntervalQuality as Q;

        let unchecked = Some(Self { quality, number });

        match quality {
            Q::Diminished(_) | Q::Augmented(_) => unchecked,
            Q::Perfect if number.is_perfect() => unchecked,
            Q::Minor | Q::Major if !number.is_perfect() => unchecked,
            _ => None,
        }
    }
    
    pub fn new_maj_or_perfect(number: IntervalNumber) -> Self {
        let quality = if number.is_perfect() {
            IntervalQuality::Perfect
        } else {
            IntervalQuality::Major
        };
        
        Self { quality, number }
    }

    pub fn strict_non_subzero(quality: IntervalQuality, number: IntervalNumber) -> Option<Self> {
         Self::new(quality, number).filter(|ivl| !ivl.is_subzero())
    }

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

        Interval::new(base_interval.quality(), IntervalNumber(signed_number))
            .expect("quality should still be valid")
    }
    
    // TODO: test with intervals where quality makes it more than 2 octaves
    pub fn between_pitches(lhs: Pitch, rhs: Pitch) -> Self {
        let lhs_letter = lhs.letter();
        let rhs_letter = rhs.letter();

        let number = lhs_letter.offset_between(rhs_letter) + 1;
        
        let number = IntervalNumber::new(number as _)
            .expect("can't be zero since offset_between returns [0, 6], and adding one");

        let number = if number == IntervalNumber::UNISON && lhs > rhs { IntervalNumber::OCTAVE } else { number };

        let base_semitones = lhs.semitones_to(rhs).0;
        
        let semitones = if lhs_letter.offset_between(rhs_letter) == 6 && base_semitones == 0 {
            base_semitones + 12
        } else {
            base_semitones
        };
        
        let quality = match semitones - number.base_semitones_with_octave_unsigned() {
            -1 if number.is_perfect() => IntervalQuality::DIMINISHED,
            -1 => IntervalQuality::Minor,
            0 if number.is_perfect() => IntervalQuality::Perfect,
            0 => IntervalQuality::Major,
            n @ 1.. => IntervalQuality::Augmented((n as u16).try_into().expect("can't be zero")),
            n @ ..-1 if number.is_perfect() => IntervalQuality::Diminished(NonZeroU16::new(-n as u16).expect("shouldn't be zero, as the first arm should've caught that")),
            n @ ..-1 => IntervalQuality::Diminished(NonZeroU16::new(-n as u16 - 1).expect("shouldn't be zero, as the first arm should've caught that")),
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
    /// # use music_theory::prelude::*;
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
    pub fn stability(&self) -> Option<Stability> {
        use IntervalQuality as Q;
        use IntervalNumber as N;

        match self.quality {
            Q::Diminished(_) | Q::Augmented(_) => Some(Stability::Dissonance),
            _ => match self.number.as_simple() {
                N::UNISON | N::FIFTH | N::OCTAVE => Some(Stability::PerfectConsonance),
                N::THIRD | N::SIXTH => Some(Stability::ImperfectConsonance),
                N::SECOND | N::SEVENTH => Some(Stability::Dissonance),
                N::FOURTH => None,
                _ => unreachable!("as_simple should return number in [1,8]"),
            }
        }
    }

    // TODO: does this work for descending intervals?
    pub fn is_subzero(&self) -> bool {
        let semitones = self.semitones().0;

        semitones != 0 && semitones.signum() != self.number.get().signum()
    }
    
    // TODO: add tests for this function
    // TODO: ensure this works for descending intervals
    pub fn expand_subzero(&self) -> Self {
        if !self.is_subzero() {
            return *self;
        }

        const OCTAVE_SEMITONES: i16 = 12;

        let semitones = self.semitones().0;

        let octaves = -semitones.div_euclid(OCTAVE_SEMITONES);
        
        let new_number = IntervalNumber::new(self.number().get() + octaves * 7)
            .expect("shouldn't be zero to begin with");
        
        let expanded = Self::strict_non_subzero(self.quality, new_number)
            .expect("should be valid quality and not subzero");
        
        debug_assert!(expanded.semitones().0 < OCTAVE_SEMITONES, "expanded shouldn't be more than an octave");
        
        expanded
    }

    pub fn quality(&self) -> IntervalQuality {
        self.quality
    }

    pub fn number(&self) -> IntervalNumber {
        self.number
    }

    pub fn semitones(&self) -> Semitone {
        let base_oct_semitones = self.number.base_semitones_with_octave_unsigned();

        use IntervalQuality as Q;

        let quality_adjust = match self.quality {
            Q::Perfect | Q::Major => 0,
            Q::Minor => -1,

            Q::Diminished(n) => if self.number.is_perfect() {
                -(n.get() as i16)
            } else {
                -(n.get() as i16 + 1)
            }

            Q::Augmented(n) => n.get() as i16,
        };

        let unsigned = base_oct_semitones + quality_adjust;

        Semitone(unsigned * self.number.get().signum())
    }

    pub fn shorthand(&self) -> String {
        format!("{}{}", self.quality.shorthand(), self.number.shorthand())
    }

    pub fn inverted(&self) -> Self {
        Self::new(self.quality.inverted(), self.number.inverted())
            .expect("valid quality")
    }

    pub fn inverted_strict_non_subzero(&self) -> Option<Self> {
        match self.inverted() {
            ivl if !ivl.is_subzero() => Some(ivl),
            _ => None,
        }
    }

    pub fn from_semitones_preferred(semitones: Semitone) -> Self {
        let semi = semitones.0;

        if semi == 0 {
            return Self::PERFECT_UNISON;
        }

        let semi_adj = (semi.abs() - 1) % 12;

        use IntervalQuality as Q;

        let (quality, base_num) = match semi_adj + 1 {
            1 => (Q::Minor, 2),
            2 => (Q::Major, 2),
            3 => (Q::Minor, 3),
            4 => (Q::Major, 3),
            5 => (Q::Perfect, 4),
            6 => (Q::DIMINISHED, 5),
            7 => (Q::Perfect, 5),
            8 => (Q::Minor, 6),
            9 => (Q::Major, 6),
            10 => (Q::Minor, 7),
            11 => (Q::Major, 7),
            12 => (Q::Perfect, 8),
            _ => unreachable!("should be in range [1,12]"),
        };

        let oct = (semi.abs() - 1) / 12;

        let unsigned = base_num + oct * 7;

        let number = IntervalNumber::new(unsigned * semi.signum())
            .expect("non zero");

        Self::new(quality, number)
            .expect("valid quality")
    }
    
    pub fn is_ascending(&self) -> bool {
        self.number.is_ascending()
    }
    
    pub fn with_direction(&self, ascending: bool) -> Self {
        Self {
            number: self.number.with_direction(ascending),
            .. *self
        }
    }
    
    pub fn as_simple(&self) -> Self {
        Self {
            quality: self.quality,
            number: self.number.as_simple(),
        }
    }
    
    // TODO: better name? and tests
    pub fn swap_direction_invert(&self) -> Self {
        -self.inverted()
    }

    pub fn abs(&self) -> Self {
        self.with_direction(true)
    }

    fn add_interval(self, rhs: Self) -> Self {
        let ln = self.number.get();
        let rn = rhs.number.get();

        let offset = {
            let ls = ln.signum();
            let rs = rn.signum();
            let ss = (ln + rn).signum();

            -ls * rs * ss + (ss == 0) as i16
        };

        let num = IntervalNumber::new(ln + rn + offset)
            .expect("nonzero");

        let distance = self.semitones().0 + rhs.semitones().0;

        let num_sign = num.get().signum();

        let difference = distance - num.base_semitones_with_octave_unsigned() * num_sign;

        let perfect = num.is_perfect();

        use IntervalQuality as IQ;

        let quality = match difference {
            0 if perfect => IQ::Perfect,
            0 if !perfect => IQ::Major,
            -1 if !perfect && num_sign == 1 => IQ::Minor,
            -1 if !perfect && num_sign == -1 => IQ::AUGMENTED,
            diff => match diff * num_sign {
                -1 if !perfect => IQ::Minor,
                n if n > 0 => IQ::Augmented(NonZeroU16::new(n as u16).expect("zero was handled already")),
                n if n < 0 && perfect => IQ::Diminished(NonZeroU16::new(-n as u16).expect("nonzero")),
                n if n < 0 && !perfect => IQ::Diminished(NonZeroU16::new(-(n + 1) as _).expect("nonzero")),
                _ => unreachable!("all cases covered"),
            }
        };

        Self::new(quality, num).expect("valid quality")
    }
    
    pub fn neg_preserve_perfect_unison(&self) -> Self {
        if self.abs() == Self::PERFECT_UNISON {
            *self
        } else {
            -*self
        }
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_interval(rhs)
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
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

impl EnharmonicEq for Interval {
    fn eq_enharmonic(&self, rhs: &Self) -> bool {
        self.semitones() == rhs.semitones()
    }
}

impl EnharmonicOrd for Interval {
    fn cmp_enharmonic(&self, rhs: &Self) -> Ordering {
        self.semitones().0.cmp(&rhs.semitones().0)
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq, Clone)]
pub enum ParseIntervalError {
    #[error("The input was in an invalid format")]
    InvalidFormat,
    #[error("The interval wasn't a valid interval")]
    InvalidInterval,
    #[error(transparent)]
    QualityErr(#[from] ParseIntervalQualityErr),
    #[error("Failed to parse number: {0}")]
    NumberErr(#[from] ParseIntError),
}

impl FromStr for Interval {
    type Err = ParseIntervalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars()
            .last()
            .ok_or(ParseIntervalError::InvalidFormat)?
            .is_ascii_digit()
        {
            let leading_negative = s.starts_with('-');

            let start = leading_negative as usize;

            let s = &s[start..];

            let split = s.find(|c: char| c.is_ascii_digit() || c == '-')
                .ok_or(ParseIntervalError::InvalidFormat)?;

            let (quality_str, num_str) = s.split_at(split);

            let quality = quality_str.parse()?;

            let number = num_str.parse()?;

            let ivl = Self::new(quality, number).ok_or(ParseIntervalError::InvalidInterval)?;

            if leading_negative {
                Ok(-ivl)
            } else {
                Ok(ivl)
            }
        } else {
            let split = s.find(|c: char| !c.is_ascii_digit() && c != '-')
                .ok_or(ParseIntervalError::InvalidFormat)?;

            let (num_str, quality_str) = s.split_at(split);

            Ok(
                Self::new(quality_str.parse()?, num_str.parse()?)
                    .ok_or(ParseIntervalError::InvalidInterval)?
            )
        }
    }
}