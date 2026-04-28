//! Example: Using MLX backend for inference
//!
//! This example demonstrates how to use PUMA with MLX backend on Apple Silicon.
//!
//! Build with: cargo build --release --features mlx
//! Run with: cargo run --release --features mlx --example mlx_inference

#[cfg(all(target_os = "macos", feature = "mlx"))]
use puma::backend::MlxEngine;

#[cfg(all(target_os = "macos", feature = "mlx"))]
use puma::backend::InferenceEngine;

#[cfg(all(target_os = "macos", feature = "mlx"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    println!("🐆 PUMA MLX Inference Example\n");

    // Create MLX engine
    println!("Initializing MLX engine...");
    let engine = MlxEngine::new()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    println!("✓ MLX engine initialized\n");

    // Generate text
    println!("Generating text...");
    let response = engine
        .generate(
            "test-model",
            "Once upon a time",
            50,   // max_tokens
            0.7,  // temperature
        )
        .await?;

    println!("\n📝 Generated Response:");
    println!("  Text: {}", response.text);
    println!("  Prompt tokens: {}", response.prompt_tokens);
    println!("  Completion tokens: {}", response.completion_tokens);

    // Test streaming
    println!("\n🔄 Testing streaming generation...");
    let mut stream = engine
        .generate_stream(
            "test-model",
            "The quick brown fox",
            30,
            0.7,
        )
        .await?;

    print!("  Tokens: ");
    use tokio_stream::StreamExt;
    while let Some(token) = stream.next().await {
        print!("{}", token);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }
    println!("\n\n✓ Example completed");

    Ok(())
}

#[cfg(not(all(target_os = "macos", feature = "mlx")))]
fn main() {
    eprintln!("❌ This example requires macOS with Apple Silicon");
    eprintln!("Build with: cargo run --features mlx --example mlx_inference");
    std::process::exit(1);
}
