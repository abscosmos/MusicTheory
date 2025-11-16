use crate::matrix::{TwelveToneRowForm, TwelveToneRowLabel, TwelveToneRowNumber};

const P: TwelveToneRowForm = TwelveToneRowForm::Prime;
const R: TwelveToneRowForm = TwelveToneRowForm::Retrograde;
const I: TwelveToneRowForm = TwelveToneRowForm::Inversion;
const RI: TwelveToneRowForm = TwelveToneRowForm::RetrogradeInversion;

macro_rules! define_consts {
    ($($letter:ident $num:literal),* $(,)?) => {
        $(
            paste::paste! {
                pub const [<$letter $num>]: Self = Self::new($letter,$num).expect("valid number");
            }
        )*
    };
    ($($letter:ident),+) => {
        $(
            define_consts! {
                $letter 0, $letter 1, $letter 2, $letter 3, $letter 4, $letter 5,
                $letter 6, $letter 7, $letter 8, $letter 9, $letter 10, $letter 11,
            }
        )+
    };
}


impl TwelveToneRowLabel {
    define_consts! { P, R, I, RI }
}