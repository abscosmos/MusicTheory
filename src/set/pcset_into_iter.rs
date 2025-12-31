use std::iter::FusedIterator;
use strum::IntoEnumIterator;
use crate::pitch::PitchClass;
use crate::set::PitchClassSet;

/// An iterator over the pitch classes in a [`PitchClassSet`].
///
/// Iterates in ascending order from C to B.
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
