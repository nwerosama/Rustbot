mod ready;

use {
  poise::serenity_prelude::{
    Context,
    EventHandler,
    Ready,
    async_trait
  },
  std::num::NonZero
};

pub const RUSTBOT_EVENT: &str = "RustbotEvent";

pub struct RustbotEvents;

#[async_trait]
impl EventHandler for RustbotEvents {
  async fn ready(
    &self,
    ctx: Context,
    ready: Ready
  ) {
    ready::on_ready(ctx, ready).await.unwrap()
  }

  async fn shards_ready(
    &self,
    _: Context,
    total_shards: NonZero<u16>
  ) {
    println!("{RUSTBOT_EVENT}[ShardsReady] {total_shards} shards are up and ready!")
  }
}
