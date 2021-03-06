use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub(crate) x: i16,
    pub(crate) y: i16,
}

impl Point {
    #[inline]
    pub fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }

    #[inline]
    pub fn step(&mut self, d: (i16, i16)) {
        self.x += d.0;
        self.y += d.1;
    }

    #[inline]
    pub fn packed(&self) -> i32 {
        ((self.x as i32) << 16) + self.y as i32
    }

    #[inline]
    pub fn quadrant(&self) -> u8 {
        match (self.x > 0, self.y >= 0) {
            (true, true) => 0,
            (false, true) => 1,
            (false, false) => 2,
            (true, false) => 3
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, h: &mut H) {
        h.write_i32(self.packed());
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
