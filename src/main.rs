mod commands;

use poise::serenity_prelude::{self as serenity};

type Error = Box<dyn std::error::Error + Send + Sync>;

const GUILD_ID: serenity::GuildId = serenity::GuildId(865673694184996885);

async fn on_ready(
  ctx: &serenity::Context,
  ready: &serenity::Ready,
  framework: &poise::Framework<(), Error>
) -> Result<(), Error> {
  println!("Connected to API as {}", ready.user.name);

  let builder = poise::builtins::create_application_commands(&framework.options().commands);
  let commands = serenity::GuildId::set_application_commands(&GUILD_ID, &ctx.http, |commands| {
      *commands = builder.clone();
      commands
    }).await;
  
  println!("Registered the following commands: \n{:#?}", commands);

  Ok(())
}

#[tokio::main]
async fn main() {
  let token = std::env::var("DISCORD_TOKEN").expect("Expected a \"DISCORD_TOKEN\" in the envvar but none was found");

  let client = poise::Framework::builder().token(token)
    .intents(serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILDS)
    .options(poise::FrameworkOptions {
      commands: vec![
        commands::ping::ping()
      ],
      pre_command: |ctx| {
        Box::pin(async move {
          println!("{} ran /{}", ctx.author().name, ctx.command().name)
        })
      },
      ..Default::default()
    }).setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)))
    .build().await.expect("Error while building the client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
