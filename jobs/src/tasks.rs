use crate::RUSTBOT_SCHEDULER;

use {
  std::{
    future::Future,
    sync::Arc
  },
  tokio::{
    task,
    time::{
      Duration,
      interval
    }
  }
};

pub struct Scheduler;

impl Scheduler {
  pub fn new() -> Arc<Self> { Arc::new(Self) }

  pub async fn spawn_job<F, E>(
    &self,
    interval_secs: u64,
    job: Arc<dyn Fn() -> F + Send + Sync + 'static>
  ) where
    F: Future<Output = Result<(), E>> + Send + 'static,
    E: std::fmt::Debug
  {
    let mut interval = interval(Duration::from_secs(interval_secs));

    loop {
      interval.tick().await;

      let job_clone = job.clone();
      task::spawn(async move {
        if let Err(y) = job_clone().await {
          eprintln!("{RUSTBOT_SCHEDULER}[Job:Error]: {y:?}");
        }
      });
    }
  }
}
