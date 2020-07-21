pub mod gripper;
pub use crate::gripper::{incremental_grip, compositional_grip_g1, compositional_grip_g2, compositional_grip_g3};

pub mod movie;
pub use crate::movie::{incremental_movie, compositional_movie};