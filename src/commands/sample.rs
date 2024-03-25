use crate::{
  Error,
  models::sample::SampleData
};

use poise::CreateReply;

/// Perform sample CRUD operations in database
#[poise::command(
  slash_command,
  subcommands("list", "create", "update", "delete"),
  subcommand_required
)]
pub async fn sample(_: poise::Context<'_, (), Error>) -> Result<(), Error> {
  Ok(())
}

/// List sample data
#[poise::command(slash_command)]
pub async fn list(
  ctx: poise::Context<'_, (), Error>,
  #[description = "ID of the sample data"] id: u64
) -> Result<(), Error> {
  let samples = SampleData::list_data(id).await?;

  let mut response = String::new();
  for sample in samples {
    response.push_str(&format!("ID: {}\n", sample.id));
    response.push_str(&format!("Text: {}\n", sample.text_val));
    response.push_str(&format!("Int: {}\n", sample.int_val));
    response.push_str(&format!("Boolean: {}\n\n", sample.boolean_val));
  }

  ctx.send(CreateReply::default()
    .content(response)
  ).await?;

  Ok(())
}

/// Create sample data
#[poise::command(slash_command)]
pub async fn create(
  ctx: poise::Context<'_, (), Error>,
  #[description = "Text value"] text: String,
  #[description = "Int value"] int: u64,
  #[description = "Boolean value"] boolean: bool
) -> Result<(), Error> {
  SampleData::create_data(text, int as i64, boolean).await?;

  ctx.send(CreateReply::default().content("Done!")).await?;

  Ok(())
}

/// Update sample data
#[poise::command(slash_command)]
pub async fn update(
  ctx: poise::Context<'_, (), Error>,
  #[description = "ID of the sample data"] id: u64,
  #[description = "Text value"] text: String,
  #[description = "Int value"] int: u64,
  #[description = "Boolean value"] boolean: bool
) -> Result<(), Error> {
  SampleData::update_data(id, text, int as i64, boolean).await?;

  ctx.send(CreateReply::default().content("Done!")).await?;

  Ok(())
}

/// Delete sample data
#[poise::command(slash_command)]
pub async fn delete(
  ctx: poise::Context<'_, (), Error>,
  #[description = "ID of the sample data"] id: u64
) -> Result<(), Error> {
  SampleData::delete_data(id).await?;

  ctx.send(CreateReply::default().content("Done!")).await?;

  Ok(())
}
