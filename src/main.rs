mod errors;
mod shutdown;
// https://cdn.toast-server.net/RustFSHiearachy.png
// Using the new filesystem hiearachy

use {
  asahi::{
    error,
    info,
    spawn
  },
  poise::serenity_prelude::{
    ActivityData,
    ClientBuilder,
    GatewayIntents,
    HttpBuilder,
    builder::CreateAllowedMentions
  },
  rustbot_cmds::collect,
  rustbot_events::RustbotEvents,
  rustbot_lib::{
    RustbotData,
    config::BINARY_PROPERTIES,
    discord_token,
    utils::get_guild_name
  },
  std::{
    borrow::Cow,
    sync::Arc
  }
};

#[tokio::main]
async fn main() {
  asahi::log_init();

  let prefix = Some(Cow::Borrowed(if BINARY_PROPERTIES.env.contains("prod") { "pg." } else { "pg!" }));

  let http = Arc::new(HttpBuilder::new(discord_token().await).build());

  let data = Arc::new(RustbotData {
    http,
    config: &BINARY_PROPERTIES
  });

  spawn(example_task::ExampleTask);

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      commands: collect(),
      pre_command: |ctx| {
        Box::pin(async move {
          let get_guild_channel_name = match ctx.channel().await {
            Some(channel) => format!("in #{}", channel.guild().unwrap_or_default().base.name),
            None => String::from("")
          };
          let prefix = ctx
            .command()
            .prefix_action
            .map_or("/", |_| ctx.framework().options.prefix_options.prefix.as_ref().unwrap());

          info!(
            "Discord[{}:S{}] {} ran {prefix}{} {get_guild_channel_name}",
            get_guild_name(ctx),
            ctx.serenity_context().shard_id,
            ctx.author().name,
            ctx.command().qualified_name,
          );
        })
      },
      prefix_options: poise::PrefixFrameworkOptions {
        prefix,
        ignore_bots: true,
        mention_as_prefix: false,
        case_insensitive_commands: true,
        execute_self_messages: false,
        ..Default::default()
      },
      on_error: |error| Box::pin(async move { errors::fw_errors(error).await }),
      allowed_mentions: Some(CreateAllowedMentions::default().empty_users()),
      initialize_owners: true,
      skip_checks_for_owners: true,
      ..Default::default()
    })
    .build();

  let mut client = ClientBuilder::new(
    discord_token().await,
    GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT
  )
  .framework(framework)
  .event_handler(RustbotEvents)
  .data(Arc::clone(&data))
  .activity(ActivityData::custom("nep nep!"))
  .await
  .expect("Error creating client");

  let shutdown_trigger = client.shard_manager.get_shutdown_trigger();
  let exit_signal = tokio::spawn(async move { shutdown::gracefully_shutdown().await });

  tokio::select! {
    client_result = client.start() => {
      if let Err(why) = client_result {
        error!("(Serenity) Error starting client: {why:#?}")
      }
    },
    shutdown = exit_signal => {
      if shutdown.unwrap() {
        shutdown_trigger();
        std::process::exit(0);
      }
    }
  }
}

mod example_task {
  use asahi::{
    AsahiCoordinator,
    AsahiResult
  };

  pub struct ExampleTask;

  #[asahi::async_trait]
  impl AsahiCoordinator for ExampleTask {
    fn name(&self) -> &'static str { "example_task" }

    fn interval(&self) -> u64 { 15 }

    async fn main_loop(&self) -> AsahiResult<()> {
      asahi::info!("hello from ExampleTask !");
      Ok(())
    }
  }
}
