use itertools::Itertools;

pub struct Vec2D<T> {
    cols: usize,
    data: Vec<T>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2DIndex {
    line: usize,
    column: usize,
}

impl<T> Vec2D<T> {
    pub fn from_lines<I>(lines: I) -> Self
    where
        I: Iterator,
        I::Item: IntoIterator<Item = T>,
    {
        let mut cols = None;
        let mut data = vec![];

        for line in lines {
            let line = line.into_iter().collect::<Vec<_>>();
            if cols.is_some() && cols.unwrap() != line.len() {
                panic!("lines with different lengths");
            } else {
                cols = Some(line.len());
            }
            data.extend(line);
        }

        Self {
            cols: cols.expect("no input lines"),
            data,
        }
    }

    pub fn get(&self, line: usize, column: usize) -> Option<&T> {
        if line >= self.lines() || column >= self.cols() {
            None
        } else {
            self.data.get(self.index(line, column))
        }
    }

    pub fn get_index(&self, index: &Vec2DIndex) -> Option<&T> {
        self.get(index.line, index.column)
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn lines(&self) -> usize {
        self.data.len() / self.cols()
    }

    pub fn enumerated_iter(&self) -> impl Iterator<Item = (Vec2DIndex, &T)> {
        self.indizes().map(|i| (i, self.get_index(&i).unwrap()))
    }

    pub fn indizes(&self) -> impl Iterator<Item = Vec2DIndex> {
        (0..self.lines())
            .cartesian_product(0..self.cols())
            .map(|(l, c)| Vec2DIndex::new(l, c))
    }

    fn index(&self, l: usize, c: usize) -> usize {
        l * self.cols() + c
    }
}

impl Vec2DIndex {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn checked_add_signed(&self, lines: isize, columns: isize) -> Option<Self> {
        Some(Self::new(
            self.line.checked_add_signed(lines)?,
            self.column.checked_add_signed(columns)?,
        ))
    }
}
