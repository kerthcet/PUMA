//! MLX backend for Apple Silicon inference
//!
//! This module provides inference using Apple's MLX framework via mlx-rs.
//! Only available on macOS with Apple Silicon when built with --features mlx.

mod engine;

pub use engine::MlxEngine;
