use {
  cargo_toml::Manifest,
  poise::serenity_prelude::UserId,
  std::sync::LazyLock
};

#[cfg(feature = "production")]
pub static GIT_COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");
#[cfg(not(feature = "production"))]
pub static GIT_COMMIT_HASH: &str = "devel";
pub static GIT_COMMIT_BRANCH: &str = env!("GIT_COMMIT_BRANCH");

pub static BOT_VERSION: LazyLock<String> = LazyLock::new(|| {
  Manifest::from_str(include_str!("../../Cargo.toml"))
    .unwrap()
    .package
    .unwrap()
    .version
    .unwrap()
});

pub fn mention_dev(ctx: super::RustbotContext<'_>) -> Option<String> {
  let devs = super::config::BINARY_PROPERTIES.developers.clone();
  let app_owners = ctx.framework().options().owners.clone();

  let mut mentions = Vec::new();

  for dev in devs {
    if app_owners.contains(&UserId::new(dev)) {
      mentions.push(format!("<@{dev}>"));
    }
  }

  if mentions.is_empty() { None } else { Some(mentions.join(", ")) }
}

pub fn get_guild_name(ctx: super::RustbotContext<'_>) -> String {
  match ctx.guild() {
    Some(guild) => guild.name.clone().to_string(),
    None => String::from("DM")
  }
}
