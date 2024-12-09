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

    fn find_index<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: Fn(&Self::Item) -> bool;

    fn with_known_size(self) -> impl ExactSizeIterator<Item = Self::Item>;

    fn middle_element(self) -> Option<Self::Item>
    where
        Self: ExactSizeIterator;
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

    fn find_index<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: Fn(&Self::Item) -> bool,
    {
        Some(self.enumerate().find(|(_, e)| predicate(e))?.0)
    }

    fn middle_element(mut self) -> Option<Self::Item>
    where
        Self: ExactSizeIterator,
    {
        if self.len() % 2 == 0 {
            None
        } else {
            let s = self.len() / 2;
            self.nth(s)
        }
    }

    fn with_known_size(self) -> impl ExactSizeIterator<Item = Self::Item> {
        self.collect::<Vec<_>>().into_iter()
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
    use std::iter::repeat;

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

    #[test]
    fn test_find_index() {
        assert_eq!("abcd".chars().find_index(|c| c.is_uppercase()), None);
        assert_eq!("abCd".chars().find_index(|c| c.is_uppercase()), Some(2));
    }

    #[test]
    fn test_with_known_size() {
        assert_eq!("abcd".chars().with_known_size().len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_with_known_size_infinite() {
        repeat(()).with_known_size().for_each(|_| {});
    }

    #[test]
    fn test_middle_element() {
        assert_eq!("".chars().with_known_size().middle_element(), None);
        assert_eq!("ab".chars().with_known_size().middle_element(), None);
        assert_eq!("abc".chars().with_known_size().middle_element(), Some('b'));
    }
}
