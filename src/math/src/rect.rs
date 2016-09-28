
use utils::{Coord};
use ::{LineSeg, Point2};

#[derive(Debug, Clone)]
pub struct Rect {
    corners: LineSeg,
}

impl Rect {
    pub fn new_from_coords(x0: Coord, y0: Coord, x1: Coord, y1: Coord) -> Rect {
        Rect::new_from_points(Point2::new(x0, y0), Point2::new(x1, y1))
    }

    pub fn new_from_points(a: Point2, b: Point2) -> Rect {
        Rect::new(LineSeg::new(a, b))
    }

    pub fn new(corners: LineSeg) -> Rect {
        assert!(corners.get_a().get_x() < corners.get_b().get_x());
        assert!(corners.get_a().get_y() < corners.get_b().get_y());
        Rect {
            corners: corners,
        }
    }

    pub fn get_corners(&self) -> LineSeg {
        self.corners.clone()
    }

    pub fn get_bot_left(&self) -> Point2 {
        self.corners.get_a()
    }

    pub fn get_top_right(&self) -> Point2 {
        self.corners.get_b()
    }

    pub fn check_collide_point(&self, point: Point2) -> bool {
        trace!("Rect Points: ({},{}) ({},{})", self.get_bot_left().get_x(), self.get_bot_left().get_y(), self.get_top_right().get_x(), self.get_top_right().get_y());
        self.get_bot_left().get_x() <= point.get_x() &&
        self.get_bot_left().get_y() <= point.get_y() &&
        self.get_top_right().get_x() >= point.get_x() &&
        self.get_top_right().get_y() >= point.get_y()
    }
}
