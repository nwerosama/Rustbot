use crate::Error;
use super::{
  super::config::BINARY_PROPERTIES,
  task_info,
  task_err
};

use std::sync::Arc;
use poise::serenity_prelude::{
  Context,
  ChannelId,
  CreateMessage
};
use tokio::time::{
  Duration,
  interval
};

pub async fn sample(ctx: Arc<Context>) -> Result<(), Error> {
  let task_name = "SampleTask";
  let mut interval = interval(Duration::from_secs(10));
  task_info(&task_name, "Task loaded!");

  loop {
    interval.tick().await;
    task_info(&task_name, "Task running!");

    if BINARY_PROPERTIES.rss_channel == 0 {
      task_err(&task_name, "RSS channel ID is not set!");
      ChannelId::new(BINARY_PROPERTIES.rustbot_logs).send_message(
        &ctx.http,
        CreateMessage::new().content("RSS channel ID is not set!")
      ).await.unwrap();

      continue;
    }

    ChannelId::new(BINARY_PROPERTIES.rss_channel).send_message(
      &ctx.http,
      CreateMessage::new().content("This is a sample message executed by a task!")
    ).await.unwrap();
  }
}
