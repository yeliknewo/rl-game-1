// use glutin::{Window, VirtualKeyCode};
// use glutin::Event::{MouseMoved, KeyboardInput, Closed, MouseInput, Resized};
// use glutin::ElementState::{Pressed, Released};

use sdl2::keyboard::{Keycode};
use sdl2::{EventPump};

use event::{FrontChannel};
use systems::control::{WindowedToControl, WindowedFromControl, ToControl, FromControl};
use systems::render::{WindowedToRender, WindowedFromRender, ToRender, FromRender};

// use ::render_thread::{FromRenderThread, ToRenderThread};

pub fn handle_events(event_pump: &mut EventPump, render_back_channel: &mut FrontChannel<WindowedToRender, WindowedFromRender>, control_back_channel: &mut FrontChannel<WindowedToControl, WindowedFromControl>) -> bool {
    for event in event_pump.poll_iter() {
        use sdl2::event::Event;
        use sdl2::event::{WindowEventId};
        match event {
            Event::Window {
                timestamp,
                window_id,
                win_event_id,
                data1,
                data2,
            } => {
                match win_event_id {
                    WindowEventId::Close => {
                        return true;
                    },
                    _ => (),
                }
            },
            Event::KeyDown {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat,
            } => {
                if let Some(keycode) = keycode {
                    match keycode {
                        Keycode::Escape => return true,
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    false
}
