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

  if question.to_ascii_lowercase().contains("rustbot, show chicken list") {
    if ctx.author().id == UserId::new(BINARY_PROPERTIES.developers[0]) {
      let chunks: Vec<String> = CHICKEN_RESPONSES.chunks(10).map(|chunk| chunk.join("\n\n")).collect();
      let pages: Vec<&str> = chunks.iter().map(|s| s.as_str()).collect();
      paginate(ctx, &pages).await?;

      return Ok(());
    } else {
      ctx.reply("No.").await?;
      return Ok(());
    }
  }

  let rand_resp = if question.to_ascii_lowercase().contains("chicken") {
    get_random_chicken_response()
  } else {
    get_random_response()
  };

  ctx.reply(format!("> {question}\n{rand_resp}")).await?;

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

const CHICKEN_RESPONSES: [&str; 35] = [
  "Cluck cluck... Reply hazy, try pecking Google.", // no
  "Meh... Figure it out yourself, or scratch around a bit.", // no
  "I don’t know... what do you think? *pecks at ground*", // no
  "BAWK! YES!", // yes
  "Cluck... no.", // no
  "It is decidedly so! *flaps wings*", // yes
  "Signs point to... maybe... hold on, let me fluff my feathers... depends on whether you'd get to know your Magic Chicken a bit better.", // no
  "Signs point to... ~~yes~~ cluck no.", // no
  "Why do you want to know? It’s a big cluckin’ yes!", // yes
  "Outlook not so clucking good.", // no
  "Outlook cluckin' hazy.", // no
  "What are you, a lost chick? Cluck!", // no
  "How the cluck do you not know that?", // no
  "Really? Asking a chicken to decide your fate? *clucks judgmentally*", // no
  "Peck back later, I'm nesting...", // no
  "I don’t know, try flapping your wings and ask again?", // no
  "The answer is a big ol' yes! *flaps happily*", // yes
  "Yes... wait, actually... no. Cluck, I’m confused.", // no
  "Maaaaybe... *chicken waddle*?", // yes
  "Definitely! *struts confidently*", // yes
  "It is decidedly so. *struts with pride*", // yes
  "My reply is a solid *cluck* no.", // no
  "My sources confirm it's a cluckin' no.\nSource: 🐔 *I made it up* 🐔", // no
  "As I see it, yes! *pecks approvingly*", // yes
  "Don’t count on it. *cluck cluck*", // no
  "Whoa, why do I have to answer this? *fluffs feathers*", // no
  "Highly unlikely. *chicken stare*", // no
  "Sure, but with extreme cluckin' caution.", // yes
  "What kind of stupid question is that?? No! *angry clucks*", // no
  "Try asking this to a fellow chicken. They probably know better than I do!", // no
  "Cluck yes! *does a happy chicken dance*", // yes
  "No way, not even for a big bag of feed.", // no
  "Yes! *lays egg of approval*", // yes
  "It's a no, better go scratch somewhere else.", // no
  "Cluck-tastic! That’s a definite yes.", // yes
];

fn get_random_response() -> &'static str {
  RESPONSES[rand::random::<usize>() % RESPONSES.len()]
}

fn get_random_chicken_response() -> &'static str {
  CHICKEN_RESPONSES[rand::random::<usize>() % CHICKEN_RESPONSES.len()]
}
