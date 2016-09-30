use specs::{VecStorage, Component};

use math::{Matrix4, Point2, Vector3, Euler, Rad};
use math::prelude::{Zero};
use utils::{GfxCoord, Coord};

#[derive(Debug)]
pub struct Transform {
    translation: Vector3<GfxCoord>,
    rotation: Euler<Rad<GfxCoord>>,
    scale: Vector3<GfxCoord>,
}

impl Transform {
    pub fn new(pos: Vector3<GfxCoord>, rotation: Euler<Rad<GfxCoord>>, scale: Vector3<GfxCoord>) -> Transform {
        Transform {
            translation: pos,
            rotation: rotation,
            scale: scale,
        }
    }

    pub fn new_identity() -> Transform {
        Transform::new(
            Vector3::zero(),
            Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
            Vector3::new(1.0, 1.0, 1.0),
        )
    }

    pub fn set_pos(&mut self, pos: Vector3<GfxCoord>) {
        self.translation = pos;
    }

    pub fn add_pos(&mut self, pos_delta: Vector3<GfxCoord>) {
        self.translation += pos_delta;
    }

    pub fn get_pos(&self) -> Vector3<GfxCoord> {
        self.translation
    }

    pub fn get_model(&self) -> Matrix4<GfxCoord> {
        Matrix4::from_translation(self.translation) * Matrix4::from(self.rotation) * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    pub fn get_gui_offset(&self) -> Point2<Coord> {
        let translation = self.get_pos();
        Point2::new(-translation.x as Coord, -translation.y as Coord)
    }
}

impl Component for Transform {
    type Storage = VecStorage<Transform>;
}
