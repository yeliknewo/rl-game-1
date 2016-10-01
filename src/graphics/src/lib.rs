#[macro_use]
extern crate gfx;

extern crate dependencies;
extern crate utils;

pub use dependencies::{find_folder, gfx_device_gl, gfx_window_glutin, glutin, image};

use gfx::handle::{RenderTargetView, DepthStencilView};
use gfx::format::{Rgba8, DepthStencil};

use glutin::{WindowBuilder};

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
pub type ColorFormat = Rgba8;
pub type DepthFormat = DepthStencil;
pub type OutColor = RenderTargetView<Resources, ColorFormat>;
pub type OutDepth = DepthStencilView<Resources, DepthFormat>;
pub type Encoder = gfx::Encoder<Resources, CommandBuffer>;
pub type RlTexture = gfx::handle::ShaderResourceView<Resources, [f32; 4]>;

pub use gfx::traits::{Factory, FactoryExt};
pub use glutin::{Window, Event, VirtualKeyCode, ElementState, MouseButton};
pub use gfx_window_glutin::{update_views};
pub use gfx::{Device, Primitive};
pub use gfx::state::{Rasterizer};
pub use gfx::tex::{FilterMethod, SamplerInfo, WrapMode};

pub fn build_graphics(title: String, width: u32, height: u32) -> (
    (OutColor, OutDepth),
    GlFactory,
    Encoder,
    Window,
    GlDevice
) {
    let builder = WindowBuilder::new()
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
