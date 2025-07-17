use {
  asahi::info,
  poise::serenity_prelude::{
    Context,
    CreateEmbed,
    CreateEmbedAuthor,
    CreateMessage,
    GenericChannelId,
    Ready
  },
  rustbot_lib::{
    RustbotData,
    RustbotResult,
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
    asahi::warn!("[Ready:S{sid}] Detected a development environment!",);
    let gateway = ctx.http.get_bot_gateway().await?;
    let session = gateway.session_start_limit;
    asahi::warn!("[Ready:S{sid}] Gateway session limit: {}/{}", session.remaining, session.total);
  }

  info!("[Ready:S{sid}] Build version: v{} ({GIT_COMMIT_HASH}:{GIT_COMMIT_BRANCH})", *BOT_VERSION);
  info!("[Ready:S{sid}] Connected to API as {}", ready.user.name);

  let message = CreateMessage::new();
  let ready_embed = CreateEmbed::new()
    .color(ctx.data::<RustbotData>().config.embed_color)
    .thumbnail(ready.user.avatar_url().unwrap_or_default())
    .author(CreateEmbedAuthor::new(format!("{} is ready!", ready.user.name)));

  GenericChannelId::new(ctx.data::<RustbotData>().config.rustbot_logs)
    .send_message(&ctx.http, message.embed(ready_embed))
    .await?;

  Ok(())
}

pub async fn on_ready(
  ctx: &Context,
  ready: &Ready
) -> RustbotResult<()> {
  if ctx.shard_id.0 == 0 {
    ready_once(ctx, ready).await.expect("Failed to call ready_once method");
  }

  Ok(())
}
