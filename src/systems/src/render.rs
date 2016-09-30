use std::sync::{Arc};

use specs::{System, RunArg};

use components::{RenderId, Transform, Camera, RenderData};
use event::{BackChannel};
use graphics::{OutColor, OutDepth, Encoder, Bundle, Shaders, make_shaders, ProjectionData, TextureData};
use utils::{Delta};

pub enum ToRender {
    GraphicsData(OutColor, OutDepth),
    Encoder(Encoder),
}

pub enum FromRender {
    Encoder(Encoder),
}

pub struct RenderSystem {
    back_channel: BackChannel<ToRender, FromRender>,
    out_color: OutColor,
    out_depth: OutDepth,
    bundles: Arc<Vec<Bundle>>,
    shaders: Shaders,
}

impl RenderSystem {
    pub fn new(back_channel: BackChannel<ToRender, FromRender>) -> RenderSystem {
        let (out_color, out_depth) = match back_channel.recv_to() {
            ToRender::GraphicsData(out_color, out_depth) => (out_color, out_depth),
            _ => panic!("render system receieved non graphics data first from channel"),
        };

        RenderSystem {
            back_channel: back_channel,
            out_color: out_color,
            out_depth: out_depth,
            bundles: Arc::new(Vec::new()),
            shaders: make_shaders(),
        }
    }

    fn render(&mut self, arg: &RunArg, mut encoder: Encoder) {
        use specs::Join;

        let (render_ids, transforms, mut cameras, mut render_datas) = arg.fetch(|w|
            (
                w.read::<RenderId>(),
                w.read::<Transform>(),
                w.write::<Camera>(),
                w.write::<RenderData>()
            )
        );

        encoder.clear(&self.out_color, [1.0, 1.0, 1.0, 1.0]);
        encoder.clear_depth(&self.out_depth, 1.0);

        let (view, proj, dirty_cam) = {
            let mut camera = {
                let mut camera_opt = None;

                for c in (&mut cameras).iter() {
                    camera_opt = Some(c);
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
                        model: transform.get_model(),
                        view: view,
                        proj: proj,
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

            datas.push((render_id.id, render_data.get_layer(), texture_data, projection_data));
        }

        datas.sort_by_key(|k| k.1);

        for data in datas {
            let b = &self.bundles[data.0];

            if let Some(texture_data) = data.2 {
                encoder.update_constant_buffer(&b.data.texture_data, &texture_data);
            }

            if let Some(projection_data) = data.3 {
                encoder.update_constant_buffer(&b.data.projection_data, &projection_data);
            }

            b.encode(&mut encoder);
        }

        self.back_channel.send_from(FromRender::Encoder(encoder));
    }

    fn set_graphics_data(&mut self, out_color: OutColor, out_depth: OutDepth) {
        self.out_color = out_color;
        self.out_depth = out_depth;

        for bundle in Arc::get_mut(&mut self.bundles).unwrap() {
            bundle.data.out_color = self.out_color.clone();
            bundle.data.out_depth = self.out_depth.clone();
        }
    }

    fn process_event(&mut self, arg: &RunArg, event: ToRender) -> bool {
        match event {
            ToRender::Encoder(encoder) => {
                self.render(arg, encoder);
                false
            },
            ToRender::GraphicsData(out_color, out_depth) => {
                self.set_graphics_data(out_color, out_depth);
                true
            },
        }
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
