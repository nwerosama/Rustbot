use crate::controllers::database::DatabaseController;

pub struct SampleData {
  pub id: i64,
  pub text_val: String,
  pub int_val: i64,
  pub boolean_val: bool
}

impl SampleData {
  pub async fn list_data(id: u64) -> Result<Vec<Self>, tokio_postgres::Error> {
    let client = DatabaseController::new().await?.client;
    let rows = client.query("
      SELECT * FROM sample
      WHERE id = $1
    ", &[&(id as i64)]).await?;

    let mut data = Vec::new();
    for row in rows {
      data.push(Self {
        id: row.get("id"),
        text_val: row.get("text_val"),
        int_val: row.get("int_val"),
        boolean_val: row.get("boolean_val")
      });
    }

    Ok(data)
  }

  pub async fn create_data(
    text: String,
    int: i64,
    boolean: bool
  ) -> Result<(), tokio_postgres::Error> {
    let client = DatabaseController::new().await?.client;
    client.execute("
      INSERT INTO sample (text_val, int_val, boolean_val)
      VALUES ($1, $2, $3)
    ", &[&text, &int, &boolean]).await?;

    Ok(())
  }

  pub async fn update_data(
    id: u64,
    text: String,
    int: i64,
    boolean: bool
  ) -> Result<(), tokio_postgres::Error> {
    let client = DatabaseController::new().await?.client;
    client.execute("
      UPDATE sample
      SET text_val = $1, int_val = $2, boolean_val = $3
      WHERE id = $4
    ", &[&text, &int, &boolean, &(id as i64)]).await?;

    Ok(())
  }

  pub async fn delete_data(id: u64) -> Result<(), tokio_postgres::Error> {
    let client = DatabaseController::new().await?.client;
    client.execute("
      DELETE FROM sample
      WHERE id = $1
    ", &[&(id as i64)]).await?;

    Ok(())
  }
}
