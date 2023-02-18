use crate::{config::Config, database, routes};
use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: database::Database,
}

pub async fn build(config: &Config) -> Result<Router<()>, Box<dyn std::error::Error>> {
    let db = database::Database::connect(&config).await?;
    let state = AppState { db };

    let app = Router::new()
        .nest("/auth", routes::auth::build_router())
        .with_state(state);

    let app = {
        if config.log_requests {
            app.layer(TraceLayer::new_for_http())
        } else {
            app
        }
    };

    let app = {
        if config.cors_development {
            app.layer(CorsLayer::very_permissive())
        } else {
            app
        }
    };

    Ok(app)
}
