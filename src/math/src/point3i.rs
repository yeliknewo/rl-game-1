use utils::{CoordI};


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Point3I {
    x: CoordI,
    y: CoordI,
    z: CoordI,
}

impl Point3I {
    pub fn new(x: CoordI, y: CoordI, z: CoordI) -> Point3I {
        Point3I {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Point3I {
        Point3I::new(0, 0, 0)
    }

    pub fn get_mut_x(&mut self) -> &mut CoordI {
        &mut self.x
    }

    pub fn get_mut_y(&mut self) -> &mut CoordI {
        &mut self.y
    }

    pub fn get_mut_z(&mut self) -> &mut CoordI {
        &mut self.z
    }

    pub fn get_x(&self) -> CoordI {
        self.x
    }

    pub fn get_y(&self) -> CoordI {
        self.y
    }

    pub fn get_z(&self) -> CoordI {
        self.z
    }
}
