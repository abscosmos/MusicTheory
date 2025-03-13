use std::ops::Deref;

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

impl<T: PartialEq> PartialEq for Placed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.octave == other.octave
    }
}

impl<T: Eq> Eq for Placed<T> {}

impl<T: Clone> Clone for Placed<T> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            octave: self.octave,
        }
    }
}

impl<T: Copy> Copy for Placed<T> { }

impl<T> Deref for Placed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}