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

pub mod layers {
    pub const TILES: u8 = 0;
}

pub mod tiles {
    pub const NAME: &'static str = "tiles.png";
    pub const SIZE: &'static [f32; 2] = &[256.0, 256.0];
    pub const DEFAULT_TINT: &'static [f32; 4] = &[0.5, 0.5, 0.5, 1.0];

    pub const EMPTY: &'static [f32; 4] = &[0.0, 0.0, 32.0, 31.5];
}
