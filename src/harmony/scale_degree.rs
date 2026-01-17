use crate::scales::numeral::Numeral7 as ScaleDegreeExp;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Ord, PartialOrd, strum_macros::FromRepr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScaleDegree {
    #[default]
    I = 1,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}

impl ScaleDegree {
    pub fn as_num(self) -> u8 {
        self as _
    }

    pub fn from_num(num: u8) -> Option<Self> {
        Self::from_repr(num)
    }

    pub(crate) fn as_experimental(self) -> ScaleDegreeExp {
        ScaleDegreeExp::from_repr(self as _).expect("implementation should be exact copy")
    }

    pub(crate) fn from_experimental(inner: ScaleDegreeExp) -> Self {
        Self::from_num(inner as _).expect("implementation should be exact copy")
    }
}