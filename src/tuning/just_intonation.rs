use std::cmp::Ordering;
use std::ops::Index;
use const_soft_float::soft_f32::SoftF32;
use serde::{Deserialize, Serialize};
use typed_floats::tf32::{self, StrictlyPositiveFinite};
use crate::pitch_class::PitchClass;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JustIntonation {
    pub a4_hz: StrictlyPositiveFinite,
    pub ratios: JustIntonationRatios,
}

impl JustIntonation {
    pub const HZ_440_LIMIT_5: Self = Self::new(440.0, JustIntonationRatios::LIMIT_5).expect("440 is in (0, inf)");

    pub const fn new(a4_hz: f32, ratios: JustIntonationRatios) -> Option<Self> {
        match StrictlyPositiveFinite::new(a4_hz) {
            Ok(a4_hz) => Some(Self { a4_hz, ratios }),
            Err(_) => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct JustIntonationRatios([StrictlyPositiveFinite; 12]);

impl JustIntonationRatios {
    pub const LIMIT_5: Self = {
        let Ok(ratios) = Self::with_ratios(
            16.0/15.0,
            9.0/8.0,
            6.0/5.0,
            5.0/4.0,
            4.0/3.0,
            45.0/32.0,
            3.0/2.0,
            8.0/5.0,
            5.0/3.0,
            9.0/5.0,
            15.0/8.0,
        ) else {
            panic!("unreachable!: should be valid ratios");
        };

        ratios
    };

    // this calls Self::with_ratios internally for a single source of truth
    pub const fn new(ratios: [StrictlyPositiveFinite; 12]) -> Result<Self, JustIntonationRatiosError> {
        let [
            unison,
            minor_second,
            major_second,
            minor_third,
            major_third,
            perfect_fourth,
            tritone,
            perfect_fifth,
            minor_sixth,
            major_sixth,
            minor_seventh,
            major_seventh,
        ] = ratios;

        // if unison != 1.0 (complicated because of const)
        if !matches!(SoftF32(unison.get()).cmp(SoftF32(1.0)), Some(Ordering::Equal)) {
            return Err(JustIntonationRatiosError::UnisonNotIdentity);
        }

        Self::with_ratios(
            minor_second.get(),
            major_second.get(),
            minor_third.get(),
            major_third.get(),
            perfect_fourth.get(),
            tritone.get(),
            perfect_fifth.get(),
            minor_sixth.get(),
            major_sixth.get(),
            minor_seventh.get(),
            major_seventh.get(),
        )
    }

    pub const fn with_ratios(
        minor_second: f32,
        major_second: f32,
        minor_third: f32,
        major_third: f32,
        perfect_fourth: f32,
        tritone: f32,
        perfect_fifth: f32,
        minor_sixth: f32,
        major_sixth: f32,
        minor_seventh: f32,
        major_seventh: f32,
    ) -> Result<Self, JustIntonationRatiosError> {
        let ratios = [
            1.0,
            minor_second,
            major_second,
            minor_third,
            major_third,
            perfect_fourth,
            tritone,
            perfect_fifth,
            minor_sixth,
            major_sixth,
            minor_seventh,
            major_seventh,
        ];

        let mut res = [tf32::MAX; 12];

        // check strictly ascending
        let mut i = 0;
        while i < ratios.len() {
            if i + 1 != ratios.len() &&
                matches!(
                    SoftF32(ratios[i]).cmp(SoftF32(ratios[i + 1])),
                    Some(Ordering::Greater | Ordering::Equal),
                )
            {
                return Err(JustIntonationRatiosError::NotStrictlyIncreasing);
            }

            // this ensures ratio is in (1.0, 2.0), complicated because of const
            res[i] = match StrictlyPositiveFinite::new(ratios[i]) {
                Ok(checked) => checked,
                _ => return Err(JustIntonationRatiosError::InvalidRatio),
            };

            i += 1;
        }

        // instead of checking if all are in [1.0, 2.0), since already checked strictly increasing
        // and first is 1.0, only need to check last!
        match SoftF32(*ratios.last().expect("should have 12 elems")).cmp(SoftF32(2.0)) {
            Some(Ordering::Less | Ordering::Equal) => Ok(Self(res)),
            Some(Ordering::Greater) => Err(JustIntonationRatiosError::InvalidRatio),
            None => panic!("unreachable!: uncomparable values already handled"),
        }
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum JustIntonationRatiosError {
    #[error("Ratio between unisons must be 1/1")]
    UnisonNotIdentity,
    #[error("Ratios must be strictly increasing order")]
    NotStrictlyIncreasing,
    #[error("The ratios were not in range [1.0, 2.0)")]
    InvalidRatio,
}

impl Index<PitchClass> for JustIntonationRatios {
    type Output = StrictlyPositiveFinite;

    fn index(&self, index: PitchClass) -> &Self::Output {
        self.0.get(index.chroma() as usize).expect("PitchClass::chroma is in [0,12)")
    }
}