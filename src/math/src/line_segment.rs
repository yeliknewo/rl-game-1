use utils::{Coord};

use ::{Point2};

#[derive(Debug, Clone)]
pub struct LineSeg {
    a: Point2,
    b: Point2,
}

impl LineSeg {
    pub fn new_from_coords(x0: Coord, y0: Coord, x1: Coord, y1: Coord) -> LineSeg {
        LineSeg::new(Point2::new(x0, y0), Point2::new(x1, y1))
    }

    pub fn new(a: Point2, b: Point2) -> LineSeg {
        LineSeg {
            a: a,
            b: b,
        }
    }

    pub fn get_a(&self) -> Point2 {
        self.a.clone()
    }

    pub fn get_b(&self) -> Point2 {
        self.b.clone()
    }
}
