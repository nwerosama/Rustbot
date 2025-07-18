use {
  poise::serenity_prelude::GuildId,
  std::sync::LazyLock
};

pub struct ConfigMeta {
  pub env:          &'static str,
  pub embed_color:  u32,
  pub rustbot_logs: u64,
  pub dev_guild:    GuildId,
  pub developers:   Vec<u64>
}

#[cfg(feature = "production")]
pub static BINARY_PROPERTIES: LazyLock<ConfigMeta> = LazyLock::new(ConfigMeta::new);

#[cfg(not(feature = "production"))]
pub static BINARY_PROPERTIES: LazyLock<ConfigMeta> = LazyLock::new(|| {
  ConfigMeta::new()
    .env("dev")
    .embed_color(0xF1D63C)
    .dev_guild(GuildId::new(865673694184996885))
});

impl ConfigMeta {
  fn new() -> Self {
    Self {
      env:          "prod",
      embed_color:  0xF1D63C,
      rustbot_logs: 1311282815601741844,
      dev_guild:    GuildId::new(865673694184996885),
      developers:   vec![
        190407856527376384, // nwero.sama
      ]
    }
  }

  // Scalable functions below;
  #[cfg(not(feature = "production"))]
  fn env(
    mut self,
    env: &'static str
  ) -> Self {
    self.env = env;
    self
  }

  #[cfg(not(feature = "production"))]
  fn embed_color(
    mut self,
    color: u32
  ) -> Self {
    self.embed_color = color;
    self
  }

  #[cfg(not(feature = "production"))]
  fn dev_guild(
    mut self,
    dev_guild: GuildId
  ) -> Self {
    self.dev_guild = dev_guild;
    self
  }
}
