use std::array;
use std::ops::Deref;
use crate::pcset::PitchClassSet;
use crate::pitch_class::PitchClass;
use crate::prelude::Semitone;

mod label;
mod consts;

pub use label::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TwelveToneMatrix {
    pub prime_0: TwelveToneRow,
}

impl TwelveToneMatrix {
    pub const fn new(prime_0: TwelveToneRow) -> Self {
        Self { prime_0 }
    }

    pub fn from_row(label: TwelveToneRowLabel, row: TwelveToneRow) -> Self {
        use TwelveToneRowLabel as Label;
        use TwelveToneRowForm as Form;
        use TwelveToneRowNumber as Num;

        match label {
            Label(Form::Prime, num) => {
                let m = Self::new(row);

                let undo = Num::new((12 - num.0) % 12)
                    .expect("must be valid");

                Self::new(m.get_row(Label(Form::Prime, undo)))
            },
            Label(Form::Retrograde, num) => {
                Self::from_row(Label(Form::Prime, num), row.reverse())
            },
            Label(Form::Inversion, num) => {
                let inv = Self::from_row(Label(Form::Prime, num), row);
                Self::from_row(Label::P0, inv.get_row(Label::I0))
            },
            Label(Form::RetrogradeInversion, num) => {
                Self::from_row(Label(Form::Inversion, num), row.reverse())
            },
        }
    }

    pub fn get_row(&self, label: TwelveToneRowLabel) -> TwelveToneRow {
        use TwelveToneRowForm as Form;

        match label.0 {
            Form::Prime => self.prime(label.1),
            Form::Retrograde => self.retrograde(label.1),
            Form::Inversion => self.inversion(label.1),
            Form::RetrogradeInversion => self.retrograde_inversion(label.1),
        }
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
        self.get_row(TwelveToneRowLabel::R0).intervals()
    }

    pub fn inversion_intervals(&self) -> [u8; 12] {
        self.get_row(TwelveToneRowLabel::I0).intervals()
    }

    pub fn retrograde_inversion_intervals(&self) -> [u8; 12] {
        self.get_row(TwelveToneRowLabel::RI0).intervals()
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

            for pc in self.prime(TwelveToneRowNumber::new(row_n).expect("must be in [0,11]")) {
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

    pub fn find_hexachord_complements(&self, hexachord: [PitchClass; 6]) -> Vec<(TwelveToneRowLabel, u8)> {
        let target = PitchClassSet::from_iter(hexachord);

        // all pitches need to be unique to find
        if target.len() != hexachord.len() as _ {
            return Vec::new();
        }

        let mut found = Vec::new();

        for label in TwelveToneRowLabel::iter() {
            for (i, chord) in self.get_row(label)
                .hexachords()
                .into_iter()
                .enumerate()
            {
                let chord_set = PitchClassSet::from_iter(chord);

                if (chord_set | target).len() == 12 {
                    found.push((label, i as _));
                }
            }
        }

        found
    }

    pub fn is_all_combinatorial(&self) -> bool {
        let hexachord = self.prime_0.hexachords()[0];

        let comb = self.find_hexachord_complements(hexachord);

        let prime = comb.iter().find(|(label, _)|
            label.0 == TwelveToneRowForm::Prime
        );

        let inversion = comb.iter().find(|(label, _)|
            label.0 == TwelveToneRowForm::Inversion
        );

        prime.is_some() && inversion.is_some()
    }

    pub fn label_hexachord(&self, hexachord: &[PitchClass; 6]) -> Option<(TwelveToneRowLabel, u8)> {
        for label in TwelveToneRowLabel::iter() {
            if let Some(pos) = self.get_row(label)
                .hexachords()
                .iter()
                .position(|c| *c == *hexachord)
            {
                assert!(
                    pos < 2,
                    "trivial: hexachords should only be [0,1]"
                );

                return Some((label, pos as _));
            }
        }

        None
    }

    fn prime(&self, n: TwelveToneRowNumber) -> TwelveToneRow {
        let prime_n = self.prime_0.map(|pc| pc + Semitone(*n as _));

        TwelveToneRow(prime_n)
    }

    fn retrograde(&self, n: TwelveToneRowNumber) -> TwelveToneRow {
        self.prime(n).reverse()
    }

    fn inversion(&self, n: TwelveToneRowNumber) -> TwelveToneRow {
        let prime = self.prime(n);

        let first = prime[0];

        let inversion = prime.map(|pc| {
            first - first.semitones_to(pc)
        });

        TwelveToneRow(inversion)
    }

    fn retrograde_inversion(&self, n: TwelveToneRowNumber) -> TwelveToneRow {
        self.inversion(n).reverse()
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


#[cfg(test)]
mod tests {
    use super::*;
    use TwelveToneRowLabel as Label;
    use TwelveToneRowForm as Form;
    use TwelveToneMatrix as Matrix;
    use PitchClass as PC;

    #[test]
    fn from_row() {
        let m = Matrix {
            prime_0: TwelveToneRow::new([PC::C, PC::Cs, PC::E, PC::D, PC::A, PC::F, PC::B, PC::Ds, PC::Gs, PC::As, PC::G, PC::Fs])
                .expect("has all intervals"),
        };

        for form in [Form::Prime, Form::Retrograde, Form::Inversion, Form::RetrogradeInversion] {
            for n in 0..12 {
                let label = Label::new(form, n).expect("valid number");

                let row = m.get_row(label);

                assert_eq!(
                    m, Matrix::from_row(label, row),
                    "Matrix::from_row({label:?}, ...) created wrong matrix",
                )
            }
        }
    }

    #[test]
    fn all_combinatorial() {
        let asc = TwelveToneMatrix {
            prime_0: TwelveToneRow::from_chromas(array::from_fn(|i| i as _))
                .expect("valid row")
        };

        assert!(
            asc.is_all_combinatorial(),
            "ascending row matrix should be all combinatorial",
        );
    }
}