use std::cmp::Ordering;

pub trait EnharmonicEq {
    fn eq_enharmonic(&self, rhs: &Self) -> bool;

    fn ne_enharmonic(&self, rhs: &Self) -> bool {
        !self.eq_enharmonic(rhs)
    }
}

pub trait EnharmonicOrd {
    fn cmp_enharmonic(&self, rhs: &Self) -> Ordering;
}