use crate::scales::numeral::Numeral7 as ScaleDegreeExp;

/// Enum of scale degrees in a heptatonic scale.
///
/// Copy of [implementation in experimental scales module][exp].
/// Intended to be used until a stable version of scales is released.
///
/// [exp]: crate::scales::numeral::Numeral7
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
    /// Convert `self` to a `u8`, starting from `1`.
    ///
    /// # Examples
    /// ```rust
    /// # use music_theory::harmony::ScaleDegree;
    /// assert_eq!(ScaleDegree::III.as_num(), 3);
    /// ```
    pub fn as_num(self) -> u8 {
        self as _
    }

    /// Convert from `u8` to `self`, *starting from `1`*.
    ///
    /// Returns `None` if not in [1, 7].
    ///
    /// # Examples
    /// ```rust
    /// # use music_theory::harmony::ScaleDegree;
    /// assert_eq!(ScaleDegree::from_num(3), Some(ScaleDegree::III));
    /// ```
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