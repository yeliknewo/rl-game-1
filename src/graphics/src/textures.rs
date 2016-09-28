use std::path::{Path};

use gfx::{Factory};
use gfx::handle::{ShaderResourceView};
use gfx::tex::{Size, AaMode, Kind};

use image;

use ::{Resources, GlFactory, ColorFormat};

pub fn load_texture<P>(factory: &mut GlFactory, path: P) -> ShaderResourceView<Resources, [f32; 4]>
where P: AsRef<Path> {
    let image = match image::open(path) {
        Ok(image) => image,
        Err(err) => panic!("image load error: {}", err),
    }.to_rgba();
    let (width, height) = image.dimensions();
    let kind = Kind::D2(width as Size, height as Size, AaMode::Single);
    let (_, view) = match factory.create_texture_const_u8::<ColorFormat>(kind, &[&image]) {
        Ok(data) => data,
        Err(err) => panic!("factory create texture const error: {}", err),
    };
    view
}
