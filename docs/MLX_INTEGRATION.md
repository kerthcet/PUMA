# MLX Integration Guide

## Overview

PUMA now supports Apple's MLX framework for high-performance inference on Apple Silicon. This integration uses [mlx-rs](https://github.com/oxiglade/mlx-rs) v0.25.3 as the binding layer.

## Quick Start

### Prerequisites

- macOS 13.0+
- Apple Silicon (M1/M2/M3/M4)
- Xcode Command Line Tools: `xcode-select --install`

### Building with MLX Support

```bash
# Build
cargo build --release --features mlx

# Run tests (on Apple Silicon)
cargo test --features mlx

# Run example
cargo run --release --features mlx --example mlx_inference
```

### Starting Server with MLX

```bash
# Build with MLX feature
cargo build --release --features mlx

# Start server (will use MLX on Apple Silicon)
./target/release/puma serve
```

The server will automatically detect and use MLX if:
1. Running on Apple Silicon macOS
2. Built with `--features mlx`

Otherwise, it falls back to MockEngine.

## Architecture

```
┌─────────────────────────────────────────┐
│           PUMA CLI/API                  │
└──────────────┬──────────────────────────┘
               │
       ┌───────┴────────┐
       │ InferenceEngine│  (trait)
       └───────┬────────┘
               │
    ┏━━━━━━━━━┻━━━━━━━━━┓
    ┃                    ┃
┌───┴─────┐      ┌───────┴─────┐
│MockEngine│      │  MlxEngine  │ 
└─────────┘      └───────┬──────┘
                         │
                  ┌──────┴──────┐
                  │   mlx-rs    │ (v0.25.3)
                  └──────┬──────┘
                         │
                  ┌──────┴──────┐
                  │   mlx-sys   │ (FFI)
                  └──────┬──────┘
                         │
                  ┌──────┴──────┐
                  │  MLX C++    │
                  └─────────────┘
```

## Current Implementation Status

### ✅ Completed

- Feature flag setup (`mlx` feature)
- Platform detection (Apple Silicon only)
- mlx-rs integration (v0.25.3)
- Device initialization (GPU/CPU)
- Basic InferenceEngine trait implementation
- Conditional compilation for non-macOS platforms
- Serve command auto-detection

### 🚧 In Progress (Placeholder)

- Model loading from PUMA cache
- Tokenization (currently uses dummy tokens)
- Token generation (placeholder implementation)
- Streaming (basic structure in place)

### 📋 TODO

- [ ] Integrate proper tokenizer (tokenizers-rs or mlx-lm)
- [ ] Implement actual model loading
- [ ] Real token generation with mlx-rs
- [ ] Model caching and memory management
- [ ] Quantization support (4-bit, 8-bit)
- [ ] Benchmark and optimize performance
- [ ] Add more examples

## Usage Example

```rust
use puma::backend::{MlxEngine, InferenceEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create engine
    let engine = MlxEngine::new()?;

    // Generate text
    let response = engine.generate(
        "model-name",
        "Hello, world!",
        100,  // max_tokens
        0.7   // temperature
    ).await?;

    println!("Generated: {}", response.text);
    Ok(())
}
```

## API Endpoints

Once MLX is enabled, the API server uses it automatically:

```bash
# Chat completion with MLX
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "your-model",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## Performance Notes

- MLX uses unified memory (CPU/GPU share memory)
- Metal GPU acceleration enabled by default
- Lazy evaluation for compute graphs
- Multi-device support (CPU/GPU)

## Troubleshooting

### Build fails on non-macOS

**Expected behavior.** MLX only works on macOS with Apple Silicon.

### "MLX feature not enabled" error at runtime

Build with the `mlx` feature:
```bash
cargo build --features mlx
```

### Server uses MockEngine instead of MLX

Check:
1. Running on Apple Silicon? `uname -m` should show `arm64`
2. Built with MLX feature? `cargo build --features mlx`
3. Check logs for MLX initialization

### Compilation takes a long time

MLX compilation (via mlx-rs) compiles the entire MLX framework. First build can take 10-20 minutes. Subsequent builds are cached.

## Future: Custom MLX Bindings

Current implementation uses mlx-rs. Future plan:

1. **Phase 1** (Current): Use mlx-rs for rapid development
2. **Phase 2**: Implement custom `puma-mlx-sys` FFI layer
3. **Phase 3**: Build custom safe wrapper with PUMA-specific optimizations

See `src/backend/mlx/README.md` for details.

## References

- [MLX Framework](https://github.com/ml-explore/mlx)
- [mlx-rs Bindings](https://github.com/oxiglade/mlx-rs)
- [mlx-rs Documentation](https://oxideai.github.io/mlx-rs/mlx_rs/)
- [MLX Python Docs](https://ml-explore.github.io/mlx/build/html/index.html)

## Contributing

When contributing to MLX backend:

1. Test on Apple Silicon macOS
2. Ensure code compiles without `mlx` feature
3. Use feature flags for MLX-specific code
4. Add tests with `#[cfg(all(target_os = "macos", feature = "mlx"))]`
5. Document any new MLX-specific features

## License

MLX integration in PUMA follows the same Apache-2.0 license as the main project.
