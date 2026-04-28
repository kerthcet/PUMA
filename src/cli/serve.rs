use colored::Colorize;
use std::sync::Arc;
use tracing::{debug, info};

use crate::api::routes::create_router;
use crate::backend::mock::MockEngine;
use crate::registry::model_registry::ModelRegistry;

#[cfg(all(target_os = "macos", feature = "mlx"))]
use crate::backend::mlx::MlxEngine;

/// Inference engine enum to support multiple backends
#[derive(Clone)]
pub enum Engine {
    Mock(MockEngine),
    #[cfg(all(target_os = "macos", feature = "mlx"))]
    Mlx(MlxEngine),
}

impl crate::backend::engine::InferenceEngine for Engine {
    async fn generate(
        &self,
        model: &str,
        prompt: &str,
        max_tokens: usize,
        temperature: f32,
    ) -> Result<crate::backend::engine::GenerateResponse, std::io::Error> {
        match self {
            Engine::Mock(engine) => engine.generate(model, prompt, max_tokens, temperature).await,
            #[cfg(all(target_os = "macos", feature = "mlx"))]
            Engine::Mlx(engine) => engine.generate(model, prompt, max_tokens, temperature).await,
        }
    }

    async fn generate_stream(
        &self,
        model: &str,
        prompt: &str,
        max_tokens: usize,
        temperature: f32,
    ) -> Result<std::pin::Pin<Box<dyn tokio_stream::Stream<Item = String> + Send>>, std::io::Error>
    {
        match self {
            Engine::Mock(engine) => {
                engine
                    .generate_stream(model, prompt, max_tokens, temperature)
                    .await
            }
            #[cfg(all(target_os = "macos", feature = "mlx"))]
            Engine::Mlx(engine) => {
                engine
                    .generate_stream(model, prompt, max_tokens, temperature)
                    .await
            }
        }
    }
}

/// Initialize the inference engine based on available features and platform
fn initialize_engine() -> Arc<Engine> {
    #[cfg(all(target_os = "macos", not(feature = "mlx")))]
    debug!("Build with --features mlx on Apple Silicon for MLX support");

    #[cfg(all(target_os = "macos", feature = "mlx"))]
    {
        info!("Initializing MLX inference engine");
        match MlxEngine::new() {
            Ok(mlx) => {
                info!("Inference engine initialized: MLX");
                return Arc::new(Engine::Mlx(mlx));
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to initialize MLX engine: {}, falling back to MockEngine",
                    e
                );
            }
        }
    }

    // Fallback to MockEngine
    info!("Inference engine initialized: MockEngine");
    Arc::new(Engine::Mock(MockEngine::new()))
}

/// Execute the serve command
pub async fn execute(
    host: &str,
    port: u16,
    model_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        "
 ███████████  █████  █████ ██████   ██████   █████████
░░███░░░░░███░░███  ░░███ ░░██████ ██████   ███░░░░░███
 ░███    ░███ ░███   ░███  ░███░█████░███  ░███    ░███
 ░██████████  ░███   ░███  ░███░░███ ░███  ░███████████
 ░███░░░░░░   ░███   ░███  ░███ ░░░  ░███  ░███░░░░░███
 ░███         ░███   ░███  ░███      ░███  ░███    ░███
 █████        ░░████████   █████     █████ █████   █████
░░░░░          ░░░░░░░░   ░░░░░     ░░░░░ ░░░░░   ░░░░░
                                                        "
        .bright_blue()
        .bold()
    );
    info!("Starting PUMA to serve model: {}", model_name);

    // Initialize inference engine
    let engine = initialize_engine();

    // Initialize model registry
    let registry = Arc::new(ModelRegistry::new(None));
    info!("Model registry loaded");

    // Create router
    let app = create_router(engine, registry);

    // Bind address
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Server listening on http://{}", addr);
    info!("Available endpoints:");
    info!("  POST /v1/chat/completions");
    info!("  POST /v1/completions");
    info!("  GET  /v1/models");
    info!("  GET  /v1/models/:model");
    info!("  GET  /health");

    // Start server
    debug!("Starting axum server");
    axum::serve(listener, app).await?;

    info!("Server shutdown");
    Ok(())
}
