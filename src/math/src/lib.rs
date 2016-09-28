extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate utils;

pub mod ortho_helper;

pub mod point2;
pub mod point2i;
pub mod point3i;
pub mod rect;
pub mod line_segment;

pub use self::ortho_helper::OrthographicHelper;

pub use self::point2::Point2;
pub use self::point2i::Point2I;
pub use self::point3i::Point3I;
pub use self::rect::Rect;
pub use self::line_segment::LineSeg;
