pub trait EnharmonicEq {
    fn eq_enharmonic(&self, rhs: &Self) -> bool;

    fn ne_enharmonic(&self, rhs: &Self) -> bool {
        !self.eq_enharmonic(rhs)
    }
}