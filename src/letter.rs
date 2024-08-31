use std::fmt;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
