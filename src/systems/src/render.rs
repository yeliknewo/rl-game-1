use std::sync::{Arc};

use specs::{System, RunArg};

use components::{RenderId, Transform, Camera, RenderData, WindowId};
use event::{BackChannel, WindowedEvent};
use graphics::{OutColor, OutDepth, Encoder, Bundle, Shaders, make_shaders, ProjectionData, TextureData, GlFactory, Packet, RlTexture, Primitive, pipe, FilterMethod, WrapMode, SamplerInfo, FactoryExt, Factory};
use utils::{Delta};

pub enum ToRender {
    GraphicsData(OutColor, OutDepth),
    Encoder(Encoder),
}

pub enum FromRender {
    Encoder(Encoder),
}

pub type WindowedToRender = WindowedEvent<ToRender>;

pub type WindowedFromRender = WindowedEvent<FromRender>;

pub struct RenderSystem {
    back_channel: BackChannel<WindowedToRender, WindowedFromRender>,
    sys: RenderSystemSend,
}

impl RenderSystem {
    pub fn new(back_channel: BackChannel<WindowedToRender, WindowedFromRender>, send: RenderSystemSend) -> RenderSystem {
        RenderSystem {
            back_channel: back_channel,
            sys: send,
        }
    }

    fn render(&mut self, arg: &RunArg, window_id: WindowId, mut encoder: Encoder) {
        use specs::Join;

        let (render_ids, transforms, mut cameras, mut render_datas) = arg.fetch(|w|
            (
                w.read::<RenderId>(),
                w.read::<Transform>(),
                w.write::<Camera>(),
                w.write::<RenderData>()
            )
        );

        // match window_id {
        //     WindowId::First => {
        //         warn!("rendering to first window");
        //     },
        //     WindowId::Second => {
        //         warn!("rendering to second window");
        //     },
        // }

        match window_id {
            WindowId::First => {
                encoder.clear(&self.sys.out_color_1, [1.0, 1.0, 1.0, 1.0]);
                encoder.clear_depth(&self.sys.out_depth_1, 1.0);
            },
            WindowId::Second => {
                encoder.clear(&self.sys.out_color_2, [0.5, 0.5, 0.5, 1.0]);
                encoder.clear_depth(&self.sys.out_depth_2, 1.0);
            },
        }

        let (view, proj, dirty_cam) = {
            let mut camera = {
                let mut camera_opt = None;

                for c in (&mut cameras).iter() {
                    if c.is_main() {
                        camera_opt = Some(c);
                    }
                }

                camera_opt.expect("No camera entity was found by render")
            };

            (camera.get_view(), camera.get_proj(), camera.take_dirty())
        };

        let mut datas = vec!();

        for (render_id, transform, mut render_data) in (&render_ids, &transforms, &mut render_datas).iter() {
            let mut projection_data = None;

            if dirty_cam {
                projection_data = Some(
                    ProjectionData {
                        model: transform.get_model().into(),
                        view: view.into(),
                        proj: proj.into(),
                    }
                );
            }

            let mut texture_data = None;

            if render_data.take_dirty() {
                texture_data = Some(
                    TextureData {
                        tint: render_data.get_tint(),
                        spritesheet_rect: render_data.get_spritesheet_rect(),
                        spritesheet_size: render_data.get_spritesheet_size(),
                        mirror_x: render_data.get_mirror_x(),
                        mirror_y: render_data.get_mirror_y(),
                    }
                );
            }

            datas.push((render_id.0, render_data.get_layer(), texture_data, projection_data));
        }

        datas.sort_by_key(|k| k.1);

        match window_id {
            WindowId::First => {
                for data in datas {
                    let b = &self.sys.bundles_1[data.0];

                    if let Some(texture_data) = data.2 {
                        encoder.update_constant_buffer(&b.data.texture_data, &texture_data);
                    }

                    if let Some(projection_data) = data.3 {
                        encoder.update_constant_buffer(&b.data.projection_data, &projection_data);
                    }

                    b.encode(&mut encoder);
                }
            },
            WindowId::Second => {
                for data in datas {
                    let b = &self.sys.bundles_2[data.0];

                    if let Some(texture_data) = data.2 {
                        encoder.update_constant_buffer(&b.data.texture_data, &texture_data);
                    }

                    if let Some(projection_data) = data.3 {
                        encoder.update_constant_buffer(&b.data.projection_data, &projection_data);
                    }

                    b.encode(&mut encoder);
                }
            },
        }

        self.back_channel.send_from((window_id, FromRender::Encoder(encoder)));
    }

    fn set_graphics_data(&mut self, window_id: WindowId, out_color: OutColor, out_depth: OutDepth) {
        match window_id {
            WindowId::First => {
                self.sys.out_color_1 = out_color;
                self.sys.out_depth_1 = out_depth;

                for bundle in Arc::get_mut(&mut self.sys.bundles_1).unwrap() {
                    bundle.data.out_color = self.sys.out_color_1.clone();
                    bundle.data.out_depth = self.sys.out_depth_1.clone();
                }
            },
            WindowId::Second => {
                self.sys.out_color_2 = out_color;
                self.sys.out_depth_2 = out_depth;

                for bundle in Arc::get_mut(&mut self.sys.bundles_2).unwrap() {
                    bundle.data.out_color = self.sys.out_color_2.clone();
                    bundle.data.out_depth = self.sys.out_depth_2.clone();
                }
            },
        }
    }

    fn process_event(&mut self, arg: &RunArg, event: WindowedToRender) -> bool {
        match event {
            (window_id, ToRender::Encoder(encoder)) => {
                self.render(arg, window_id, encoder);
                false
            },
            (window_id, ToRender::GraphicsData(out_color, out_depth)) => {
                self.set_graphics_data(window_id, out_color, out_depth);
                true
            },
        }
    }
}

pub struct RenderSystemSend {
    out_color_1: OutColor,
    out_depth_1: OutDepth,
    out_color_2: OutColor,
    out_depth_2: OutDepth,
    bundles_1: Arc<Vec<Bundle>>,
    bundles_2: Arc<Vec<Bundle>>,
    shaders: Shaders,
}

impl RenderSystemSend {
    pub fn new(out_color_1: OutColor, out_depth_1: OutDepth, out_color_2: OutColor, out_depth_2: OutDepth) -> RenderSystemSend {
        // warn!("Starting New Render System");
        // let (mut first, mut second) = (None, None);
        //
        // warn!("Waiting For Init Values From Back Channel");
        // while first.is_none() || second.is_none() {
        //     match back_channel.recv_to() {
        //         (WindowId::First, ToRender::GraphicsData(out_color, out_depth)) => first = Some((out_color, out_depth)),
        //         (WindowId::Second, ToRender::GraphicsData(out_color, out_depth)) => second = Some((out_color, out_depth)),
        //         _ => panic!("got the wrong values in channel when starting render system"),
        //     }
        // }

        warn!("Creating Render System Struct");
        RenderSystemSend {
            // back_channel: back_channel,
            // out_color_1: first.as_ref().unwrap().0.clone(),
            // out_depth_1: first.unwrap().1,
            // out_color_2: second.as_ref().unwrap().0.clone(),
            // out_depth_2: second.unwrap().1,
            out_color_1: out_color_1,
            out_depth_1: out_depth_1,
            out_color_2: out_color_2,
            out_depth_2: out_depth_2,
            bundles_1: Arc::new(Vec::new()),
            bundles_2: Arc::new(Vec::new()),
            shaders: make_shaders(),
        }
    }

    pub fn add_render(&mut self,
        window_id: WindowId,
        factory: &mut GlFactory,
        packet: &Packet,
        texture: RlTexture
    ) -> RenderId {
        let shader_set = factory.create_shader_set(self.shaders.get_vertex_shader(), self.shaders.get_fragment_shader()).unwrap();

        let program = factory.create_program(&shader_set).unwrap();

        let pso = factory.create_pipeline_from_program(
            &program,
            Primitive::TriangleList,
            packet.get_rasterizer(),
            pipe::new()
        ).unwrap();

        let sampler_info = SamplerInfo::new(
            FilterMethod::Scale,
            WrapMode::Mirror,
        );

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(packet.get_vertices(), packet.get_indices());

        let data = {
            match window_id {
                WindowId::First => {
                    pipe::Data {
                        vbuf: vbuf,
                        spritesheet: (texture, factory.create_sampler(sampler_info)),
                        texture_data: factory.create_constant_buffer(1),
                        projection_data: factory.create_constant_buffer(1),
                        out_color: self.out_color_1.clone(),
                        out_depth: self.out_depth_1.clone(),
                    }
                },
                WindowId::Second => {
                    pipe::Data {
                        vbuf: vbuf,
                        spritesheet: (texture, factory.create_sampler(sampler_info)),
                        texture_data: factory.create_constant_buffer(1),
                        projection_data: factory.create_constant_buffer(1),
                        out_color: self.out_color_2.clone(),
                        out_depth: self.out_depth_2.clone(),
                    }
                },
            }
        };

        let id = {
            match window_id {
                WindowId::First => {
                    let id = self.bundles_1.len();
                    let mut bundles = Arc::get_mut(&mut self.bundles_1).unwrap();
                    bundles.push(Bundle::new(slice, pso, data));
                    id
                },
                WindowId::Second => {
                    let id = self.bundles_2.len();
                    let mut bundles = Arc::get_mut(&mut self.bundles_2).unwrap();
                    bundles.push(Bundle::new(slice, pso, data));
                    id
                }
            }
        };

        RenderId(id)
    }
}

impl System<Delta> for RenderSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let mut event = self.back_channel.recv_to();
        while self.process_event(&arg, event) {
            event = self.back_channel.recv_to();
        }
    }
}
