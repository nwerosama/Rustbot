use crate::Error;

use reqwest::get;
use std::collections::HashMap;
use serde_json::Value;

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
  // println!("{:?}", response);

  // Extract the required values from the parsed JSON
  let server = &response["server"];
  let slots = &response["slots"];

  // Variable list of JSON values
  let name = server["name"].as_str().unwrap();
  let ver = server["version"].as_str().unwrap();
  let map = server["mapName"].as_str().unwrap();
  let slot_cap = slots["capacity"].as_i64().unwrap();
  let slot_cur = slots["used"].as_i64().unwrap();

  ctx.send(|m| {
    m.embed(|e| {
      e.color(crate::COLOR)
        .author(|a| {
          a.name(format!("{}/{}", slot_cur, slot_cap))
        })
        .title(name)
        .description("*Nobody is playing*")
        .fields(vec![
          ("Map", map, true),
          ("Version", ver, true),
          ("Time", "xx:xx", true),
          ("Slot usage", "xx/xx", true),
          ("Autosave", "xx", true),
          ("Timescale", "0x", true)
        ])
        .footer(|f| {
          f.text("Last updated")
        })
        .timestamp(poise::serenity_prelude::Timestamp::now())
    })
  }).await?;

  Ok(())
}
