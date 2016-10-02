extern crate graphics;

use graphics::{Packet, Vertex, Rasterizer};

pub fn make_square_render() -> Packet {
    let vertices = vec!(
        Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    );

    let indices = vec!(
        0, 3, 2, 2, 1, 0,
    );

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

pub type RenderType = u8;
pub type Layer = u8;
pub type Name = &'static str;
pub type Size = &'static [f32; 2];
pub type Tint = &'static [f32; 4];
pub type Sprite = &'static [f32; 4];

pub mod layers {
    use ::{Layer};

    pub const TILES: Layer = 0;
}

pub mod tiles {
    use ::{Name, Size, Tint, RenderType, Sprite};

    pub const NAME: Name = "tiles.png";
    pub const SIZE: Size = &[256.0, 256.0];
    pub const DEFAULT_TINT: Tint = &[0.5, 0.5, 0.5, 1.0];
    pub const ID: RenderType = 0;

    pub const EMPTY: Sprite = &[0.0, 0.0, 32.0, 31.5];
}
