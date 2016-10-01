use event::{BackChannel};
use graphics::{Window, VirtualKeyCode};
use graphics::Event::{MouseMoved, KeyboardInput, Closed, MouseInput, Resized};
use graphics::ElementState::{Pressed, Released};
use systems::control::{ToControl};

use ::render_thread::{FromRenderThread, ToRenderThread};

pub fn handle_events(window: &mut Window, back_channel: &mut BackChannel<ToRenderThread, FromRenderThread>) -> bool {
    for event in window.poll_events() {
        match event {
            KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
            Closed => return true,
            MouseMoved(x, y) => back_channel.send_from(FromRenderThread::ToControl(ToControl::MouseMoved(x as u32, y as u32))),
            MouseInput(state, button) => back_channel.send_from(FromRenderThread::ToControl(ToControl::MouseInput(match state {
                Pressed => true,
                Released => false,
            }, button))),
            KeyboardInput(state, _, Some(VirtualKeyCode::D)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Right)) => back_channel.send_from(FromRenderThread::ToControl(ToControl::Right(match state {
                Pressed => true,
                Released => false,
            }))),
            KeyboardInput(state, _, Some(VirtualKeyCode::A)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Left)) => back_channel.send_from(FromRenderThread::ToControl(ToControl::Left(match state {
                Pressed => true,
                Released => false,
            }))),
            KeyboardInput(state, _, Some(VirtualKeyCode::W)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Up)) => back_channel.send_from(FromRenderThread::ToControl(ToControl::Up(match state {
                Pressed => true,
                Released => false,
            }))),
            KeyboardInput(state, _, Some(VirtualKeyCode::S)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Down)) => back_channel.send_from(FromRenderThread::ToControl(ToControl::Down(match state {
                Pressed => true,
                Released => false,
            }))),
            Resized(width, height) => back_channel.send_from(FromRenderThread::ToControl(ToControl::Resize(width, height))),
            _ => (),
        }
    }

    false
}
