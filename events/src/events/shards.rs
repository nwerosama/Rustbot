use super::{
  EventProcessor,
  RUSTBOT_EVENT
};

use std::num::NonZero;
use rustbot_lib::RustbotResult;
use poise::serenity_prelude::ShardStageUpdateEvent;

impl EventProcessor<'_> {
  pub async fn on_shards_ready(
    &self,
    total_shards: &NonZero<u16>
  ) -> RustbotResult<()> {
    let shards = if *total_shards == NonZero::new(1).unwrap() { "shard is" } else { "shards are" };
    println!("{RUSTBOT_EVENT}[ShardsReady]: {total_shards} {shards} ready!");

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
