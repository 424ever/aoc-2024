pub struct Vec2D<T> {
    cols: usize,
    data: Vec<T>,
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

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn lines(&self) -> usize {
        self.data.len() / self.cols()
    }

    fn index(&self, l: usize, c: usize) -> usize {
        l * self.cols() + c
    }
}
