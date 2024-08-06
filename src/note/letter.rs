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
