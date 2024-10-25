use crate::RustbotError;
use super::PoiseContext;

use poise::{
  CreateReply,
  serenity_prelude::ChannelId
};

/// Developer commands
#[poise::command(
  slash_command,
  prefix_command,
  owners_only,
  subcommands("deploy", "servers", "shards", "echo")
)]
pub async fn dev(_: PoiseContext<'_>) -> Result<(), RustbotError> {
  Ok(())
}

/// Deploy commands to this guild or globally
#[poise::command(prefix_command)]
async fn deploy(ctx: PoiseContext<'_>) -> Result<(), RustbotError> {
  poise::builtins::register_application_commands_buttons(ctx).await?;
  Ok(())
}

/// View how many servers the bot is in
#[poise::command(prefix_command)]
async fn servers(ctx: PoiseContext<'_>) -> Result<(), RustbotError> {
  poise::builtins::servers(ctx).await?;
  Ok(())
}

/// View the status of available shards
#[poise::command(prefix_command)]
async fn shards(ctx: PoiseContext<'_>) -> Result<(), RustbotError> {
  let shard_runners = ctx.framework().shard_manager().runners.clone();
  let runners = shard_runners.lock().await;

  let mut shard_info = Vec::new();
  for (id, runner) in runners.iter() {
    shard_info.push(format!(
      "**Shard {}**\n> Heartbeat: {}\n> Status: `{}`",
      id,
      match runner.latency {
        Some(lat) => format!("`{}ms`", lat.as_millis()),
        None => "Waiting for heartbeat...".to_string()
      },
      runner.stage
    ))
  }

  ctx.reply(shard_info.join("\n\n")).await?;

  Ok(())
}

/// Turn your message into a bot message
#[poise::command(slash_command)]
async fn echo(
  ctx: super::PoiseContext<'_>,
  #[description = "Message to be echoed as a bot"] message: String,
  #[description = "Channel to send this to"]
  #[channel_types("Text", "PublicThread", "PrivateThread")] channel: Option<ChannelId>
) -> Result<(), RustbotError> {
  ctx.defer_ephemeral().await?;

  let channel = match channel {
    Some(c) => c,
    None => ctx.channel_id()
  };

  match ChannelId::new(channel.get()).say(ctx.http(), message).await {
    Ok(_) => {
      ctx.send(
        CreateReply::new()
          .content("Sent!")
          .ephemeral(true)
      ).await?;
    },
    Err(y) => {
      ctx.send(
        CreateReply::new()
          .content(format!("Failed... `{y}`"))
          .ephemeral(true)
      ).await?;
      return Ok(());
    }
  }

  Ok(())
}
