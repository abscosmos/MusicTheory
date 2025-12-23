use std::ops::Index;
use serde::{Deserialize, Serialize};
use typed_floats::tf32::{self, StrictlyPositiveFinite};
use crate::pitch_class::PitchClass;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JustIntonation {
    pub a4_hz: StrictlyPositiveFinite,
    pub ratios: JustIntonationRatios,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct JustIntonationRatios([StrictlyPositiveFinite; 12]);

impl JustIntonationRatios {
    pub fn new(ratios: [StrictlyPositiveFinite; 12]) -> Result<Self, JustIntonationRatiosError> {
        if ratios[0] != 1.0 {
            return Err(JustIntonationRatiosError::UnisonNotIdentity);
        }

        for window in ratios.windows(2) {
            let &[a, b] = window else {
                unreachable!("window size is two");
            };

            if a <= b {
                return Err(JustIntonationRatiosError::NotStrictlyIncreasing);
            }
        }

        if ratios.windows(2)
            .any(|window| {
                let &[a, b] = window else {
                    unreachable!("window size is two");
                };

                a <= b
            })
        {
            return Err(JustIntonationRatiosError::NotStrictlyIncreasing);
        }

        Ok(Self(ratios))
    }

    pub const fn as_array(self) -> [StrictlyPositiveFinite; 12] {
        self.0
    }

    /// This function should only be used to define constants!
    const fn expect_valid(ratios: [f32; 12]) -> Self {
        let mut res = [tf32::MAX; 12];

        let mut i = 0;
        while i < 12 {
            res[i] = match StrictlyPositiveFinite::new(ratios[i]) {
                Ok(ratio) => ratio,
                Err(_) => panic!("all ratios must be strictly positive and finite"),
            };

            i += 1;
        }

        Self(res)
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum JustIntonationRatiosError {
    #[error("Ratio between unisons must be 1/1")]
    UnisonNotIdentity,
    #[error("Ratios must be strictly increasing order")]
    NotStrictlyIncreasing,
}

impl Index<PitchClass> for JustIntonationRatios {
    type Output = StrictlyPositiveFinite;

    fn index(&self, index: PitchClass) -> &Self::Output {
        self.0.get(index.chroma() as usize).expect("PitchClass::chroma is in [0,12)")
    }
}