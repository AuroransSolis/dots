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

    // (3, 0) =>
    pub fn rot_90_acw(&self) -> Self {
        let &Point { x, y } = self;
        Point::new(-y, x - 1)
    }

    // (-2, 3) => (4, 2)
    pub fn rot_90_cw(&self) -> Self {
        let &Point { x, y } = self;
        Point::new(y + 1, -x)
    }

    pub fn reflect_across_y(&self) -> Point {
        let &Point { x, y } = self;
        Point::new(-x + 1, y)
    }

    pub fn reflect_across_x(&self) -> Point {
        let &Point { x, y } = self;
        Point::new(x, -y + 1)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}