use std::array;
use std::ops::Add;
use crate::interval::Interval;

fn build_inner<T: Add<Interval, Output = T> + Clone>(ivl: Interval, curr: &mut T) -> T {
    let ret = curr.clone();

    *curr = curr.clone() + ivl;

    ret
}

pub fn build_from<T: Add<Interval, Output = T> + Clone, const N: usize>(rel_ivls: [Interval; N], root: T) -> [T; N] {
    let mut curr = root;

    array::from_fn(|i| build_inner(rel_ivls[i], &mut curr))
}

pub fn boxed_build_from<T: Add<Interval, Output = T> + Clone>(rel_ivls: &[Interval], root: T) -> Box<[T]> {
    let mut curr = root;

    rel_ivls.iter()
        .map(|ivl| build_inner(*ivl, &mut curr))
        .collect()
}