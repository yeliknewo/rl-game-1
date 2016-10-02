use std::collections::{HashMap};

use specs::{System, RunArg};

use components::{RenderId, Transform, Camera, RenderData};
use event::{BackChannel, WindowedEvent};
use graphics::{OutColor, OutDepth, Encoder, Bundle, Shaders, make_shaders, ProjectionData, TextureData, GlFactory, Packet, RlTexture, Primitive, pipe, FilterMethod, WrapMode, SamplerInfo, FactoryExt, Factory};
use utils::{Delta, WindowId};

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

        // warn!("Starting Render");
        let (render_ids, transforms, mut cameras, mut render_datas) = arg.fetch(|w|
            (
                w.read::<RenderId>(),
                w.read::<Transform>(),
                w.write::<Camera>(),
                w.write::<RenderData>()
            )
        );

        let out = self.sys.outs.get(&window_id).unwrap_or_else(|| panic!("Unable to find Outs for: {:?}", window_id));

        if window_id.0 == 1 {
            encoder.clear(&out.0, [1.0, 0.0, 0.0, 1.0]);
        } else {
            encoder.clear(&out.0, [0.0, 0.0, 1.0, 1.0]);
        }

        encoder.clear_depth(&out.1, 1.0);

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
            if render_id.clone_window_id() != window_id {
                // warn!("Render Ids: {:?}, {:?}", render_id.clone_window_id(), &window_id);
                continue;
            }
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

            datas.push((render_id.get_render_id_num(), render_data.get_layer(), texture_data, projection_data));
        }

        datas.sort_by_key(|k| k.1);

        for data in datas {
            let b = &self.sys.bundles_map.get(&window_id).unwrap_or_else(|| panic!("Can't find Bundle Vec for: {:?}", &window_id)).get(data.0).unwrap_or_else(|| panic!("Can't find bundle of: {:?}", data.0));

            if let Some(texture_data) = data.2 {
                encoder.update_constant_buffer(&b.get_data().texture_data, &texture_data);
            }

            if let Some(projection_data) = data.3 {
                encoder.update_constant_buffer(&b.get_data().projection_data, &projection_data);
            }

            b.encode(&mut encoder);
        }

        self.back_channel.send_from((window_id, FromRender::Encoder(encoder)));
    }

    fn set_graphics_data(&mut self, window_id: WindowId, out_color: OutColor, out_depth: OutDepth) {
        *self.sys.outs.get_mut(&window_id).unwrap_or_else(|| panic!("Can't find out for window id: {:?}", window_id)) = (out_color.clone(), out_depth.clone());

        for mut bundle in self.sys.bundles_map.get_mut(&window_id).unwrap_or_else(|| panic!("Can't find Bundle Vec for: {:?}", window_id)) {
            bundle.get_mut_data().out_color = out_color.clone();
            bundle.get_mut_data().out_depth = out_depth.clone();
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
    outs: HashMap<WindowId, (OutColor, OutDepth)>,
    bundles_map: HashMap<WindowId, Vec<Bundle>>,
    shaders: Shaders,
}

impl RenderSystemSend {
    pub fn new(outs: HashMap<WindowId, (OutColor, OutDepth)>) -> RenderSystemSend {
        warn!("Creating Render System Struct");
        RenderSystemSend {
            outs: outs,
            bundles_map: HashMap::new(),
            shaders: make_shaders(),
        }
    }

    pub fn add_render(&mut self,
        window_id: WindowId,
        factory: &mut GlFactory,
        packet: &Packet,
        texture: RlTexture
    ) -> RenderId {
        warn!("Creating Shader Set");
        let shader_set = factory.create_shader_set(self.shaders.get_vertex_shader(), self.shaders.get_fragment_shader()).unwrap_or_else(|err| panic!("Create Shader Set Error: {:?}", err));

        warn!("Creating Program");
        let program = factory.create_program(&shader_set).unwrap_or_else(|err| panic!("Create Program Error: {:?}", err));

        warn!("Creating Pipeline from Program");
        let pso = factory.create_pipeline_from_program(
            &program,
            Primitive::TriangleList,
            packet.get_rasterizer(),
            pipe::new()
        ).unwrap_or_else(|err| panic!("Create Pipeline from Program Error: {:?}", err));

        warn!("Creating Sampler Info");
        let sampler_info = SamplerInfo::new(
            FilterMethod::Scale,
            WrapMode::Mirror,
        );

        warn!("Creating Vertex Buffer");
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(packet.get_vertices(), packet.get_indices());

        warn!("Creating Pipe Data");
        let data = pipe::Data {
            vbuf: vbuf,
            spritesheet: (texture, factory.create_sampler(sampler_info)),
            texture_data: factory.create_constant_buffer(1),
            projection_data: factory.create_constant_buffer(1),
            out_color: self.outs.get(&window_id).unwrap_or_else(|| panic!("Can't find outs for: {:?}", window_id)).0.clone(),
            out_depth: self.outs.get(&window_id).unwrap_or_else(|| panic!("Can't find outs for: {:?}", window_id)).1.clone(),
        };

        warn!("Creating Id");


        warn!("Getting Bundles as Mutable");
        let mut bundles = {
            if self.bundles_map.get(&window_id).is_some() {
                self.bundles_map.get_mut(&window_id).unwrap_or_else(|| panic!("Can't find Bundle Vec for: {:?}", window_id))
            } else {
                self.bundles_map.insert(window_id.clone(), vec!());
                self.bundles_map.get_mut(&window_id).unwrap_or_else(|| panic!("Can't find Bundle Vec for: {:?}", window_id))
            }
        };

        let id = bundles.len();

        warn!("Adding new bundle to Bundles");
        bundles.push(Bundle::new(slice, pso, data));

        warn!("Returning Render Id");
        RenderId::new(window_id.clone(), id)
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
