use std::ops::Sub;

pub struct Pairs<I, T> {
    iter: I,
    last: Option<T>,
}

pub trait IteratorExtensions: Iterator {
    fn pairs(self) -> Pairs<Self, Self::Item>
    where
        Self: Sized;
    fn differences(self) -> impl Iterator<Item = <Self::Item as Sub>::Output>
    where
        Self::Item: Sub;
}

impl<T> IteratorExtensions for T
where
    T: Iterator,
    T::Item: Clone,
{
    fn pairs(self) -> Pairs<Self, Self::Item>
    where
        Self: Sized,
    {
        Pairs::new(self)
    }

    fn differences(self) -> impl Iterator<Item = <Self::Item as Sub>::Output>
    where
        Self::Item: Sub + Clone,
    {
        self.pairs().map(|t: (Self::Item, Self::Item)| t.1.sub(t.0))
    }
}

impl<I, T> Pairs<I, T> {
    pub fn new(iter: I) -> Self {
        Self { iter, last: None }
    }
}

impl<I, T> Iterator for Pairs<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.last.is_none() {
            self.last = Some(self.iter.next()?);
        }

        let curr = self.iter.next()?;
        let it = (
            self.last.clone().expect("last element is None"),
            curr.clone(),
        );

        self.last = Some(curr);

        Some(it)
    }
}

#[cfg(test)]
mod tests {
    use crate::iters::IteratorExtensions;

    #[test]
    fn test_pairs() {
        assert_eq!(
            "abcd"
                .chars()
                .pairs()
                .map(|(c1, c2)| String::from_iter([c1, c2]))
                .collect::<Vec<_>>(),
            vec!["ab", "bc", "cd"]
        );
    }

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
