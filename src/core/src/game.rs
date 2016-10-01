use art::{layers, tiles};
use components::{Camera, RenderData, RenderId, Transform};
use event::{BackChannel};
use math::{OrthographicHelper, Point3, Vector3};
use systems::render::{RenderSystem, RenderSystemSend, WindowedToRender, WindowedFromRender};
use systems::control::{ControlSystem, WindowedToControl, WindowedFromControl};
use systems::{World, Planner};
use utils::{Delta, FpsCounter, precise_time_ns};

pub struct Game {
    planner: Planner<Delta>,
    last_time: u64,
    fps_counter: FpsCounter,
}

impl Game {
    pub fn new(
        render_ids_1: Vec<RenderId>,
        render_ids_2: Vec<RenderId>,
        render_system_send: RenderSystemSend,
        render_back_channel: BackChannel<WindowedToRender, WindowedFromRender>,
        control_back_channel: BackChannel<WindowedToControl, WindowedFromControl>,
        ortho_helper: OrthographicHelper,
    ) -> Game {
        warn!("Starting New Game");
        let mut planner = {
            let mut world = World::new();

            world.register::<Camera>();
            world.register::<RenderData>();
            world.register::<RenderId>();
            world.register::<Transform>();

            Planner::<Delta>::new(world, 8)
        };

        warn!("Creating Render System");
        let renderer = RenderSystem::new(render_back_channel, render_system_send);

        warn!("Creating Camera");
        planner.mut_world().create_now()
            .with(Camera::new(
                Point3::new(0.0, 0.0, 2.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                ortho_helper,
                true
            ))
            .build();

        planner.mut_world().create_now()
            .with(render_ids_1[0])
            .with(Transform::new_identity())
            .with(RenderData::new(layers::TILES, *tiles::DEFAULT_TINT, tiles::EMPTY, tiles::SIZE))
            .build();

        planner.mut_world().create_now()
            .with(render_ids_2[0])
            .with(Transform::new_identity())
            .with(RenderData::new(layers::TILES, *tiles::DEFAULT_TINT, tiles::EMPTY, tiles::SIZE))
            .build();

        warn!("Adding Control System");
        planner.add_system(
            ControlSystem::new(control_back_channel),
            "control",
            30
        );

        warn!("Adding Render System");
        planner.add_system(
            renderer,
            "renderer",
            10
        );

        warn!("Creating Game Struct");
        Game {
            planner: planner,
            last_time: precise_time_ns(),
            fps_counter: FpsCounter::new(),
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = precise_time_ns();
        let delta = (new_time - self.last_time) as Delta / 1e9;
        self.last_time = new_time;

        self.planner.dispatch(delta);
        self.fps_counter.frame(delta);

        true
    }
}
