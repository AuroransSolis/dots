use std::fmt::{self, Display};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn step(&mut self, d: (i32, i32)) {
        self.x += d.0;
        self.y += d.1;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}