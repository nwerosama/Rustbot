pub mod config;
mod data;
pub use data::RustbotData;
pub mod utils;

type RustbotError = Box<dyn std::error::Error + Send + Sync>;
pub type RustbotContext<'a> = poise::Context<'a, RustbotData, RustbotError>;
pub type RustbotFwCtx<'a> = poise::FrameworkContext<'a, RustbotData, RustbotError>;
pub type RustbotResult<T> = Result<T, RustbotError>;
