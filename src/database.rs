use crate::{config::Config, models::User};
use mongodb::{bson::doc, options::ClientOptions, Client, Collection, Database as MongoDatabase};

#[derive(Clone)]
pub struct Database {
    pub client: Client,
    pub db: MongoDatabase,
    pub users: Collection<User>,
}

impl Database {
    pub async fn connect(config: &Config) -> Result<Database, Box<dyn std::error::Error>> {
        let client_options = ClientOptions::parse(&config.mongodb_uri).await?;

        let client = Client::with_options(client_options)?;
        let db = client.database(&config.db_name);

        client
            .database("admin")
            .run_command(doc! { "ping": 1 }, None)
            .await?;

        tracing::info!("connected to mongodb");
        Ok(Database {
            client,
            users: db.collection("users"),
            db,
        })
    }
}
