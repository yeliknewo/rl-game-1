use specs::{Component, VecStorage};

use utils::{WindowId};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RenderId {
    window_id: WindowId,
    render_id_num: usize,
}

impl RenderId {
    pub fn new(window_id: WindowId, render_id_num: usize) -> RenderId {
        RenderId {
            window_id: window_id,
            render_id_num: render_id_num,
        }
    }

    pub fn clone_window_id(&self) -> WindowId {
        self.window_id.clone()
    }

    pub fn get_render_id_num(&self) -> usize {
        self.render_id_num
    }
}

impl Component for RenderId {
    type Storage = VecStorage<RenderId>;
}
