mod commands;
// https://cdn.toast-server.net/RustFSHiearchy.png
// Using the new filesystem hierarchy

use rustbot_tokens::token_path;
use poise::serenity_prelude::{
  builder::CreateAllowedMentions,
  ClientBuilder,
  ActivityData,
  GatewayIntents
};
use rustbot_lib::{
  utils::{
    mention_dev,
    get_guild_name
  },
  RustbotError,
  RustbotData,
  config::BINARY_PROPERTIES
};
use rustbot_events::events::processor;
use std::{
  sync::Arc,
  borrow::Cow
};

#[tokio::main]
async fn main() {
  let prefix = if BINARY_PROPERTIES.env.contains("prod") {
    Some(Cow::Borrowed("pg."))
  } else {
    Some(Cow::Borrowed("pg!"))
  };

  let commands = commands::collect!();
  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      commands,
      pre_command: |ctx| Box::pin(async move {
        let get_guild_channel_name = match ctx.guild_channel().await {
          Some(channel) => format!("in #{}", channel.name.clone()),
          None => String::from("")
        };
        let prefix = match ctx.command().prefix_action {
          Some(_) => ctx.framework().options.prefix_options.prefix.as_ref().unwrap(),
          None => "/"
        };

        println!(
          "Discord[{}:S{}]: {} ran {}{} {}",
          get_guild_name(ctx),
          ctx.serenity_context().shard_id,
          ctx.author().name,
          prefix,
          ctx.command().qualified_name,
          get_guild_channel_name);
      }),
      prefix_options: poise::PrefixFrameworkOptions {
        prefix,
        case_insensitive_commands: true,
        ignore_bots: true,
        execute_self_messages: false,
        mention_as_prefix: false,
        ..Default::default()
      },
      on_error: |error| Box::pin(async move {
        match error {
          poise::FrameworkError::Command { error, ctx, .. } => {
            println!("PoiseCommandError({}): {}", ctx.command().qualified_name, error);
            ctx.reply(format!(
              "Encountered an error during command execution, ask {} to check console for more details!",
              mention_dev(ctx).unwrap_or_default()
            )).await.expect("Error sending message");
          },
          poise::FrameworkError::EventHandler { error, event, .. } => println!("PoiseEventHandlerError({}): {}", event.snake_case_name(), error),
          poise::FrameworkError::NotAnOwner { ctx, .. } => {
            println!("PoiseNotAnOwner: {} tried to execute a developer-level command ({})", ctx.author().name, ctx.command().qualified_name);
            ctx.reply("Whoa, you discovered a hidden command! Too bad, I can't allow you to execute it as you're not my creator.").await.expect("Error sending message");
          },
          poise::FrameworkError::UnknownInteraction { interaction, .. } => println!(
            "PoiseUnknownInteractionError: {} tried to execute an unknown interaction ({})",
            interaction.user.name,
            interaction.data.name
          ),
          poise::FrameworkError::UnknownCommand { msg, .. } => println!(
            "PoiseUnknownCommandError: {} tried to execute an unknown command ({})",
            msg.author.name,
            msg.content
          ),
          poise::FrameworkError::ArgumentParse { ctx, error, .. } => {
            println!("PoiseArgumentParseError: {}", error);
            ctx.reply(format!("Error parsing argument(s): {error}")).await.expect("Error sending message");
          },
          poise::FrameworkError::CommandPanic { ctx, payload, .. } => {
            if let Some(payload) = payload.clone() {
              println!("PoiseCommandPanic: {payload}");
              ctx.reply(format!(
                "The command panicked, please tell my developer about this!\n**Error:**```\n{payload}\n```"
              )).await.expect("Error sending message");
            } else {
              println!("PoiseCommandPanic: No payload provided");
              let uh_oh = [
                "Well, this is concerning... Hopefully you notified my developer about this!",
                "The command panicked, but didn't leave any trace behind... Suspicious!",
              ].join("\n");
              ctx.reply(uh_oh).await.expect("Error sending message");
            }
          },
          other => println!("PoiseOtherError: {other}")
        }
      }),
      allowed_mentions: Some(CreateAllowedMentions::default().empty_users()),
      initialize_owners: true,
      skip_checks_for_owners: true,
      event_handler: |framework, event| Box::pin(processor(framework, event)),
      ..Default::default()
    })
    .build();

  let mut client = ClientBuilder::new(
    &token_path().await.main,
    GatewayIntents::GUILDS
    | GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT
  )
  .framework(framework)
  .data(Arc::new(RustbotData {}))
  .activity(ActivityData::custom("nep nep!"))
  .await.expect("Error creating client");

  if let Err(why) = client.start_autosharded().await {
    println!("Error starting client: {:#?}", why);
  }
}
