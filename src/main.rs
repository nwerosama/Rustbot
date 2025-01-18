mod errors;
mod shutdown;
// https://cdn.toast-server.net/RustFSHiearchy.png
// Using the new filesystem hierarchy

use {
  poise::serenity_prelude::{
    ActivityData,
    ClientBuilder,
    GatewayIntents,
    builder::CreateAllowedMentions
  },
  rustbot_cmds::collect,
  rustbot_events::events::processor,
  rustbot_lib::{
    RustbotData,
    config::BINARY_PROPERTIES,
    utils::get_guild_name
  },
  rustbot_tokens::discord_token,
  std::{
    borrow::Cow,
    sync::Arc
  }
};

#[tokio::main]
async fn main() {
  let prefix = if BINARY_PROPERTIES.env.contains("prod") {
    Some(Cow::Borrowed("pg."))
  } else {
    Some(Cow::Borrowed("pg!"))
  };

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      commands: collect!(),
      pre_command: |ctx| {
        Box::pin(async move {
          let get_guild_channel_name = match ctx.guild_channel().await {
            Some(channel) => format!("in #{}", channel.name.clone()),
            None => String::from("")
          };
          let prefix = match ctx.command().prefix_action {
            Some(_) => ctx.framework().options.prefix_options.prefix.as_ref().unwrap(),
            None => "/"
          };

          println!(
            "Discord[{}:S{}]: {} ran {prefix}{} {get_guild_channel_name}",
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
      event_handler: |framework, event| Box::pin(processor(framework, event)),
      ..Default::default()
    })
    .build();

  let mut client = ClientBuilder::new(
    discord_token().await,
    GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT
  )
  .framework(framework)
  .data(Arc::new(RustbotData {}))
  .activity(ActivityData::custom("nep nep!"))
  .await
  .expect("Error creating client");

  let shard_manager = client.shard_manager.clone();

  tokio::spawn(async move {
    shutdown::gracefully_shutdown().await;
    shard_manager.shutdown_all().await;
  });

  if let Err(why) = client.start_autosharded().await {
    println!("Error starting client: {why:#?}");
  }
}
