mod ready;
mod shards;

use poise::serenity_prelude::FullEvent;
use rustbot_lib::{
  RustbotFwCtx,
  RustbotResult
};

pub const RUSTBOT_EVENT: &str = "RustbotEvent";

struct EventProcessor<'a> {
  framework: RustbotFwCtx<'a>
}

pub async fn processor(
  framework: RustbotFwCtx<'_>,
  event: &FullEvent
) -> RustbotResult<()> {
  let processor = EventProcessor { framework };

  match event {
    FullEvent::Ready { data_about_bot } => processor.on_ready(data_about_bot).await?,
    FullEvent::ShardsReady { total_shards } => processor.on_shards_ready(total_shards).await?,
    FullEvent::ShardStageUpdate { event } => processor.on_shards_stageupdate(event).await?,
    _ => {}
  }

  Ok(())
}
