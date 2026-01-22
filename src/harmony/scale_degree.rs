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
    /// Scale degree I: The tonic.
    #[default]
    I = 1,
    /// Scale degree II: The supertonic.
    II,
    /// Scale degree III: The mediant.
    III,
    /// Scale degree IV: The subdominant.
    IV,
    /// Scale degree V: The dominant.
    V,
    /// Scale degree VI: The submediant.
    VI,
    /// Scale degree VII: The leading tone (or subtonic in natural minor).
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

#[cfg(test)]
mod tests {
    use super::ScaleDegree as Deg;

    #[test]
    fn basic_properties() {
        assert_eq!(Deg::IV, Deg::IV, "scale degree eq failed");
        assert!(Deg::II < Deg::V, "scale degree lt failed");
        assert!(Deg::VI > Deg::I, "scale degree gt failed");

        assert_eq!(Deg::from_num(5), Some(Deg::V), "scale degree from_num in range failed");
        assert_eq!(Deg::from_num(0), None, "scale degree from_num invalid failed");
        assert_eq!(Deg::from_num(8), None, "scale degree from_num invalid failed");

        assert_eq!(Deg::default(), Deg::I, "scale degree default failed");

        for d in 1..=7 {
            let from_num = Deg::from_num(d).expect("valid degree");

            assert_eq!(from_num.as_num(), d, "scale degree round trip to u8 failed");
        }
    }

    #[test]
    fn experimental_round_trip() {
        for d in 1..=7 {
            let deg = Deg::from_num(d).expect("valid degree");

            assert_eq!(
                Deg::from_experimental(deg.as_experimental()), deg,
                "scale degree round trip to experimental failed"
            )
        }
    }
}