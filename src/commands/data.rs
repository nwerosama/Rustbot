use crate::Error;

use reqwest::get;
use std::collections::HashMap;
use serde_json::Value;
use poise::CreateReply;
use serenity::builder::{
  CreateEmbed,
  CreateEmbedAuthor,
  CreateEmbedFooter
};

/// Retrieve the data from FS-Server
#[poise::command(slash_command)]
pub async fn data(
  ctx: poise::Context<'_, (), Error>,
  #[description = "DSS url to extract"] url: String
) -> Result<(), Error> {
  // Find .xml and replace it with .json before passing to reqwest.
  let url = url.replace(".xml", ".json");

  // Send a GET request to the provided URL
  let response = get(&url).await?.json::<HashMap<String, Value>>().await?;

  // Extract the required values from the parsed JSON
  let server = &response["server"];
  let slots = &response["slots"];

  // Variable list of JSON values
  let name = server["name"].as_str().unwrap();
  let ver = server["version"].as_str().unwrap();
  let map = server["mapName"].as_str().unwrap();
  let slot_cap = slots["capacity"].as_i64().unwrap();
  let slot_cur = slots["used"].as_i64().unwrap();
  let daytime = server["dayTime"].as_i64().unwrap();

  // todo: Add careerSavegame support when passing in DSS url.
  // So I can get the following values for Autosave, Timescale and Slot usage.

  // Convert dayTime (ms) to a military time format
  let hour = (daytime / 3600 / 1000) % 24;
  let minute = (daytime / 60 / 1000) % 60;
  let time = format!("{:02}:{:02}", hour, minute);

  let embed = CreateEmbed::new().color(crate::EMBED_COLOR);
  ctx.send(CreateReply::default()
    .embed(embed
      .title(name)
      .description("*Nobody is playing*")
      .fields(vec![
        ("Map", map, true),
        ("Version", ver, true),
        ("Time", &time, true),
        ("Slot usage", "xx/xx", true),
        ("Autosave", "xx", true),
        ("Timescale", "0x", true)
      ])
      .author(CreateEmbedAuthor::new(format!("{}/{}", slot_cur, slot_cap)).clone())
      .footer(CreateEmbedFooter::new("Last updated").clone())
      .timestamp(poise::serenity_prelude::Timestamp::now())
    )
  ).await?;

  Ok(())
}
