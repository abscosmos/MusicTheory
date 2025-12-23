#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Dynamics {
    PPP = 0,
    PP = 1,
    P = 2,
    MP = 3,
    MF = 4,
    F = 5,
    FF = 6,
    FFF = 7,
}