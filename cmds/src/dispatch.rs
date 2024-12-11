mod dev;
mod eightball;
mod ping;
mod uptime;

pub use {
  dev::dev,
  eightball::eightball,
  ping::ping,
  uptime::uptime
};

#[macro_export]
macro_rules! collect {
  () => {
    vec![
      // Developer command(s)
      $crate::dev(),
      // Utility commands
      $crate::ping(),
      $crate::uptime(),
      // Unsorted mess
      $crate::eightball(),
    ]
  };
}
