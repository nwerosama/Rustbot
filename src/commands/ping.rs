use serde::Deserialize;
use rustbot_lib::{
  RustbotContext,
  RustbotResult
};

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

/// Check latency between bot and WebSocket as well as Discord's API latency
#[poise::command(
  slash_command,
  install_context = "Guild|User",
  interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn ping(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  let statuspage: StatusPage = reqwest::get("https://discordstatus.com/metrics-display/5k2rt9f7pmny/day.json")
  .await.unwrap()
  .json()
  .await.unwrap();

  let mut latencies = Vec::new();
  latencies.push(format!("Discord: `{:.0?}ms`", statuspage.metrics[0].summary.mean));
  latencies.push(format!("WebSocket: `{:.0?}`", ctx.ping().await));
  latencies.push(format!("Shard ID: `{}`", ctx.serenity_context().shard_id));

  ctx.reply(latencies.join("\n")).await?;

  Ok(())
}
