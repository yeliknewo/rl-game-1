use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub enum WindowId {
    First,
    Second,
}

impl Component for WindowId {
    type Storage = VecStorage<WindowId>;
}
