use {
  poise::serenity_prelude::async_trait,
  rustbot_lib::{
    RustbotData,
    RustbotResult
  },
  std::sync::Arc,
  tokio::time::{
    Duration,
    interval
  }
};

#[async_trait]
pub trait TaskCoordinator: Send + Sync {
  fn name(&self) -> &'static str;
  fn interval(&self) -> Duration;
  async fn run(
    &self,
    data: Arc<RustbotData>
  ) -> RustbotResult<()>;
}

pub async fn spawn_task<T: TaskCoordinator + 'static>(
  task: T,
  data: Arc<RustbotData>
) {
  let t_name = task.name().to_string();
  let t_interval = task.interval();

  tokio::spawn(async move {
    let mut interval = interval(t_interval);
    loop {
      interval.tick().await;
      if let Err(e) = task.run(data.clone()).await {
        eprintln!("TaskCoordinator[Err:{t_name}] {e}")
      }
    }
  });
}
