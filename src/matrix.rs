use std::array;
use std::ops::Deref;
use crate::pcset::PitchClassSet;
use crate::pitch_class::PitchClass;
use crate::prelude::Semitone;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TwelveToneMatrix {
    pub prime_0: TwelveToneRow,
}

impl TwelveToneMatrix {
    pub fn prime(&self, n: u8) -> Option<TwelveToneRow> {
        if n >= 12 {
            return None;
        }

        let prime_n = self.prime_0.map(|pc| pc + Semitone(n as _));

        Some(TwelveToneRow(prime_n))
    }

    pub fn retrograde(&self, n: u8) -> Option<TwelveToneRow> {
        self.prime(n).map(|r| r.reverse())
    }

    pub fn inversion(&self, n: u8) -> Option<TwelveToneRow> {
        let prime = self.prime(n)?;

        let first = prime[0];

        let inversion = prime.map(|pc| {
           first - first.semitones_to(pc)
        });

        Some(TwelveToneRow(inversion))
    }

    pub fn retrograde_inversion(&self, n: u8) -> Option<TwelveToneRow> {
        self.inversion(n).map(|r| r.reverse())
    }

    pub fn order_primes(&self) -> [u8; 12] {
        self.order_inversions().map(|n| (12 - n) % 12)
    }

    pub fn order_inversions(&self) -> [u8; 12] {
        let first = self.prime_0[0];

        self.prime_0.map(|pc| first.semitones_to(pc).0 as _)
    }

    pub fn prime_intervals(&self) -> [u8; 12] {
        self.prime_0.intervals()
    }

    pub fn retrograde_intervals(&self) -> [u8; 12] {
        self.retrograde(0)
            .expect("r0 must be in range")
            .intervals()
    }

    pub fn inversion_intervals(&self) -> [u8; 12] {
        self.inversion(0)
            .expect("i0 must be in range")
            .intervals()
    }

    pub fn retrograde_inversion_intervals(&self) -> [u8; 12] {
        self.retrograde_inversion(0)
            .expect("ri0 must be in range")
            .intervals()
    }

    pub fn has_all_intervals(&self) -> bool {
        self.prime_0.has_all_intervals()
    }

    pub fn rotate(&self, offset: u8) -> Option<Self> {
        Some(Self { prime_0: self.prime_0.rotate(offset)? })
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TwelveToneRow(pub(crate) [PitchClass; 12]);

impl TwelveToneRow {
    pub fn new(prime_0: [PitchClass; 12]) -> Option<Self> {
        let pc_set = PitchClassSet::from_iter(prime_0);

        (pc_set.len() == 12).then_some(Self(prime_0))
    }

    pub fn from_chromas(prime_0: [u8; 12]) -> Result<Self, TwelveToneRowFromChromasError> {
        if prime_0.iter().any(|&n| n >= 12) {
            return Err(TwelveToneRowFromChromasError::InvalidNums);
        }

        let prime_0 = prime_0.map(|repr|
            PitchClass::from_repr(repr).expect("just checked, so must be in range")
        );

        Self::new(prime_0).ok_or(TwelveToneRowFromChromasError::MissingPitches)
    }

    pub fn reverse(&self) -> Self {
        let mut new = self.clone();
        new.0.reverse();
        new
    }

    pub fn get(&self) -> &[PitchClass; 12] {
        &self.0
    }

    pub fn intervals(&self) -> [u8; 12] {
        array::from_fn(|i|
            self.0[i].semitones_to(self.0[(i+1) % 12]).0 as _
        )
    }

    pub fn has_all_intervals(&self) -> bool {
        let ivls = self.intervals();

        (1..12).all(|i| ivls.contains(&i))
    }

    pub fn rotate(&self, offset: u8) -> Option<Self> {
        if offset > 12 {
            None
        } else {
            let mut new = self.clone();
            new.0.rotate_left(offset as usize % 12);
            Some(new)
        }
    }

    pub fn trichords(&self) -> [[PitchClass; 3]; 4] {
        self.divide()
    }

    pub fn tetrachords(&self) -> [[PitchClass; 4]; 3] {
        self.divide()
    }

    pub fn hexachords(&self) -> [[PitchClass; 6]; 2] {
        self.divide()
    }

    fn divide<const N: usize, const C: usize>(&self) -> [[PitchClass; N]; C] {
        // unfortunately, this assertion can't be done at compile time
        // (waiting on feature generic_const_exprs)
        // we'll have to rely on a test to trigger this assertion
        assert_eq!(N * C, 12, "Must evenly and correctly divide the row");

        array::from_fn(|i|
            array::from_fn(|j| self.0[i * N + j])
        )
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum TwelveToneRowFromChromasError {
    #[error("One or more numbers wasn't a valid pitch class chroma [0,11]")]
    InvalidNums,
    #[error("Didn't have all 12 chromas [0,11]")]
    MissingPitches,
}

impl IntoIterator for TwelveToneRow {
    type Item = PitchClass;
    type IntoIter = <[PitchClass; 12] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for TwelveToneRow {
    type Target = [PitchClass; 12];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}