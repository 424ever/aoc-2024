use std::ops::{Add, Mul, Sub};

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
pub struct CoordDiff2D {
    dx: i32,
    dy: i32,
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

    pub fn is_valid(&self, coord: &Coord2D) -> bool {
        coord.x < self.width && coord.y < self.height
    }
}

impl Coord2D {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn into_bounded(self, bounds: Bounds2D) -> BoundedCoord2D {
        BoundedCoord2D::new(self, bounds)
    }

    pub fn go_in(&self, dir: &Direction2D) -> Option<Coord2D> {
        Some(match dir {
            Direction2D::Up => Self {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
            Direction2D::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction2D::Left => Self {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction2D::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        })
    }
}

impl Add<CoordDiff2D> for Coord2D {
    type Output = Option<Self>;

    fn add(self, rhs: CoordDiff2D) -> Self::Output {
        Some(Self::new(
            self.x.checked_add_signed(rhs.dx)?,
            self.y.checked_add_signed(rhs.dy)?,
        ))
    }
}

impl Sub for Coord2D {
    type Output = CoordDiff2D;

    fn sub(self, rhs: Self) -> Self::Output {
        CoordDiff2D {
            dx: i32::try_from(self.x).unwrap() - i32::try_from(rhs.x).unwrap(),
            dy: i32::try_from(self.y).unwrap() - i32::try_from(rhs.y).unwrap(),
        }
    }
}

impl Mul<i32> for CoordDiff2D {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
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
        self.coord
    }

    pub fn bounds(&self) -> Bounds2D {
        self.bounds
    }

    pub fn go_in(&self, dir: &Direction2D) -> Option<BoundedCoord2D> {
        let new = self.coord.go_in(dir)?;
        Self::if_valid(self.bounds(), new)
    }

    fn if_valid(bounds: Bounds2D, coord: Coord2D) -> Option<BoundedCoord2D> {
        if bounds.is_valid(&coord) {
            Some(Self::new(coord, bounds))
        } else {
            None
        }
    }
}

impl Add<CoordDiff2D> for BoundedCoord2D {
    type Output = Option<Self>;

    fn add(self, rhs: CoordDiff2D) -> Self::Output {
        Self::if_valid(self.bounds(), (self.coord + rhs)?)
    }
}

impl Sub for BoundedCoord2D {
    type Output = CoordDiff2D;

    fn sub(self, rhs: Self) -> Self::Output {
        self.coord - rhs.coord
    }
}
