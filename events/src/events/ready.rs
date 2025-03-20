use super::RUSTBOT_EVENT;

use {
  poise::serenity_prelude::{
    ChannelId,
    Context,
    CreateEmbed,
    CreateEmbedAuthor,
    CreateMessage,
    Ready
  },
  rustbot_lib::{
    RustbotResult,
    config::BINARY_PROPERTIES,
    utils::{
      BOT_VERSION,
      GIT_COMMIT_BRANCH,
      GIT_COMMIT_HASH
    }
  }
};

async fn ready_once(
  ctx: &Context,
  ready: &Ready
) -> RustbotResult<()> {
  let sid = ctx.shard_id;

  #[cfg(not(feature = "production"))]
  {
    println!("{RUSTBOT_EVENT}[Ready:Notice:S{sid}]: Detected a development environment!",);
    let gateway = ctx.http.get_bot_gateway().await?;
    let session = gateway.session_start_limit;
    println!(
      "{RUSTBOT_EVENT}[Ready:Notice:S{sid}]: Session limit: {}/{}",
      session.remaining, session.total
    );
  }

  println!(
    "{RUSTBOT_EVENT}[Ready:S{sid}]: Build version: v{} ({GIT_COMMIT_HASH}:{GIT_COMMIT_BRANCH})",
    *BOT_VERSION
  );
  println!("{RUSTBOT_EVENT}[Ready:S{sid}]: Connected to API as {}", ready.user.name);

  let message = CreateMessage::new();
  let ready_embed = CreateEmbed::new()
    .color(BINARY_PROPERTIES.embed_color)
    .thumbnail(ready.user.avatar_url().unwrap_or_default())
    .author(CreateEmbedAuthor::new(format!("{} is ready!", ready.user.name)));

  ChannelId::new(BINARY_PROPERTIES.rustbot_logs)
    .send_message(&ctx.http, message.add_embed(ready_embed))
    .await?;

  Ok(())
}

pub async fn on_ready(
  ctx: Context,
  ready: Ready
) -> RustbotResult<()> {
  let shard_id = ctx.shard_id;

  if shard_id.0 == 0 {
    ready_once(&ctx, &ready).await.expect("Failed to call ready_once method");
  }

  Ok(())
}
