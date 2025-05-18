use music_theory::prelude as mt;
use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::PyValueError;

#[pyclass]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pitch(mt::Pitch);

#[pymethods]
impl Pitch {
    #[new]
    fn new(letter: Letter, accidental: AccidentalSign) -> PyResult<Self> {
        Ok(Self(mt::Pitch::from_letter_and_accidental(letter.to_mt(), accidental.0)))
    }
    
    #[getter(letter)]
    fn get_letter(&self) -> Letter {
        Letter::from_mt(self.0.letter())
    }
    
    #[setter(letter)]
    fn set_letter(&mut self, letter: Letter) {
        *self = Self(mt::Pitch::from_letter_and_accidental(letter.to_mt(), self.0.accidental()));
    }
    
    fn transpose(&self, interval: Interval) -> Self {
        Self(self.0 + interval.0)
    }
}

#[pyclass]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Letter {
    C = 0,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[pymethods]
impl Letter {
    #[new]
    fn new(letter: &str) -> PyResult<Self> {
        let letter = letter.parse().map_err(|_| PyValueError::new_err("Invalid letter"))?;
        
        Ok(Letter::from_mt(letter))
    }
}

impl Letter {
    fn to_mt(&self) -> mt::Letter {
        mt::Letter::from_step(*self as u8).expect("should have exact same repr as mt::Letter")
    }
    
    fn from_mt(mt: mt::Letter) -> Self {
        match mt {
            mt::Letter::C => Letter::C,
            mt::Letter::D => Letter::D,
            mt::Letter::E => Letter::E,
            mt::Letter::F => Letter::F,
            mt::Letter::G => Letter::G,
            mt::Letter::A => Letter::A,
            mt::Letter::B => Letter::B,
        }
    }
}

#[pyclass]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AccidentalSign(mt::AccidentalSign);

#[pymethods]
impl AccidentalSign {
    #[classattr]
    const DOUBLE_FLAT: Self = Self(mt::AccidentalSign::DOUBLE_FLAT);

    #[classattr]
    const FLAT: Self = Self(mt::AccidentalSign::FLAT);

    #[classattr]
    const NATURAL: Self = Self(mt::AccidentalSign::NATURAL);

    #[classattr]
    const SHARP: Self = Self(mt::AccidentalSign::SHARP);

    #[classattr]
    const DOUBLE_SHARP: Self = Self(mt::AccidentalSign::DOUBLE_SHARP);
    
    
    #[new]
    fn new(offset: i16) -> Self {
        Self(mt::AccidentalSign { offset })
    }
}

#[pyclass]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interval(mt::Interval);

#[pymethods]
impl Interval {
    #[new]
    fn new(ivl: &str) -> PyResult<Self> {
        let ivl = ivl.parse().map_err(|_| PyValueError::new_err("Invalid interval"))?;
        
        Ok(Self(ivl))
    }
}
