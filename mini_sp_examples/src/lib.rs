pub mod gripper;
pub use crate::gripper::{incremental_grip, compositional_grip};

pub mod gripper_orig;
pub use crate::gripper_orig::{incremental_grip_orig, compositional_grip_orig};