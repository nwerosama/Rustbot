pub mod config;
mod data;
pub use data::RustbotData;
pub mod utils;

pub type RustbotError = Box<dyn std::error::Error + Send + Sync>;
pub type RustbotCtx<'a> = poise::Context<'a, RustbotData, RustbotError>;
