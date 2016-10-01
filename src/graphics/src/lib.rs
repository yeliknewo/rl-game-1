#[macro_use]
extern crate gfx;

extern crate dependencies;
extern crate utils;

pub use dependencies::{find_folder, gfx_device_gl, gfx_window_glutin, glutin, sdl2, gfx_window_sdl, image};

use gfx::handle::{RenderTargetView, DepthStencilView};
use gfx::format::{Srgba8, DepthStencil};

pub mod pipeline;
pub mod shaders;
pub mod textures;

pub use self::pipeline::{pipe, Vertex, Bundle, Index, Packet, make_shaders, ProjectionData, TextureData};
pub use self::shaders::{Shaders};
pub use self::textures::{load_texture};

pub type GlDevice = gfx_device_gl::Device;
pub type GlFactory = gfx_device_gl::Factory;
pub type Resources = gfx_device_gl::Resources;
pub type CommandBuffer = gfx_device_gl::CommandBuffer;
pub type ColorFormat = Srgba8;
pub type DepthFormat = DepthStencil;
pub type OutColor = RenderTargetView<Resources, ColorFormat>;
pub type OutDepth = DepthStencilView<Resources, DepthFormat>;
pub type Encoder = gfx::Encoder<Resources, CommandBuffer>;
pub type RlTexture = gfx::handle::ShaderResourceView<Resources, [f32; 4]>;

pub use gfx::traits::{Factory, FactoryExt};
pub use gfx::{Device, Primitive};
pub use gfx::state::{Rasterizer};
pub use gfx::tex::{FilterMethod, SamplerInfo, WrapMode};

pub fn build_graphics_sdl(title: String, width: u32, height: u32) -> (
    (OutColor, OutDepth),
    GlFactory,
    Encoder,
    sdl2::video::Window,
    GlDevice
) {
    let sdl = sdl2::init().unwrap();

    let mut builder = sdl.video().unwrap().window(title.as_str(), width, height);
    let (window, _, device, mut factory, out_color, out_depth) = gfx_window_sdl::init(&mut builder); //_ = glcontext

    let encoder = factory.create_command_buffer().into();

    (
        (
            out_color,
            out_depth
        ),
        factory,
        encoder,
        window,
        device
    )
}

pub fn build_graphics_glutin(title: String, width: u32, height: u32) -> (
    (OutColor, OutDepth),
    GlFactory,
    Encoder,
    glutin::Window,
    GlDevice
) {
    let builder = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height);
        // .with_vsync();

    let (window, device, mut factory, out_color, out_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let encoder = factory.create_command_buffer().into();

    (
        (
            out_color,
            out_depth
        ),
        factory,
        encoder,
        window,
        device
    )
}
