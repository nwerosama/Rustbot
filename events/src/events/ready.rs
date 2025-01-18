use super::{
  EventProcessor,
  RUSTBOT_EVENT
};

use {
  poise::serenity_prelude::{
    ChannelId,
    CreateEmbed,
    CreateEmbedAuthor,
    CreateMessage,
    Ready
  },
  rustbot_lib::{
    RustbotFwCtx,
    RustbotResult,
    config::BINARY_PROPERTIES,
    utils::{
      BOT_VERSION,
      GIT_COMMIT_BRANCH,
      GIT_COMMIT_HASH
    }
  },
  std::sync::atomic::{
    AtomicBool,
    Ordering::Relaxed
  }
};

static READY_ONCE: AtomicBool = AtomicBool::new(false);

async fn ready_once(
  ready: &Ready,
  framework: RustbotFwCtx<'_>
) -> RustbotResult<()> {
  #[cfg(not(feature = "production"))]
  {
    println!(
      "{RUSTBOT_EVENT}[Ready:Notice:S{}]: Detected a non-production environment!",
      framework.serenity_context.shard_id
    );
    let gateway = framework.serenity_context.http.get_bot_gateway().await?;
    let session = gateway.session_start_limit;
    println!(
      "{RUSTBOT_EVENT}[Ready:Notice:S{}]: Session limit: {}/{}",
      framework.serenity_context.shard_id, session.remaining, session.total
    );
  }

  println!(
    "{RUSTBOT_EVENT}[Ready:S{}]: Build version: {} ({GIT_COMMIT_HASH}:{GIT_COMMIT_BRANCH})",
    framework.serenity_context.shard_id, *BOT_VERSION
  );
  println!(
    "{RUSTBOT_EVENT}[Ready:S{}]: Connected to API as {}",
    framework.serenity_context.shard_id, ready.user.name
  );

  let message = CreateMessage::new();
  let ready_embed = CreateEmbed::new()
    .color(BINARY_PROPERTIES.embed_color)
    .thumbnail(ready.user.avatar_url().unwrap_or_default())
    .author(CreateEmbedAuthor::new(format!("{} is ready!", ready.user.name)));

  ChannelId::new(BINARY_PROPERTIES.rustbot_logs)
    .send_message(&framework.serenity_context.http, message.add_embed(ready_embed))
    .await?;

  Ok(())
}

impl EventProcessor<'_> {
  pub async fn on_ready(
    &self,
    data_about_bot: &Ready
  ) -> RustbotResult<()> {
    if !READY_ONCE.swap(true, Relaxed) {
      ready_once(data_about_bot, self.framework)
        .await
        .expect("Failed to call ready_once method");
    }

    Ok(())
  }
}
