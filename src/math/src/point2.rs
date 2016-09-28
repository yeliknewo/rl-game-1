use utils::{Coord};

use ::{Point2I};

use std::ops::{Add, Sub, Mul, Div, SubAssign, MulAssign};

#[derive(Debug, PartialEq, Clone)]
pub struct Point2 {
    x: Coord,
    y: Coord,
}

impl Point2 {
    pub fn new(x: Coord, y: Coord) -> Point2 {
        Point2 {
            x: x,
            y: y,
        }
    }

    pub fn zero() -> Point2 {
        Point2::new(0.0, 0.0)
    }

    pub fn get_x(&self) -> Coord {
        self.x
    }

    pub fn get_y(&self) -> Coord {
        self.y
    }

    pub fn get_mut_x(&mut self) -> &mut Coord {
        &mut self.x
    }

    pub fn get_mut_y(&mut self) -> &mut Coord {
        &mut self.y
    }

    pub fn normalized(&self) -> Point2 {
        self.clone() / self.length()
    }

    pub fn length(&self) -> Coord {
        (self.get_x().powi(2) + self.get_y().powi(2)).sqrt()
    }

    pub fn is_zero(&self) -> bool {
        self.get_x() == 0.0 && self.get_y() == 0.0
    }

    pub fn is_normal(&self) -> bool {
        self.get_x().is_normal() && self.get_y().is_normal()
    }

    pub fn is_finite(&self) -> bool {
        self.get_x().is_finite() && self.get_y().is_finite()
    }

    pub fn abs(&self) -> Point2 {
        Point2::new(self.get_x().abs(), self.get_y().abs())
    }

    pub fn distance_to_point2i(&self) -> Coord {
        let rounded: Point2I = self.clone().into();
        let unrounded: Point2 = rounded.into();
        (self.clone() - unrounded).length()
    }

    pub fn rounded(&self) -> Point2 {
        Point2::new(self.get_x().round(), self.get_y().round())
    }

    pub fn interpolate(&self, other: &Point2, percent: Coord) -> Point2 {
        (other.sub_ref(self) * percent).add_ref(self)
    }

    pub fn add_ref(&self, other: &Point2) -> Point2 {
        Point2::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }

    pub fn sub_ref(&self, other: &Point2) -> Point2 {
        Point2::new(self.get_x() - other.get_x(), self.get_y() - other.get_y())
    }
}

impl Add<Point2> for Point2 {
    type Output = Point2;

    fn add(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() + other.get_x(), self.get_y() + other.get_y())
    }
}

impl Sub<Point2> for Point2 {
    type Output = Point2;

    fn sub(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() - other.get_x(), self.get_y() - other.get_y())
    }
}

impl Mul<Point2> for Point2 {
    type Output = Point2;

    fn mul(self, other: Point2) -> Point2 {
        Point2::new(self.get_x() * other.get_x(), self.get_y() * other.get_y())
    }
}

impl Mul<Coord> for Point2 {
    type Output = Point2;

    fn mul(self, other: Coord) -> Point2 {
        Point2::new(self.get_x() * other, self.get_y() * other)
    }
}

impl Div<Coord> for Point2 {
    type Output = Point2;

    fn div(self, other: Coord) -> Point2 {
        Point2::new(self.get_x() / other, self.get_y() / other)
    }
}

impl SubAssign<Point2> for Point2 {
    fn sub_assign(&mut self, other: Point2) {
        self.x -= other.get_x();
        self.y -= other.get_y();
    }
}

impl MulAssign<Point2> for Point2 {
    fn mul_assign(&mut self, other: Point2) {
        self.x *= other.get_x();
        self.y *= other.get_y();
    }
}

impl MulAssign<Coord> for Point2 {
    fn mul_assign(&mut self, other: Coord) {
        self.x *= other;
        self.y *= other;
    }
}

impl From<Point2I> for Point2 {
    fn from(other: Point2I) -> Self {
        Point2::new(other.get_x() as Coord, other.get_y() as Coord)
    }
}
