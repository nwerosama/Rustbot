use poise::serenity_prelude::UserId;
use cargo_toml::Manifest;
use std::sync::LazyLock;

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

pub fn format_timestamp(timestamp: i64) -> String {
  format!("<t:{timestamp}>\n<t:{timestamp}:R>")
}

pub fn mention_dev(ctx: super::RustbotCtx<'_>) -> Option<String> {
  let devs = super::config::BINARY_PROPERTIES.developers.clone();
  let app_owners = ctx.framework().options().owners.clone();

  let mut mentions = Vec::new();

  for dev in devs {
    if app_owners.contains(&UserId::new(dev)) {
      mentions.push(format!("<@{dev}>"));
    }
  }

  if mentions.is_empty() {
    None
  } else {
    Some(mentions.join(", "))
  }
}

pub fn get_guild_name(ctx: super::RustbotCtx<'_>) -> String {
  match ctx.guild() {
    Some(guild) => guild.name.clone().to_string(),
    None => String::from("DM")
  }
}

pub fn format_duration(secs: u64) -> String {
  let days = secs / 86400;
  let hours = (secs % 86400) / 3600;
  let minutes = (secs % 3600) / 60;
  let seconds = secs % 60;

  let components = [
    (days, "d"),
    (hours, "h"),
    (minutes, "m"),
    (seconds, "s"),
  ];

  let formatted_string: Vec<String> = components
  .iter()
  .filter(|&&(value, _)| value > 0)
  .map(|&(value, suffix)| format!("{}{}", value, suffix))
  .collect();

  formatted_string.join(", ")
}
