mod dev;
mod edit_test;
mod eightball;
mod pin_test;
mod ping;
mod uptime;

pub use {
  dev::dev,
  edit_test::edit_test,
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
      $crate::edit_test(),
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
