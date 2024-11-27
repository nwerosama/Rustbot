mod dev;
mod eightball;
mod ping;
mod uptime;

pub use dev::dev;
pub use eightball::eightball;
pub use ping::ping;
pub use uptime::uptime;

macro_rules! collect {
  () => {
    vec![
      // Developer command(s)
      commands::dev(),

      // Utility commands
      commands::ping(),
      commands::uptime(),

      // Unsorted mess
      commands::eightball(),
    ]
  };
}
pub(crate) use collect;
