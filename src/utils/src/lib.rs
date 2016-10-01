#[macro_use]
extern crate log;

pub mod fps_counter;

pub use fps_counter::FpsCounter;

pub type Delta = f64;
pub type Coord = f64;
pub type CoordI = i64;
pub type GfxCoord = f32;
