use crate::set::Direction;
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

    pub fn step(&self, direction: Direction) -> Self {
        let Point { x, y } = self;
        let (dx, dy) = direction.single_step();
        Point::new(x + dx, y + dy)
    }

    pub fn step_opposite(&self, direction: Direction) -> Self {
        let Point { x, y } = self;
        let (dx, dy) = direction.opposite_single_step();
        Point::new(x + dx, y + dy)
    }

    pub fn rot_90_acw(&self) -> Self {
        let &Point { x, y } = self;
        Point::new(-y, x - 1)
    }

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