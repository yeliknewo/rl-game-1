#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate find_folder;
extern crate image;

pub mod pipeline;
pub mod shaders;
pub mod textures;

use gfx::{Encoder};
use gfx::handle::{RenderTargetView, DepthStencilView};
use gfx::format::{Rgba8, DepthStencil};

use glutin::{WindowBuilder, Window};

pub use self::shaders::{Shaders};

pub type Device = gfx_device_gl::Device;
pub type GlFactory = gfx_device_gl::Factory;
pub type Resources = gfx_device_gl::Resources;
pub type CommandBuffer = gfx_device_gl::CommandBuffer;

pub type ColorFormat = Rgba8;
pub type DepthFormat = DepthStencil;

pub fn build_graphics(width: u32, height: u32) -> (
    (RenderTargetView<Resources, ColorFormat>, DepthStencilView<Resources, DepthFormat>),
    GlFactory,
    Encoder<Resources, CommandBuffer>,
    Window,
    Device
) {
    let builder = WindowBuilder::new()
        .with_title("RL_GAME_1")
        .with_dimensions(width, height)
        .with_vsync();

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