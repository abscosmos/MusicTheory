use crate::pitch::Pitch;

macro_rules! define_pitches {
    ($($name:ident = $value:expr),* $(,)?) => {
        $(pub const $name: Self = Self($value);)*

        #[cfg(test)]
        #[allow(unused, reason = "This constant is intended to only be used for tests")]
        pub(crate) const ALL_CONSTS: &'static [Self] = &[
            $(Self::$name),*
        ];
    };
}

#[doc(hidden)]
impl Pitch {
    define_pitches! {
        F_DOUBLE_FLAT = -15,
        C_DOUBLE_FLAT = -14,
        G_DOUBLE_FLAT = -13,
        D_DOUBLE_FLAT = -12,
        A_DOUBLE_FLAT = -11,
        E_DOUBLE_FLAT = -10,
        B_DOUBLE_FLAT = -9,

        F_FLAT = -8,
        C_FLAT = -7,
        G_FLAT = -6,
        D_FLAT = -5,
        A_FLAT = -4,
        E_FLAT = -3,
        B_FLAT = -2,

        F = -1,
        C = 0,
        G = 1,
        D = 2,
        A = 3,
        E = 4,
        B = 5,

        F_SHARP = 6,
        C_SHARP = 7,
        G_SHARP = 8,
        D_SHARP = 9,
        A_SHARP = 10,
        E_SHARP = 11,
        B_SHARP = 12,

        F_DOUBLE_SHARP = 13,
        C_DOUBLE_SHARP = 14,
        G_DOUBLE_SHARP = 15,
        D_DOUBLE_SHARP = 16,
        A_DOUBLE_SHARP = 17,
        E_DOUBLE_SHARP = 18,
        B_DOUBLE_SHARP = 19,
    }
}