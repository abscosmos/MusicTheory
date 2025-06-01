use std::ops::RangeInclusive;

pub struct WrappingRange<T: Ord>(RangeInclusive<T>);

impl<T: Ord> WrappingRange<T> {
    pub fn new(domain: RangeInclusive<T>) -> Self {
        match domain.into_inner() {
            (start, end) if start <= end => Self(start..=end),
            (start, end) => Self(end..=start),
        }
    }

    pub fn contains(&self, range: RangeInclusive<T>, val: &T) -> bool {
        let start = range.start();
        let end = range.end();

        assert!(self.0.contains(val));
        assert!(self.0.contains(start));
        assert!(self.0.contains(end));

        if start <= end {
            range.contains(val)
        } else {
            val >= start || val <= end
        }
    }
}