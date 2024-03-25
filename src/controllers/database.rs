use poise::serenity_prelude::prelude::TypeMapKey;
use tokio_postgres::{Client, NoTls, Error};

pub struct DatabaseController {
  pub client: Client
}

impl TypeMapKey for DatabaseController {
  type Value = DatabaseController;
}

impl DatabaseController {
  pub async fn new() -> Result<DatabaseController, Error> {
    let db_uri = std::env::var("DATABASE_URI").expect("Expected a \"DATABASE_URI\" in the envvar but none was found");
    let (client, connection) = tokio_postgres::connect(&db_uri, NoTls).await?;

    tokio::spawn(async move {
      if let Err(e) = connection.await {
        eprintln!("Connection error: {}", e);
      }
    });

    // Sample model
    client.batch_execute("
      CREATE TABLE IF NOT EXISTS sample (
        id BIGSERIAL PRIMARY KEY,
        text_val VARCHAR(255) NOT NULL,
        int_val BIGINT NOT NULL,
        boolean_val BOOLEAN NOT NULL
      );
    ").await?;

    Ok(DatabaseController { client })
  }
}
