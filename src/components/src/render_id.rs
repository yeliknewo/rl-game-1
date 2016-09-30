use specs::{Component, VecStorage};

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct RenderId(usize);

impl Component for RenderId {
    type Storage = VecStorage<RenderId>;
}
