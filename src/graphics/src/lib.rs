#[macro_use]
extern crate gfx;
#[macro_use]
extern crate log;

extern crate components;
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

// pub type SdlGraphicsTup = (
//     (OutColor, OutDepth),
//     GlFactory,
//     Encoder,
//     sdl2::video::Window,
//     GlDevice,
//     sdl2::video::GLContext
// );

pub mod rl_sdl2 {
    use std::collections::{HashMap};

    use utils::{WindowId};

    use sdl2::{self, Sdl};
    use sdl2::video::{Window, GLContext};

    use gfx_window_sdl::{self};

    use ::{OutColor, OutDepth, GlFactory, Encoder, GlDevice};

    pub type WindowSettings<'a> = (&'a str, u32, u32);

    pub struct SdlGraphicEncoder {
        encoder: Encoder,
        sdl_graphic: SdlGraphic,
    }

    impl SdlGraphicEncoder {
        fn new(encoder: Encoder, sdl_graphic: SdlGraphic) -> SdlGraphicEncoder {
            SdlGraphicEncoder {
                encoder: encoder,
                sdl_graphic: sdl_graphic,
            }
        }

        pub fn take_encoder(self) -> (Encoder, SdlGraphic) {
            (self.encoder, self.sdl_graphic)
        }
    }

    pub struct SdlGraphic {
        out_color: OutColor,
        out_depth: OutDepth,
        factory: GlFactory,
        window: Window,
        device: GlDevice,
        gl_context: GLContext,
    }

    impl SdlGraphic {
        fn new(out_color: OutColor,
            out_depth: OutDepth,
            factory: GlFactory,
            window: Window,
            device: GlDevice,
            gl_context: GLContext) -> SdlGraphic {
            SdlGraphic {
                out_color: out_color,
                out_depth: out_depth,
                factory: factory,
                window: window,
                device: device,
                gl_context: gl_context,
            }
        }

        pub fn get_out_color(&self) -> OutColor {
            self.out_color.clone()
        }

        pub fn get_out_depth(&self) -> OutDepth {
            self.out_depth.clone()
        }

        pub fn get_factory(&self) -> &GlFactory {
            &self.factory
        }

        pub fn get_mut_factory(&mut self) -> &mut GlFactory {
            &mut self.factory
        }

        pub fn get_window(&self) -> &Window {
            &self.window
        }

        pub fn get_mut_window(&mut self) -> &mut Window {
            &mut self.window
        }

        pub fn get_device(&self) -> &GlDevice {
            &self.device
        }

        pub fn get_mut_device(&mut self) -> &mut GlDevice {
            &mut self.device
        }

        pub fn get_gl_context(&self) -> &GLContext {
            &self.gl_context
        }
    }

    pub fn build_graphics_sdl(window_settings_vec: Vec<WindowSettings>) -> (Sdl, HashMap<WindowId, SdlGraphicEncoder>)  {
        let sdl = sdl2::init().unwrap_or_else(|err| panic!("Error while sdl2::init: {:?}", err));

        let video = sdl.video().unwrap_or_else(|err| panic!("Error while making sdl.video(): {:?}", err));
        let gl_attr = video.gl_attr();
        gl_attr.set_context_version(3, 2);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        let mut out = HashMap::new();

        for window_settings in window_settings_vec {
            let (title, width, height) = window_settings;
            let mut builder = video.window(title, width, height);
            let (window, context, device, mut factory, out_color, out_depth) = gfx_window_sdl::init(&mut builder); //_ = glcontext

            let encoder = factory.create_command_buffer().into();

            out.insert(WindowId(window.id()), SdlGraphicEncoder::new(
                encoder,
                SdlGraphic::new(
                    out_color,
                    out_depth,
                    factory,
                    window,
                    device,
                    context
                )
            ));
        }

        (sdl, out)
    }
}

pub mod rl_glutin {
    use glutin::{Window, WindowBuilder};
    use gfx_window_glutin::{self};

    use ::{OutColor, OutDepth, GlFactory, Encoder, GlDevice, ColorFormat, DepthFormat};

    pub fn build_graphics_glutin(title: String, width: u32, height: u32) -> (
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
}
