use std::array;
use std::ops::Add;
use crate::interval::Interval;

fn build_inner<T: Add<Interval, Output = T> + Clone>(i: usize, curr: &mut T, ivls: &[Interval], mode: u8) -> T {
    let ret = curr.clone();

    *curr = curr.clone() + ivls[(i + mode as usize - 1) % ivls.len()];

    ret
}

pub fn build_from<T: Add<Interval, Output = T> + Clone, const N: usize>(rel_ivls: [Interval; N], root: T, mode: u8) -> [T; N] {
    assert!(1 <= mode && mode as usize <= N, "mode should be in range");

    let mut curr = root;

    array::from_fn(|i| build_inner(i, &mut curr, &rel_ivls, mode))
}

pub fn boxed_build_from<T: Add<Interval, Output = T> + Clone>(rel_ivls: &[Interval], root: T, mode: u8) -> Box<[T]> {
    assert!(1 <= mode && mode as usize <= rel_ivls.len(), "mode should be in range");

    let mut curr = root;

    (0..rel_ivls.len())
        .map(|i| build_inner(i, &mut curr, rel_ivls, mode))
        .collect()
}