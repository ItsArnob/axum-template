use dotenv;
use std::{env, net::SocketAddr};

pub struct Config {
    pub mongodb_uri: String,
    pub db_name: String,
    pub socket_address: SocketAddr,
    pub log_requests: bool,
    pub cors_development: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {
        if args.len() >= 2 {
            dotenv::from_path(&args[1])?;
        } else {
            if dotenv::dotenv().is_err() {
                tracing::debug!("No .env file found. using environment variables from shell");
            }
        };

        let host = env::var("HOST").unwrap_or_else(|_| {
            let host = "127.0.0.1";
            tracing::debug!("HOST variable is not set. using default: {host}");
            host.into()
        });
        let port = env::var("PORT").unwrap_or_else(|_| {
            let port = "5000";
            tracing::debug!("PORT variable is not set. using default: {port}");
            port.into()
        });

        let db_name = env::var("DB_NAME").unwrap_or_else(|_| {
            let db_name = "name";
            tracing::debug!("DB_NAME variable is not set. using default: {db_name}");
            db_name.into()
        });

        let mongodb_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
            let mongodb_uri = "mongodb://127.0.0.1:27017";
            tracing::debug!("MONGODB_URI variable is not set. using default: {mongodb_uri}");
            mongodb_uri.into()
        });

        let socket_address: SocketAddr = format!("{host}:{port}").parse()?;

        let config = Config {
            mongodb_uri,
            db_name,
            socket_address,
            log_requests: env::var("LOG_REQUESTS").is_ok(),
            cors_development: env::var("CORS_DEVELOPMENT").is_ok(),
        };

        if config.log_requests {
            tracing::info!("Enabled request logging.")
        }

        if config.cors_development {
            tracing::info!("Enabled \"very_permissive\" cors settings.")
        }

        Ok(config)
    }
}
