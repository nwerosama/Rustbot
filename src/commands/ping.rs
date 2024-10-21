use crate::RustbotError;
use super::PoiseContext;

use serde::Deserialize;

#[derive(Deserialize)]
struct StatusPage {
  metrics: Vec<Metrics>
}

#[derive(Deserialize)]
struct Metrics {
  summary: Summary
}

#[derive(Deserialize)]
struct Summary {
  mean: f64
}

/// Check latency of the bot's WS connection and Discord's API
#[poise::command(slash_command)]
pub async fn ping(ctx: PoiseContext<'_>) -> Result<(), RustbotError> {
  let statuspage: StatusPage = reqwest::get("https://discordstatus.com/metrics-display/5k2rt9f7pmny/day.json")
    .await.unwrap()
    .json()
    .await.unwrap();

  let mut latencies = Vec::new();
  latencies.push(format!("Discord: `{:.0?}ms`", statuspage.metrics[0].summary.mean));
  latencies.push(format!("WebSocket: `{:.0?}`", ctx.ping().await));

  ctx.reply(latencies.join("\n")).await?;

  Ok(())
}
