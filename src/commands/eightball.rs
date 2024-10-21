use crate::RustbotError;
use super::PoiseContext;

use rustbot_lib::config::BINARY_PROPERTIES;
use poise::{
  serenity_prelude::UserId,
  builtins::paginate
};

/// Ask the Magic 8-Ball a yes/no question and get an unpredictable answer
#[poise::command(
  slash_command,
  rename = "8ball"
)]
pub async fn eightball(
  ctx: PoiseContext<'_>,
  #[description = "Your yes/no question"] question: String
) -> Result<(), RustbotError> {
  if question.to_ascii_lowercase().contains("rustbot, show list") {
    if ctx.author().id == UserId::new(BINARY_PROPERTIES.developers[0]) {
      let chunks: Vec<String> = RESPONSES.chunks(10).map(|chunk| chunk.join("\n\n")).collect();
      let pages: Vec<&str> = chunks.iter().map(|s| s.as_str()).collect();
      paginate(ctx, &pages).await?;

      return Ok(());
    } else {
      ctx.reply("No.").await?;
      return Ok(());
    }
  }

  ctx.reply(format!(
    "> {}\n{}",
    question,
    get_random_response()
  )).await?;

  Ok(())
}

const RESPONSES: [&str; 30] = [
  "Reply hazy. Look it up on Google.", // no
  "Meh — Figure it out yourself.", // no
  "I don't know, what do you think?", // no
  "Yes.", // yes
  "No.", // no
  "It is decidedly so", // yes
  "Signs point to... maybe... depends on... \
  hold on, let me get my glasses, this is getting \
  pretty tiny... depends on whether you'd be up \
  to getting to know your Magic 8-Ball a little better.", // no
  "Signs point to... ~~yes~~ no.", // no
  "Why do you want to know the answer? It's obviously a yes.", // yes
  "Outlook not so good.", // no
  "Outlook hazy.", // no
  "What are you, stupid?", // no
  "How the hell do you not know that?", // no
  "Really? Making a decision based on what the plastic 8-Ball says? Jesus...", // no
  "Try asking later...", // no
  "I don't know, whip out the ouija board and try again?", // no
  "The answer is yes.", // yes
  "Yes, actually no. Wait, nevermind.", // no
  "Maybeee...", // yes
  "Definitely!", // yes
  "It is decidedly so.", // yes
  "My reply is no.", // no
  "My sources confirms that the answer is no.\n\
  Source: :sparkles: *i made it up* :sparkles:", // no
  "As I see it, yes.", // yes
  "Don't count on it.", // no
  "Whoa! Why do I have to answer this?", // no
  "Highly unlikely.", // no
  "Sure, but with extreme cautions.", // yes
  "What kind of stupid question is that?? No! I'm not answering that!", // no
  "Try asking this to a chicken. Probably knows it better than I do!", // no
];

fn get_random_response() -> &'static str {
  RESPONSES[rand::random::<usize>() % RESPONSES.len()]
}
