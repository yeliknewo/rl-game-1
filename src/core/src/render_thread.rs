use std::thread::{self, JoinHandle};

use art::{make_square_render, tiles};
use components::{RenderId};
use event::{BackChannel};
use graphics::{build_graphics, update_views, load_texture, Device};
use systems::render::{RenderSystemSend, ToRender, FromRender};
use systems::control::{ToControl, FromControl};
use utils::{Search, WindowId};

use ::handle_events::handle_events;

pub enum ToRenderThread {
    RenderSystem(RenderSystemSend),
    FromRender(FromRender),
    FromControl(FromControl),
}

pub enum FromRenderThread {
    RenderSystem(RenderSystemSend, Vec<RenderId>),
    ToRender(ToRender),
    ToControl(ToControl),
}

pub fn start(window_id: WindowId, mut back_channel: BackChannel<ToRenderThread, FromRenderThread>) -> JoinHandle<()> {
    thread::spawn(move || {
        let ((mut out_color, mut out_depth), mut factory, encoder, mut window, mut device) = build_graphics(640, 480);

        back_channel.send_from(FromRenderThread::ToRender(ToRender::GraphicsData(out_color.clone(), out_depth.clone())));

        match back_channel.recv_to() {
            ToRenderThread::RenderSystem(mut render_system) => {
                let packet = make_square_render();

                let assets_folder = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

                let tiles_render = {
                    let texture = load_texture(
                        &mut factory,
                        assets_folder.join(
                            tiles::NAME
                        )
                    );
                    render_system.add_render(
                        window_id.clone(),
                        &mut factory,
                        &packet,
                        texture
                    )
                };

                back_channel.send_from(FromRenderThread::RenderSystem(render_system, vec!(tiles_render)));
            },
            _ => panic!("shits fucked"),
        }

        back_channel.send_from(FromRenderThread::ToRender(ToRender::Encoder(encoder.clone_empty())));
        back_channel.send_from(FromRenderThread::ToRender(ToRender::Encoder(encoder)));

        'render: loop {
            match back_channel.recv_to() {
                ToRenderThread::FromRender(FromRender::Encoder(mut encoder)) => {
                    if handle_events(&mut window, &mut back_channel) {
                        break 'render;
                    }

                    encoder.flush(&mut device);
                    back_channel.send_from(FromRenderThread::ToRender(ToRender::Encoder(encoder)));
                    window.swap_buffers().unwrap();
                    device.cleanup();
                },
                ToRenderThread::FromControl(FromControl::Resize) => {
                    update_views(&window, &mut out_color, &mut out_depth);
                    back_channel.send_from(FromRenderThread::ToRender(ToRender::GraphicsData(out_color.clone(), out_depth.clone())));
                },
                ToRenderThread::RenderSystem(_) => panic!("shits fucked"),
            }
        }
    })
}
