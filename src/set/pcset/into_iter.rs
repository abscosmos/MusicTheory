use std::iter::FusedIterator;
use strum::IntoEnumIterator;
use crate::pitch::PitchClass;
use crate::set::pcset::PitchClassSet;

// TODO: unsure if there should be a separate wrapper type returned when calling into_iter?
//     since the into_iter type is just a wrapper, the iterator impls could be directly on PitchClassSet
//     it might be confusing to be able to call .last(), .next(), on a collection type

/// An iterator over the pitch classes in a [`PitchClassSet`].
///
/// Iterates in ascending order from C to B.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PitchClassSetIntoIter(PitchClassSet);

impl Iterator for PitchClassSetIntoIter {
    type Item = PitchClass;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: it's possible to do this with trailing_zeros and leading_zeros
        let next = PitchClass::iter().find(|pc| self.0.is_set(*pc));

        if let Some(pc) = next {
            self.0 = self.0.with_cleared(pc);
        }

        next
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len() as _;
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.0.len() as _
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl DoubleEndedIterator for PitchClassSetIntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO: it's possible to do this with trailing_zeros and leading_zeros
        let next = PitchClass::iter().rfind(|pc| self.0.is_set(*pc));

        if let Some(pc) = next {
            self.0 = self.0.with_cleared(pc);
        }

        next
    }
}

impl ExactSizeIterator for PitchClassSetIntoIter {
    #[inline]
    fn len(&self) -> usize {
        self.0.len() as _
    }
}

impl FusedIterator for PitchClassSetIntoIter {}

impl IntoIterator for PitchClassSet {
    type Item = PitchClass;
    type IntoIter = PitchClassSetIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        PitchClassSetIntoIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_forward() {
        let set = PitchClassSet::from_iter([
            PitchClass::C,
            PitchClass::E,
            PitchClass::G,
        ]);

        assert_eq!(
            set.into_iter().collect::<Vec<_>>(),
            [PitchClass::C, PitchClass::E, PitchClass::G]
        );
    }

    #[test]
    fn iter_backward() {
        let set = PitchClassSet::from_iter([
            PitchClass::C,
            PitchClass::E,
            PitchClass::G,
        ]);

        assert_eq!(
            set.into_iter().rev().collect::<Vec<_>>(),
            vec![PitchClass::G, PitchClass::E, PitchClass::C]
        );
    }

    #[test]
    fn iter_empty() {
        let set = PitchClassSet::default();

        assert_eq!(set.into_iter().count(), 0);
        assert_eq!(set.into_iter().next(), None);
        assert_eq!(set.into_iter().next_back(), None);
    }

    #[test]
    fn iter_full() {
        assert_eq!(
            PitchClassSet::CHROMATIC_AGGREGATE.into_iter().collect::<Vec<_>>(),
            PitchClass::iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn iter_size_hint() {
        let set = PitchClassSet::from_iter([
            PitchClass::C,
            PitchClass::E,
            PitchClass::G,
        ]);

        let mut iter = set.into_iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.len(), 3);

        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn iter_nth() {
        let set = PitchClassSet::from_iter([
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
            PitchClass::G,
        ]);

        let mut iter = set.into_iter();
        assert_eq!(iter.nth(2), Some(PitchClass::E));
        assert_eq!(iter.next(), Some(PitchClass::F));
    }

    #[test]
    fn iter_nth_back() {
        let set = PitchClassSet::from_iter([
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
            PitchClass::G,
        ]);

        let mut iter = set.into_iter();
        assert_eq!(iter.nth_back(2), Some(PitchClass::E));
        assert_eq!(iter.next_back(), Some(PitchClass::D));
    }

    #[test]
    fn iter_last() {
        let set = PitchClassSet::from_iter([
            PitchClass::C,
            PitchClass::E,
            PitchClass::G,
        ]);

        assert_eq!(set.into_iter().last(), Some(PitchClass::G));
    }

    #[test]
    fn iter_double_ended() {
        let set = PitchClassSet::from_iter([
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
        ]);

        let mut iter = set.into_iter();
        assert_eq!(iter.next(), Some(PitchClass::C));
        assert_eq!(iter.next_back(), Some(PitchClass::F));
        assert_eq!(iter.next(), Some(PitchClass::D));
        assert_eq!(iter.next_back(), Some(PitchClass::E));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
