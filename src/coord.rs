#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction2D {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bounds2D {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord2D {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundedCoord2D {
    coord: Coord2D,
    bounds: Bounds2D,
}

impl Direction2D {
    pub fn turn_right(&self) -> Direction2D {
        match self {
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
        }
    }
}

impl Bounds2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn positions(&self) -> impl Iterator<Item = Coord2D> + use<'_> {
        (0..self.height()).flat_map(|y| (0..self.width()).map(move |x| Coord2D::new(x, y)))
    }

    pub fn expand_width(&mut self, width: u32) {
        self.width = self.width.max(width);
    }

    pub fn expand_height(&mut self, height: u32) {
        self.height = self.height.max(height);
    }
}

impl Coord2D {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn into_bounded(self, bounds: Bounds2D) -> BoundedCoord2D {
        BoundedCoord2D::new(self, bounds)
    }

    pub fn go_in(&self, dir: &Direction2D) -> Coord2D {
        match dir {
            Direction2D::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction2D::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction2D::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction2D::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl BoundedCoord2D {
    pub fn new(coord: Coord2D, bounds: Bounds2D) -> Self {
        if coord.x >= bounds.width || coord.y >= bounds.height {
            panic!("coordinates out-of-bounds");
        }
        Self { coord, bounds }
    }

    pub fn unbounded(&self) -> Coord2D {
        self.coord.clone()
    }

    pub fn bounds(&self) -> Bounds2D {
        self.bounds
    }

    pub fn can_go_in(&self, dir: &Direction2D) -> bool {
        match dir {
            Direction2D::Up => self.coord.y > 0,
            Direction2D::Down => self.coord.y < self.bounds.height - 1,
            Direction2D::Left => self.coord.x > 0,
            Direction2D::Right => self.coord.x < self.bounds.width - 1,
        }
    }

    pub fn go_in(&self, dir: &Direction2D) -> Option<BoundedCoord2D> {
        if !self.can_go_in(dir) {
            None
        } else {
            Some(Self {
                coord: self.coord.go_in(dir),
                bounds: self.bounds,
            })
        }
    }
}
