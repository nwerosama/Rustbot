mod errors;
mod shutdown;
// https://cdn.toast-server.net/RustFSHiearachy.png
// Using the new filesystem hiearachy

use {
  poise::serenity_prelude::{
    ActivityData,
    ClientBuilder,
    GatewayIntents,
    builder::CreateAllowedMentions
  },
  rustbot_cmds::collect,
  rustbot_events::RustbotEvents,
  rustbot_lib::{
    RustbotData,
    config::BINARY_PROPERTIES,
    utils::get_guild_name
  },
  rustbot_tasks::spawn_task,
  rustbot_tokens::discord_token,
  std::{
    borrow::Cow,
    sync::Arc
  }
};

#[tokio::main]
async fn main() {
  let prefix = Some(Cow::Borrowed(if BINARY_PROPERTIES.env.contains("prod") { "pg." } else { "pg!" }));

  let data = Arc::new(RustbotData {});

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      commands: collect!(),
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

          println!(
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

  spawn_task(example_task::ExampleTask, Arc::clone(&data)).await;
  spawn_task(example_task2::ExampleTask2, Arc::clone(&data)).await;

  let exit_signal = tokio::spawn(async move { shutdown::gracefully_shutdown().await });

  tokio::select! {
    client_result = client.start() => {
      if let Err(why) = client_result {
        eprintln!("Client error: {why:#?}")
      }
    },
    shutdown = exit_signal => {
      if shutdown.unwrap() {
        std::process::exit(0)
      }
    }
  }
}

mod example_task {
  use {
    poise::serenity_prelude::async_trait,
    rustbot_lib::{
      RustbotData,
      RustbotResult
    },
    rustbot_tasks::TaskCoordinator,
    std::sync::Arc,
    tokio::time::Duration
  };

  #[derive(Clone)]
  pub struct ExampleTask;

  #[async_trait]
  impl TaskCoordinator for ExampleTask {
    fn name(&self) -> &'static str { "example_task" }

    fn interval(&self) -> Duration { Duration::from_secs(10) }

    async fn run(
      &self,
      _: Arc<RustbotData>
    ) -> RustbotResult<()> {
      println!("hello from ExampleTask !");
      Ok(())
    }
  }
}

mod example_task2 {
  use {
    poise::serenity_prelude::async_trait,
    rustbot_lib::{
      RustbotData,
      RustbotResult
    },
    rustbot_tasks::TaskCoordinator,
    std::sync::Arc,
    tokio::time::Duration
  };

  #[derive(Clone)]
  pub struct ExampleTask2;

  #[async_trait]
  impl TaskCoordinator for ExampleTask2 {
    fn name(&self) -> &'static str { "example_task2" }

    fn interval(&self) -> Duration { Duration::from_secs(5) }

    async fn run(
      &self,
      _: Arc<RustbotData>
    ) -> RustbotResult<()> {
      println!("hello from ExampleTask2 !");
      Ok(())
    }
  }
}
