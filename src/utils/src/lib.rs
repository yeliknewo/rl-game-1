#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
extern crate find_folder;

pub use find_folder::Search;

pub mod fps_counter;

pub use fps_counter::FpsCounter;

pub type Delta = f64;
pub type Coord = f64;
pub type CoordI = i64;
pub type GfxCoord = f32;

pub use time::{precise_time_ns};

#[derive(Debug, Clone)]
pub enum WindowId {
    First,
    Second,
}
