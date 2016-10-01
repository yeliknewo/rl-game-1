#[macro_use]
extern crate log;

extern crate art;
extern crate components;
extern crate event;
extern crate graphics;
extern crate math;
extern crate systems;
extern crate utils;

use std::thread;

use event::{two_way_channel};
use math::{OrthographicHelper};
use systems::render::{RenderSystemSend, ToRender};
use utils::{WindowId};

mod game;
mod handle_events;
mod render_thread;

use game::Game;
use render_thread::{ToRenderThread, FromRenderThread};

pub fn start() {
    warn!("Starting Core Start");
    let (width, height): (u32, u32) = (640, 480);

    let left = -10.0;
    let right = 10.0;

    let near = 0.0;
    let far = 10.0;

    let aspect_ratio = width as f32 / height as f32;

    let ortho_helper = OrthographicHelper::new(aspect_ratio, left, right, near, far);

    let (mut render_event_dev_1, render_event_1) = two_way_channel();
    let (mut render_event_dev_2, render_event_2) = two_way_channel();

    // let render_1_handle =
    warn!("Starting First Render Thread");
    render_thread::start(WindowId::First, render_event_1);
    // let render_2_handle =
    warn!("Starting Second Render Thread");
    render_thread::start(WindowId::Second, render_event_2);
    warn!("Finished Starting Second Render Thread");

    let (mut render_event_core, render_event_game) = two_way_channel();
    let (mut control_event_core, control_event_game) = two_way_channel();



    let (out_color_1, out_depth_1) = match render_event_dev_1.recv_from() {
        FromRenderThread::ToRender(ToRender::GraphicsData(out_color, out_depth)) => (out_color, out_depth),
        _ => panic!("Expected Graphics Data"),
    };

    let (out_color_2, out_depth_2) = match render_event_dev_2.recv_from() {
        FromRenderThread::ToRender(ToRender::GraphicsData(out_color, out_depth)) => (out_color, out_depth),
        _ => panic!("Expected Graphics Data"),
    };

    render_event_dev_1.send_to(ToRenderThread::RenderSystem(RenderSystemSend::new(out_color_1, out_depth_1, out_color_2, out_depth_2)));
    let (render_system, render_ids_1) = match render_event_dev_1.recv_from() {
        FromRenderThread::RenderSystem(render_system, render_ids) => (render_system, render_ids),
        _ => panic!("Shits fucked"),
    };

    render_event_dev_2.send_to(ToRenderThread::RenderSystem(render_system));
    let (render_system, render_ids_2) = match render_event_dev_2.recv_from() {
        FromRenderThread::RenderSystem(render_system, render_ids) => (render_system, render_ids),
        _ => panic!("Shits fucked"),
    };

    warn!("Creating Game");
    let game = Game::new(
        render_ids_1,
        render_ids_2,
        render_system,
        render_event_game,
        control_event_game,
        ortho_helper
    );

    warn!("Creating Game Thread");
    thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });

    warn!("Entering Main Loop");
    'main: loop {

        if let Some(event) = render_event_core.try_recv_from() {
            match event {
                (WindowId::First, event) => render_event_dev_1.send_to(ToRenderThread::FromRender(event)),
                (WindowId::Second, event) => render_event_dev_2.send_to(ToRenderThread::FromRender(event)),
            }
        }

        if let Some(event) = control_event_core.try_recv_from() {
            match event {
                (WindowId::First, event) => render_event_dev_1.send_to(ToRenderThread::FromControl(event)),
                (WindowId::Second, event) => render_event_dev_2.send_to(ToRenderThread::FromControl(event)),
            }
        }

        if let Some(event) = render_event_dev_1.try_recv_from() {
            match event {
                FromRenderThread::ToRender(event) => render_event_core.send_to((WindowId::First, event)),
                FromRenderThread::ToControl(event) => control_event_core.send_to((WindowId::First, event)),
                FromRenderThread::RenderSystem(_, _) => panic!("shits fucked"),
            }
        }

        if let Some(event) = render_event_dev_2.try_recv_from() {
            match event {
                FromRenderThread::ToRender(event) => render_event_core.send_to((WindowId::Second, event)),
                FromRenderThread::ToControl(event) => control_event_core.send_to((WindowId::Second, event)),
                FromRenderThread::RenderSystem(_, _) => panic!("shits fucked"),
            }
        }
    }

    // render_1_handle.join().unwrap();
    // render_2_handle.join().unwrap();
}
