#[macro_use]
extern crate log;

extern crate art;
extern crate components;
extern crate dependencies;
extern crate event;
extern crate graphics;
extern crate math;
extern crate systems;
extern crate utils;

use std::thread;
use std::collections::HashMap;

use art::{RenderType};
pub use dependencies::{find_folder, sdl2};
use components::{RenderId};
use event::{two_way_channel};
use graphics::{Device, OutColor, OutDepth};
use graphics::rl_sdl2::{build_graphics_sdl};
use math::{OrthographicHelper};
use systems::control::{WindowedFromControl, WindowedToControl, FromControl};
use systems::render::{RenderSystemSend, WindowedFromRender, WindowedToRender, FromRender, ToRender};
use utils::{WindowId};

mod game;
mod handle_events;

use game::Game;
use handle_events::{handle_events};

#[derive(Hash, Eq, PartialEq)]
pub struct RenderKey {
    window_id: WindowId,
    render_const: RenderType,
}

impl RenderKey {
    pub fn new(window_id: WindowId, render_const: RenderType) -> RenderKey {
        RenderKey {
            window_id: window_id,
            render_const: render_const,
        }
    }

    pub fn clone_window_id(&self) -> WindowId {
        self.window_id.clone()
    }

    pub fn get_window_id(&self) -> &WindowId {
        &self.window_id
    }
}

pub type RenderIds = HashMap<RenderKey, RenderId>;

pub fn start() {
    warn!("Starting Core Start");
    let (width, height): (u32, u32) = (640, 480);

    let left = -10.0;
    let right = 10.0;

    let near = 0.0;
    let far = 10.0;

    let aspect_ratio = width as f32 / height as f32;

    let ortho_helper = OrthographicHelper::new(aspect_ratio, left, right, near, far);

    warn!("Building SDL Graphics");
    let (sdl, mut sdl_graphic_encoders) = build_graphics_sdl(vec!(("First Window", 640, 480), ("Second Window", 640, 480)));

    warn!("Make two way channels");
    let (mut render_event_core, render_event_game) = two_way_channel::<WindowedToRender, WindowedFromRender>();
    let (mut control_event_core, control_event_game) = two_way_channel::<WindowedToControl, WindowedFromControl>();

    let mut outs: HashMap<WindowId, (OutColor, OutDepth)> = HashMap::new();
    let mut stage_two_encoder = HashMap::new();

    let mut sdl_graphics = HashMap::new();

    warn!("Sdl Graphic Encoders Len: {:?}", sdl_graphic_encoders.len());

    for (window_id, sdl_graphic_encoder) in sdl_graphic_encoders.drain() {
        // let ((mut out_color, mut out_depth), mut factory, mut encoder, mut window, mut device, context) = sdl_graphic;

        let (encoder, sdl_graphic) = sdl_graphic_encoder.take_encoder();

        let out_color = sdl_graphic.get_out_color();
        let out_depth = sdl_graphic.get_out_depth();

        outs.insert(window_id.clone(), (out_color, out_depth));
        sdl_graphics.insert(window_id.clone(), sdl_graphic);

        render_event_core.send_to((window_id.clone(), ToRender::Encoder(encoder.clone_empty())));
        stage_two_encoder.insert(window_id, encoder);
    }

    for (window_id, encoder) in stage_two_encoder.drain() {
        render_event_core.send_to((window_id, ToRender::Encoder(encoder)));
    }

    warn!("Making Render System");
    let mut render_system = RenderSystemSend::new(outs);

    warn!("Making Square Render");
    let packet = art::make_square_render();

    warn!("Finding assets folder");
    let assets_folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap_or_else(|err| panic!("Unable to find Assets Folder: {:?}", err));

    let mut render_ids: RenderIds = HashMap::new();

    warn!("Sdl Graphics Len: {:?}", sdl_graphics.len());

    warn!("Making Tiles Render");
    for (window_id, mut sdl_graphic) in sdl_graphics.iter_mut() {
        let tiles_render = {
            let texture = graphics::textures::load_texture(
                sdl_graphic.get_mut_factory(),
                assets_folder.join(
                    art::tiles::NAME
                )
            );
            render_system.add_render(
                window_id.clone(),
                sdl_graphic.get_mut_factory(),
                &packet,
                texture
            )
        };

        render_ids.insert(RenderKey::new(window_id.clone(), art::tiles::ID), tiles_render);
    }

    warn!("Render Ids Len: {:?}", render_ids.len());

    warn!("Making Game");
    let game = Game::new(
        render_ids,
        render_system,
        render_event_game,
        control_event_game,
        ortho_helper
    );

    warn!("Starting Game Thread");
    let game_handle = thread::spawn(|| {
        let mut game = game;
        while game.frame() { }
    });

    warn!("Creating Event Pump");
    let mut event_pump = sdl.event_pump().unwrap_or_else(|err| panic!("Error while making event pump: {:?}", err));
    event_pump.enable_event(sdl2::event::EventType::Window);
    event_pump.enable_event(sdl2::event::EventType::Quit);

    warn!("Entering Main Loop");
    'main: loop {
        // warn!("Looping Main Loop");
        if let Some(event) = render_event_core.try_recv_from() {
            match event {//render_event_core.recv_from() {
                (window_id, FromRender::Encoder(mut encoder)) => {
                    // warn!("Handling Events");
                    if handle_events(&mut event_pump, &mut render_event_core, &mut control_event_core) {
                        break 'main;
                    }

                    let mut sdl_graphic = sdl_graphics.get_mut(&window_id).unwrap_or_else(|| panic!("Unable to find SdlGraphic for: {:?}", window_id));

                    // warn!("Making Context Current");
                    // sdl_graphic.get_window().gl_make_current(sdl_graphic.get_gl_context()).unwrap_or_else(|err| panic!("fuck off"));
                    sdl_graphic.get_window().gl_set_context_to_current().unwrap_or_else(|err| panic!("Error while making context current: {:?}", err));
                    // warn!("Flushing Encoder");
                    encoder.flush(sdl_graphic.get_mut_device());
                    // warn!("Sending Encoder Back");
                    render_event_core.send_to((window_id, ToRender::Encoder(encoder)));
                    // warn!("Swapping Windows");
                    sdl_graphic.get_mut_window().gl_swap_window();
                    // warn!("Cleanup");
                    sdl_graphic.get_mut_device().cleanup();
                },
            }
        }

        if let Some(event) = control_event_core.try_recv_from() {
            match event {
                (_, FromControl::Resize) => { //_ = window_id

                },
            }
        }
    }

    // game_handle.join().unwrap_or_else(|err| panic!("Error while joining game handle: {:?}", err));
}
