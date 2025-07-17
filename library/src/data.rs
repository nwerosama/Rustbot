use {
  poise::serenity_prelude::Http,
  std::sync::Arc
};

pub struct RustbotData {
  pub http:   Arc<Http>,
  pub config: &'static crate::config::ConfigMeta
}
