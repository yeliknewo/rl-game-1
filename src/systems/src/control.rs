use specs::{System, RunArg};
use glutin::{MouseButton};

use event::{BackChannel, WindowedEvent};
use utils::{Delta};

#[derive(Debug)]
pub enum ToControl {
    MouseMoved(u32, u32),
    MouseInput(bool, MouseButton),
    Right(bool),
    Left(bool),
    Up(bool),
    Down(bool),
    Resize(u32, u32),
}

#[derive(Debug)]
pub enum FromControl {
    Resize,
}

pub type WindowedToControl = WindowedEvent<ToControl>;

pub type WindowedFromControl = WindowedEvent<FromControl>;

#[derive(Debug)]
pub struct ControlSystem {
    back_channel: BackChannel<WindowedToControl, WindowedFromControl>,
}

impl ControlSystem {
    pub fn new(back_channel: BackChannel<WindowedToControl, WindowedFromControl>) -> ControlSystem {
        ControlSystem {
            back_channel: back_channel,
        }
    }
}

impl System<Delta> for ControlSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        arg.fetch(|_| ());
    }
}
