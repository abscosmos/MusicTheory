use crate::chord::{Chord, InvalidInversion};
use crate::interval::Interval;
use crate::key::Key;
use crate::pitch::Pitch;
use crate::scales::heptatonic::DiatonicMode;
use strum_macros::FromRepr;

// not typed at all!
pub mod inversions {
    pub const INV_ROOT: u8 = 0;
    pub const INV_6: u8 = 1;
    pub const INV_64: u8 = 2;
    pub const INV_65: u8 = 1;
    pub const INV_43: u8 = 2;
    pub const INV_42: u8 = 3;
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromRepr)]
pub enum ScaleDegree {
    I = 1,
    II = 2,
    III = 3,
    IV = 4,
    V = 5,
    VI = 6,
    VII = 7,
}

impl ScaleDegree {
    pub fn as_idx(self) -> u8 {
        (self as u8) - 1
    }

    pub fn from_idx(idx: u8) -> Option<Self> {
        Self::from_repr(idx + 1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
}


#[derive(Debug, thiserror::Error, Clone, Eq, PartialEq)]
#[error("Invalid inversion for chord type")]
pub struct InvalidInversionError;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RomanChord {
    pub degree: ScaleDegree,
    pub triad_quality: Quality,
    seventh_quality: Option<Quality>,
    inversion: u8,
}

impl RomanChord {
    pub fn new(
        degree: ScaleDegree,
        triad_quality: Quality,
        seventh_quality: Option<Quality>,
        inversion: u8,
    ) -> Result<Self, InvalidInversionError> {
        let max_inversion = if seventh_quality.is_some() { 3 } else { 2 };

        if inversion > max_inversion {
            return Err(InvalidInversionError);
        }

        Ok(Self {
            degree,
            triad_quality,
            seventh_quality,
            inversion,
        })
    }

    pub fn triad(degree: ScaleDegree, triad_quality: Quality) -> Self {
        Self::new(degree, triad_quality, None, inversions::INV_ROOT)
            .expect("root position inversion always valid")
    }

    pub fn seventh(degree: ScaleDegree, triad_quality: Quality, seventh_quality: Quality) -> Self {
        Self::new(degree, triad_quality, Some(seventh_quality), inversions::INV_ROOT)
            .expect("root position inversion always valid")
    }

    pub fn seventh_quality(&self) -> Option<Quality> {
        self.seventh_quality
    }

    pub fn inversion(&self) -> u8 {
        self.inversion
    }

    pub fn with_inversion(self, inversion: u8) -> Result<Self, InvalidInversionError> {
        Self::new(
            self.degree,
            self.triad_quality,
            self.seventh_quality,
            inversion,
        )
    }

    pub fn intervals(&self) -> Vec<Interval> {
        use Interval as I;
        use Quality as Q;

        let mut intervals = vec![I::PERFECT_UNISON];

        let triad = match self.triad_quality {
            Q::Major => [I::MAJOR_THIRD, I::PERFECT_FIFTH],
            Q::Minor => [I::MINOR_THIRD, I::PERFECT_FIFTH],
            Q::Diminished => [I::MINOR_THIRD, I::DIMINISHED_FIFTH],
            Q::Augmented => [I::MAJOR_THIRD, I::AUGMENTED_FIFTH],
        };

        let seventh = self.seventh_quality.map(|q| match q {
            Quality::Major => I::MAJOR_SEVENTH,
            Quality::Minor => I::MINOR_SEVENTH,
            Quality::Diminished => I::DIMINISHED_SEVENTH,
            Quality::Augmented => I::AUGMENTED_SEVENTH,
        });

        intervals.extend(triad);
        intervals.extend(seventh);

        intervals
    }

    // source of truth for alterations
    // TODO: move this somewhere else?
    fn mode_has_raised_leading_tone(mode: DiatonicMode) -> bool {
        matches!(mode, DiatonicMode::Aeolian | DiatonicMode::Dorian)
    }
}
