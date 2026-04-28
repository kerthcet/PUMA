pub mod engine;
pub mod mock;

pub use engine::*;
pub use mock::MockEngine;

#[cfg(all(target_os = "macos", feature = "mlx"))]
pub mod mlx;

#[cfg(all(target_os = "macos", feature = "mlx"))]
pub use mlx::MlxEngine;
