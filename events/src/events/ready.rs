use crate::PoiseFwCtx;
use super::{
  EventProcessor,
  RUSTBOT_EVENT
};

use rustbot_lib::{
  RustbotError,
  utils::{
    BOT_VERSION,
    GIT_COMMIT_HASH,
    GIT_COMMIT_BRANCH
  },
  config::BINARY_PROPERTIES
};
use std::sync::atomic::{
  AtomicBool,
  Ordering
};
use poise::serenity_prelude::{
  Ready,
  ChannelId,
  CreateMessage,
  CreateEmbed,
  CreateEmbedAuthor
};

static READY_ONCE: AtomicBool = AtomicBool::new(false);

async fn ready_once(
  ready: &Ready,
  framework: PoiseFwCtx<'_>
) -> Result<(), RustbotError> {
  #[cfg(not(feature = "production"))]
  {
    println!("{RUSTBOT_EVENT}[Ready:Notice:S{}]: Detected a non-production environment!", framework.serenity_context.shard_id);
    let gateway = framework.serenity_context.http.get_bot_gateway().await?;
    let session = gateway.session_start_limit;
    println!("{RUSTBOT_EVENT}[Ready:Notice:S{}]: Session limit: {}/{}", framework.serenity_context.shard_id, session.remaining, session.total);
  }

  println!("{RUSTBOT_EVENT}[Ready:S{}]: Build version: {} ({}:{})", framework.serenity_context.shard_id, *BOT_VERSION, GIT_COMMIT_HASH, GIT_COMMIT_BRANCH);
  println!("{RUSTBOT_EVENT}[Ready:S{}]: Connected to API as {}", framework.serenity_context.shard_id, ready.user.name);

  let message = CreateMessage::new();
  let ready_embed = CreateEmbed::new()
    .color(BINARY_PROPERTIES.embed_color)
    .thumbnail(ready.user.avatar_url().unwrap_or_default())
    .author(CreateEmbedAuthor::new(format!("{} is ready!", ready.user.name)));

  ChannelId::new(BINARY_PROPERTIES.rustbot_logs).send_message(&framework.serenity_context.http, message.add_embed(ready_embed)).await?;

  Ok(())
}

impl EventProcessor<'_> {
  pub async fn on_ready(
    &self,
    data_about_bot: &Ready
  ) -> Result<(), RustbotError> {
    if !READY_ONCE.swap(true, Ordering::Relaxed) {
      ready_once(data_about_bot, self.framework).await.expect("Failed to call ready_once method");
    }

    Ok(())
  }
}
