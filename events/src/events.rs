mod ready;

use poise::serenity_prelude::{
  Context,
  EventHandler,
  FullEvent,
  async_trait
};

pub const RUSTBOT_EVENT: &str = "RustbotEvent";

pub struct RustbotEvents;

#[async_trait]
impl EventHandler for RustbotEvents {
  async fn dispatch(
    &self,
    ctx: &Context,
    event: &FullEvent
  ) {
    match event {
      FullEvent::Ready { data_about_bot, .. } => ready::on_ready(ctx, data_about_bot).await.unwrap(),
      FullEvent::ShardsReady { total_shards, .. } => println!("{RUSTBOT_EVENT}[ShardsReady] {total_shards} shards are up and ready!"),
      _ => ()
    }
  }
}
