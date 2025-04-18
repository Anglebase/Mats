mod mat;
mod math;
mod types;
pub mod dynamic;

pub use mat::Mat;
pub use math::*;
pub use types::*;

/// If the feature `glsl` is enabled, this module will be introduced to support the interface with GLSL.
#[cfg(feature = "glsl")]
pub mod uniform;
#[cfg(feature = "glsl")]
pub use uniform::*;

mod generated;