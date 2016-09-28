use std::ops::{Add};

use utils::{Coord, CoordI};

use ::{Point2};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Point2I {
    x: CoordI,
    y: CoordI,
}

impl Point2I {
    pub fn new(x: CoordI, y: CoordI) -> Point2I {
        Point2I {
            x: x,
            y: y,
        }
    }

    pub fn get_x(&self) -> CoordI {
        self.x
    }

    pub fn get_y(&self) -> CoordI {
        self.y
    }

    pub fn get_mut_x(&mut self) -> &mut CoordI {
        &mut self.x
    }

    pub fn get_mut_y(&mut self) -> &mut CoordI {
        &mut self.y
    }

    pub fn add_ref(&self, other: &Point2I) -> Point2I {
        Point2I::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }

    pub fn sub_ref(&self, other: &Point2I) -> Point2I {
        Point2I::new(self.get_x() - other.get_x(), self.get_y() - other.get_y())
    }

    pub fn length(&self) -> Coord {
        ((self.get_x().pow(2) + self.get_y().pow(2)) as Coord).sqrt()
    }
}

impl Add<Point2I> for Point2I {
    type Output = Point2I;

    fn add(self, other: Point2I) -> Point2I {
        self.add_ref(&other)
    }
}

impl From<Point2> for Point2I {
    fn from(other: Point2) -> Self {
        Point2I::new(other.get_x().round() as CoordI, other.get_y().round() as CoordI)
    }
}
