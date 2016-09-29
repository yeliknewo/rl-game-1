extern crate event;
extern crate graphics;
extern crate math;
extern crate systems;

use event::{two_way_channel, FrontChannel, BackChannel};
use graphics::{build_graphics};
use math::{OrthographicHelper};
use systems::render::{ToRender};

pub fn start() {
    let (width, height): (u32, u32) = (640, 480);

    let fov = 90.0;

    let znear = 0.0;
    let zfar = 10.0;

    let aspect_ratio = width as f32 / height as f32;

    let orthohelper = OrthographicHelper::new(aspect_ratio, fov, znear, zfar);

    let ((mut out_color, mut out_depth), mut factory, encoder, window, mut device) = build_graphics(640, 480);

    let (mut render_event_dev, render_game_event) = two_way_channel::<ToRender, ()>();

    render_event_dev.send_to(ToRender::GraphicsData(out_color.clone(), out_depth.clone()));

    render_event_dev.send_to(ToRender::Encoder(encoder.clone_empty()));
    render_event_dev.send_to(ToRender::Encoder(encoder));
}
