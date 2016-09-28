extern crate event;
extern crate graphics;
extern crate math;

use event::{two_way_channel};
use graphics::{build_graphics};
use math::{OrthographicHelper};

pub fn start() {
    let (width, height): (u32, u32) = (640, 480);

    let fov = 90.0;

    let znear = 0.0;
    let zfar = 10.0;

    let aspect_ratio = width as f32 / height as f32;

    let orthohelper = OrthographicHelper::new(aspect_ratio, fov, znear, zfar);

    let ((mut out_color, mut out_depth), mut factory, encoder, window, mut device) = build_graphics(640, 480);

    let (mut event_dev, game_event) = two_way_channel();

    event_dev.send_to_render(ToRender::GraphicsData(out_color.clone(), out_depth.clone()));

    event_dev.send_to_render(ToRender::Encoder(encoder.clone_empty()));
    event_dev.send_to_render(ToRender::Encoder(encoder));
}
