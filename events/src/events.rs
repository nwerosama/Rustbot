mod ready;
mod shards;

use rustbot_lib::{
  RustbotError,
  RustbotData
};
use poise::{
  FrameworkContext,
  serenity_prelude::FullEvent
};

pub const RUSTBOT_EVENT: &str = "RustbotEvent";

struct EventProcessor<'a> {
  framework: FrameworkContext<'a, RustbotData, RustbotError>
}

pub async fn processor(
  framework: FrameworkContext<'_, RustbotData, RustbotError>,
  event: &FullEvent
) -> Result<(), RustbotError> {
  let processor = EventProcessor { framework };

  match event {
    FullEvent::Ready { data_about_bot } => processor.on_ready(data_about_bot).await?,
    FullEvent::ShardsReady { total_shards } => processor.on_shards_ready(total_shards).await?,
    FullEvent::ShardStageUpdate { event } => processor.on_shards_stageupdate(event).await?,
    _ => {}
  }

  Ok(())
}
