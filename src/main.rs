use std::{env, process};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod config;
mod database;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    // TODO: change `axum_template` to a specific name based on cargo.toml.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("axum_template=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting server");

    let args: Vec<String> = env::args().collect();
    let config = config::Config::build(&args).unwrap_or_else(|err| {
        tracing::error!("Failed to build config: {}", err.to_string());
        process::exit(1)
    });

    let app = app::build(&config).await.unwrap_or_else(|err| {
        tracing::error!("Failed to build app: {}", err.to_string());
        process::exit(1)
    });

    tracing::info!("Listening on {}", &config.socket_address);
    axum::Server::bind(&config.socket_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
