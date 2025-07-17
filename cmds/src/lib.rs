mod dev;
mod eightball;
mod ping;
mod uptime;

use {
  dev::dev,
  eightball::eightball,
  ping::ping,
  uptime::uptime
};

use {
  poise::Command,
  rustbot_lib::{
    RustbotContext,
    RustbotData,
    RustbotError,
    RustbotResult
  }
};

pub type PoiseCmdData = Vec<Command<RustbotData, RustbotError>>;

pub fn collect() -> PoiseCmdData {
  let mut cmds = collect_global();
  cmds.extend(collect_local());
  cmds
}

fn collect_global() -> PoiseCmdData { vec![deploy(), eightball(), ping(), uptime()] }

fn collect_local() -> PoiseCmdData { vec![dev()] }

/// Deploy the commands both globally and locally
#[poise::command(prefix_command, owners_only)]
pub async fn deploy(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  let http = ctx.http();

  #[cfg(not(feature = "production"))]
  poise::builtins::register_in_guild(http, &collect(), ctx.data().config.dev_guild)
    .await
    .unwrap();

  #[cfg(feature = "production")]
  {
    poise::builtins::register_globally(http, &collect_global()).await.unwrap();
    poise::builtins::register_in_guild(http, &collect_local(), ctx.data().config.dev_guild)
      .await
      .unwrap();
  }

  #[cfg(feature = "production")]
  ctx
    .reply(format!(
      "Deployed global commands and local commands in **{}**",
      ctx.data().config.dev_guild.name(ctx.cache()).expect("no guild cache")
    ))
    .await
    .unwrap();

  #[cfg(not(feature = "production"))]
  ctx
    .reply(format!(
      "Deployed local commands in **{}**",
      ctx.data().config.dev_guild.name(ctx.cache()).expect("no guild cache")
    ))
    .await
    .unwrap();

  Ok(())
}
