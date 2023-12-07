mod commands;

use poise::serenity_prelude::{self as serenity};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub static COLOR: i32 = 0xf1d63c;

async fn on_ready(
  ctx: &serenity::Context,
  ready: &serenity::Ready,
  framework: &poise::Framework<(), Error>
) -> Result<(), Error> {
  println!("Connected to API as {}", ready.user.name);

  serenity::ChannelId(865673694184996888).send_message(&ctx.http, |m| {
    m.embed(|e| {
      e.color(COLOR)
        .thumbnail(ready.user.avatar_url().unwrap_or_default())
        .author(|a| {
          a.name(format!("{} is ready!", ready.user.name))
        })
    })
  }).await?;

  let register_commands = std::env::var("REGISTER_CMDS").unwrap_or_else(|_| String::from("true")).parse::<bool>().unwrap_or(true);

  if register_commands {
    let builder = poise::builtins::create_application_commands(&framework.options().commands);
    let commands = serenity::Command::set_global_application_commands(&ctx.http, |commands| {
      *commands = builder.clone();
      commands
    }).await;

    match commands {
      Ok(cmdmap) => {
        for command in cmdmap.iter() {
          println!("Registered command globally: {}", command.name);
        }
      },
      Err(why) => println!("Error registering commands: {:?}", why)
    }
  }

  Ok(())
}

#[tokio::main]
async fn main() {
  let token = std::env::var("DISCORD_TOKEN").expect("Expected a \"DISCORD_TOKEN\" in the envvar but none was found");

  let client = poise::Framework::builder().token(token)
    .intents(serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILDS)
    .options(poise::FrameworkOptions {
      commands: vec![
        commands::ping::ping(),
        commands::eval::eval(),
        commands::data::data()
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
