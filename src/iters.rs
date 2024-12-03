use itertools::Itertools;
use std::ops::Sub;

pub trait Differences: Iterator {
    fn differences(self) -> impl Iterator<Item = Self::Item>;
}

impl<T> Differences for T
where
    T: Iterator,
    T::Item: Sub<Output = T::Item>,
    T::Item: Clone,
{
    fn differences(self) -> impl Iterator<Item = Self::Item> {
        self.tuple_windows()
            .map(|t: (Self::Item, Self::Item)| t.1.sub(t.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::iters::Differences;

    #[test]
    fn test_diffs() {
        assert_eq!(
            vec![0, 2, 1, 3]
                .into_iter()
                .differences()
                .collect::<Vec<_>>(),
            vec![2, -1, 2]
        );
    }
}
