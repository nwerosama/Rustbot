use {
  poise::{
    CreateReply,
    serenity_prelude::ChannelId
  },
  rustbot_lib::{
    RustbotContext,
    RustbotResult
  }
};

/* async fn format_shard_info(
  id: &ShardId,
  runner: &ShardRunnerInfo,
  ctx: &RustbotContext<'_>
) -> String {
  let mut string = String::new();

  let heartbeat = match runner.latency {
    Some(lat) => format!("`{}ms`", lat.as_millis()),
    None => "Waiting for heartbeat...".to_string()
  };

  let status = runner.stage.to_string();
  let shard_count = ctx.cache().shard_count();
  let guild_count = ctx.cache().guilds().into_iter().filter(|g| g.shard_id(shard_count) == id.0).count() as u64;

  string.push_str(&format!("**Shard {id}**\n"));
  string.push_str(&format!("> Heartbeat: {heartbeat}\n"));
  string.push_str(&format!("> Status: `{status}`\n"));
  string.push_str(&format!("> Guilds: **{guild_count}**"));

  string
} */

/// Developer commands
#[poise::command(
  prefix_command,
  slash_command,
  owners_only,
  install_context = "Guild|User",
  interaction_context = "Guild|BotDm|PrivateChannel",
  subcommands("deploy", "servers", "echo")
)]
pub async fn dev(_: RustbotContext<'_>) -> RustbotResult<()> { Ok(()) }

/// Deploy commands to this guild or globally
#[poise::command(prefix_command)]
async fn deploy(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  poise::builtins::register_application_commands_buttons(ctx).await?;
  Ok(())
}

/// View how many servers the bot is in
#[poise::command(slash_command)]
async fn servers(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  poise::builtins::servers(ctx).await?;
  Ok(())
}

// Re-enable when proper implementation takes place, since
// it got removed during Serenity's gateway refactor.
// -----
/* /// View the status of available shards
#[poise::command(slash_command)]
async fn shards(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  let shard_runners = ctx.framework().shard_manager().runners.clone();
  let runners = shard_runners.lock().await;

  if runners.is_empty() {
    ctx.reply("`ShardsReady` event hasn't fired yet!").await?;
    return Ok(())
  }

  let mut shard_info = Vec::new();
  for (id, runner) in runners.iter() {
    let info = format_shard_info(id, runner, &ctx).await;
    shard_info.push(info);
  }

  ctx.reply(shard_info.join("\n\n")).await?;

  Ok(())
} */

/// Turn your message into a bot message
#[poise::command(slash_command)]
async fn echo(
  ctx: RustbotContext<'_>,
  #[description = "Message to be echoed as a bot"] message: String,
  #[description = "Channel to send this to"]
  #[channel_types("Text", "PublicThread", "PrivateThread")]
  channel: Option<ChannelId>
) -> RustbotResult<()> {
  ctx.defer_ephemeral().await?;

  let channel = match channel {
    Some(c) => c,
    None => ctx.channel_id()
  };

  match ChannelId::new(channel.get()).say(ctx.http(), message).await {
    Ok(_) => {
      ctx.send(CreateReply::new().content("Sent!").ephemeral(true)).await?;
    },
    Err(y) => {
      ctx.send(CreateReply::new().content(format!("Failed... `{y}`")).ephemeral(true)).await?;
      return Ok(());
    }
  }

  Ok(())
}
