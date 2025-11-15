use std::fmt;
use crate::pcset::PitchClassSet;
use crate::pitch_class::PitchClass;
use crate::prelude::Semitone;

#[derive(Clone, Eq, PartialEq)]
pub struct TwelveToneMatrix([PitchClass; 12]);

impl TwelveToneMatrix {
    pub fn new(prime_0: [PitchClass; 12]) -> Option<Self> {
        let pc_set = PitchClassSet::from_iter(prime_0);

        (pc_set.len() == 12).then_some(Self(prime_0))
    }

    pub fn from_chromas(prime_0: [u8; 12]) -> Result<Self, TwelveToneMatrixFromNumsError> {
        if prime_0.iter().any(|&n| n >= 12) {
            return Err(TwelveToneMatrixFromNumsError::InvalidNums);
        }

        let prime_0 = prime_0.map(PitchClass::from_repr).map(Option::unwrap);

        Self::new(prime_0).ok_or(TwelveToneMatrixFromNumsError::MissingPitches)
    }

    pub fn prime(&self, n: u8) -> Option<[PitchClass; 12]> {
        if n >= 12 {
            return None;
        }

        Some(self.0.map(|pc| pc + Semitone(n as _)))
    }

    pub fn retrograde(&self, n: u8) -> Option<[PitchClass; 12]> {
        self.prime(n).map(|mut p| {
            p.reverse();
            p
        })
    }

    pub fn inversion(&self, n: u8) -> Option<[PitchClass; 12]> {
        let prime = self.prime(n)?;

        let first = prime[0];

        let inversion = prime.map(|pc| {
           first - first.semitones_to(pc)
        });

        Some(inversion)
    }

    pub fn retrograde_inversion(&self, n: u8) -> Option<[PitchClass; 12]> {
        self.inversion(n).map(|mut p| {
            p.reverse();
            p
        })
    }

    pub fn order_primes(&self) -> [u8; 12] {
        self.order_inversions().map(|n| (12 - n) % 12)
    }

    pub fn order_inversions(&self) -> [u8; 12] {
        let first = self.0[0];

        self.0.map(|pc| first.semitones_to(pc).0 as _)
    }

    pub fn table_string(&self) -> String {
        use std::fmt::Write;

        let row_space = "      ";
        let mut s = row_space.to_owned();

        for inv_n in self.order_inversions() {
            write!(s, " I-{inv_n:<2} ").expect("String::write_fmt shouldn't fail");
        }

        for row_n in self.order_primes() {
            write!(s, "\nP-{row_n:<2} |").expect("String::write_fmt shouldn't fail");

            for pc in self.prime(row_n).expect("must be in range") {
                // this is due to how formatting width works
                let pc_str = pc.to_string();

                write!(s, " {pc_str:<5}").expect("String::write_fmt shouldn't fail");
            }

            write!(s, " | R-{row_n:<2}").expect("String::write_fmt shouldn't fail");
        }

        write!(s, "\n{row_space}").expect("String::write_fmt shouldn't fail");

        for inv_n in self.order_inversions() {
            write!(s, " RI-{inv_n:<2}").expect("String::write_fmt shouldn't fail");
        }

        s
    }
}

impl fmt::Debug for TwelveToneMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(TwelveToneMatrix))
            .field("P-0", &self.0)
            .finish()
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum TwelveToneMatrixFromNumsError {
    #[error("One or more numbers wasn't a valid pitch class chroma [0,11]")]
    InvalidNums,
    #[error("Didn't have all 12 chromas [0,11]")]
    MissingPitches,
}