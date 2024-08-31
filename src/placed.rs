pub struct Placed<T> {
    pub base: T,
    pub octave: i16,
}

impl<T> Placed<T> {
    pub fn to_base(self) -> T {
        self.base
    }

    pub fn as_base(&self) -> &T {
        &self.base
    }
}