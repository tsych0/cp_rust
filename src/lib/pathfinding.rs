use crate::indexmap::IndexMap;
use std::convert::{TryFrom, TryInto};
use std::hash::Hash;

pub fn reverse_path<N, V, F>(parents: &IndexMap<N, V>, mut parent: F, start: usize) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: FnMut(&V) -> usize,
{
    let mut i = start;
    let path = std::iter::from_fn(|| {
        parents.get_index(i).map(|(node, value)| {
            i = parent(value);
            node
        })
    })
    .collect::<Vec<&N>>();
    // Collecting the going through the vector is needed to revert the path because the
    // unfold iterator is not double-ended due to its iterative nature.
    path.into_iter().rev().cloned().collect()
}

/// Return the square root of `n` if `n` is square, `None` otherwise.
pub fn uint_sqrt<T>(n: T) -> Option<T>
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::cmp::PartialOrd
        + From<u8>
        + TryInto<f64>
        + TryFrom<u64>,
    <T as TryInto<f64>>::Error: std::fmt::Debug,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
{
    // Convert to f64 for sqrt calculation
    let n_f64: f64 = n.try_into().ok()?;
    let root_f64 = n_f64.sqrt();

    // Convert back to T
    let root: T = T::try_from(root_f64 as u64).ok()?;

    // Check if it's a perfect square
    if root * root == n {
        Some(root)
    } else {
        None
    }
}

/// Move a two-dimensional coordinate into a given direction provided that:
#[must_use]
pub fn move_in_direction(
    start: (usize, usize),
    direction: (isize, isize),
    dimensions: (usize, usize),
) -> Option<(usize, usize)> {
    let (row, col) = start;
    if row >= dimensions.0 || col >= dimensions.1 || direction == (0, 0) {
        return None;
    }
    let (new_row, new_col) = (row as isize + direction.0, col as isize + direction.1);
    (new_row >= 0
        && (new_row as usize) < dimensions.0
        && new_col >= 0
        && (new_col as usize) < dimensions.1)
        .then_some((new_row as usize, new_col as usize))
}

/// Repeatedly call [`move_in_direction`] until the returned value is `None`.
pub fn in_direction(
    start: (usize, usize),
    direction: (isize, isize),
    dimensions: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    std::iter::successors(Some(start), move |current| {
        move_in_direction(*current, direction, dimensions)
    })
    .skip(1)
}

/// Constrain `value` into `0..upper` by adding or subtracting `upper`
///  as many times as necessary.
#[must_use]
pub const fn constrain(value: isize, upper: usize) -> usize {
    if value > 0 {
        value as usize % upper
    } else {
        (upper - (-value) as usize % upper) % upper
    }
}
