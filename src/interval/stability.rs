use serde::{Serialize, Deserialize};

/// The harmonic stability level of an interval.
///
/// Represents the degree of consonance or dissonance of an interval in traditional
/// music theory. Consonant intervals sound stable and at rest, while dissonant
/// intervals create tension that typically resolves to consonances.
///
/// # Variants
///
/// - [`PerfectConsonance`](Self::PerfectConsonance): The most stable intervals - perfect unisons, fifths, and octaves
/// - [`ImperfectConsonance`](Self::ImperfectConsonance): Consonant but less pure - major and minor thirds and sixths
/// - [`Dissonance`](Self::Dissonance): Unstable intervals that create tension - seconds, sevenths, and augmented/diminished intervals
///
/// # Examples
///
/// ```
/// # use music_theory::prelude::*;
/// // Check if an interval is consonant
/// let stability = Interval::MAJOR_THIRD.stability().unwrap();
/// assert!(stability.is_consonant());
///
/// // Perfect consonances are the most stable
/// let perfect = Interval::PERFECT_FIFTH.stability().unwrap();
/// assert_eq!(perfect, Stability::PerfectConsonance);
///
/// // Dissonances create harmonic tension
/// let dissonance = Interval::MAJOR_SEVENTH.stability().unwrap();
/// assert_eq!(dissonance, Stability::Dissonance);
/// assert!(!dissonance.is_consonant());
///
/// // Perfect fourths are consonant melodically, but dissonant harmonically
/// let ambiguous = Interval::PERFECT_FOURTH.stability();
/// assert_eq!(ambiguous, None);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Stability {
    PerfectConsonance,
    ImperfectConsonance,
    Dissonance,
}

impl Stability {
    pub fn is_consonant(self) -> bool {
        matches!(self, Self::PerfectConsonance | Self::ImperfectConsonance)
    }
}