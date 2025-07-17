pub mod config;
mod data;
mod tsclient;
pub mod utils;

pub use {
  data::RustbotData,
  tsclient::{
    discord_token,
    token_path
  }
};

pub type RustbotError = Box<dyn std::error::Error + Send + Sync>;
pub type RustbotContext<'a> = poise::Context<'a, RustbotData, RustbotError>;
pub type RustbotFwCtx<'a> = poise::FrameworkContext<'a, RustbotData, RustbotError>;
pub type RustbotResult<T> = Result<T, RustbotError>;
