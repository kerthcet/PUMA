# MLX Backend

This module provides inference support using Apple's MLX framework via [mlx-rs](https://github.com/oxiglade/mlx-rs) bindings.

## Requirements

- macOS 13.0 or later
- Apple Silicon (M1/M2/M3/M4)
- Xcode Command Line Tools

## Installation

### 1. Install Xcode Command Line Tools

```bash
xcode-select --install
```

### 2. Build PUMA with MLX support

```bash
# Enable the mlx feature
cargo build --release --features mlx

# Or install
cargo install --path . --features mlx
```

The mlx-rs library will automatically download and compile MLX during the build process.

## Usage

```rust
use puma::backend::MlxEngine;

// Create MLX engine
let engine = MlxEngine::new()?;

// Generate text
let response = engine.generate(
    "model-name",
    "Hello, world!",
    100,  // max_tokens
    0.7   // temperature
).await?;
```

## Architecture

```
mlx/
├── mod.rs       # Module exports and feature gating
├── engine.rs    # Main inference engine implementation
└── README.md    # This file
```

## Implementation Status

### Phase 1: mlx-rs Integration (Current)
- ✅ Module structure
- ✅ Feature gating (macOS + Apple Silicon only)
- ✅ mlx-rs dependency integration (v0.25.3)
- ✅ Basic engine trait implementation
- ✅ Device initialization (GPU/CPU)
- ⚠️  Placeholder tokenization/generation (functional but not production-ready)

### Phase 2: TODO
- [ ] Model loading from PUMA cache
- [ ] Proper tokenizer integration (tokenizers-rs)
- [ ] Actual token generation with mlx-rs
- [ ] Streaming support
- [ ] Model caching and memory management
- [ ] Multi-model support
- [ ] Quantization support (4-bit, 8-bit)

### Phase 3: Custom Implementation (Future)
- [ ] Replace mlx-rs with custom mlx-sys FFI layer
- [ ] Build custom safe Rust wrapper
- [ ] Fine-grained control over MLX operations

## Architecture

```
Current:
PUMA → MlxEngine → mlx-rs (v0.25.3) → mlx-sys → MLX C++ library

Future:
PUMA → MlxEngine → puma-mlx (custom) → puma-mlx-sys → MLX C++ library
```

## References

- [MLX GitHub](https://github.com/ml-explore/mlx)
- [mlx-rs bindings](https://github.com/oxiglade/mlx-rs) (currently used)
- [mlx-rs documentation](https://oxideai.github.io/mlx-rs/mlx_rs/)
- [MLX Python API](https://ml-explore.github.io/mlx/build/html/index.html)
