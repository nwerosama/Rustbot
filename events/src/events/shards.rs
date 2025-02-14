use super::{
  EventProcessor,
  RUSTBOT_EVENT
};

use {
  poise::serenity_prelude::ShardStageUpdateEvent,
  rustbot_lib::RustbotResult,
  std::num::NonZero
};

impl EventProcessor<'_> {
  pub async fn on_shards_ready(
    &self,
    total_shards: &NonZero<u16>
  ) -> RustbotResult<()> {
    let is_singular = total_shards.get() == 1;
    println!(
      "{RUSTBOT_EVENT}[ShardsReady]: {total_shards} {} ready!",
      if is_singular { "shard is" } else { "shards are" }
    );
    Ok(())
  }

  pub async fn on_shards_stageupdate(
    &self,
    event: &ShardStageUpdateEvent
  ) -> RustbotResult<()> {
    println!("{RUSTBOT_EVENT}[ShardStageUpdate:S{}]: {event:#?}", event.shard_id);
    Ok(())
  }
}
