mod dev;
mod eightball;
mod pin_test;
mod ping;
mod uptime;

pub use {
  dev::dev,
  eightball::eightball,
  pin_test::pin_test,
  ping::ping,
  uptime::uptime
};

#[macro_export]
macro_rules! collect {
  () => {
    vec![
      // Developer command(s)
      $crate::dev(),
      $crate::pin_test(),
      // Utility commands
      $crate::ping(),
      $crate::uptime(),
      // Unsorted mess
      $crate::eightball(),
    ]
  };
}
