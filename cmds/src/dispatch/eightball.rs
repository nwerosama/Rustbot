use {
  poise::{
    builtins::paginate,
    serenity_prelude::UserId
  },
  rand::random,
  rustbot_lib::{
    RustbotContext,
    RustbotResult,
    config::BINARY_PROPERTIES
  }
};

#[derive(poise::ChoiceParameter, Clone)]
enum ResponseMode {
  Normal,
  Chicken,
  #[name = "Chaotic & Unhinged"]
  Chaotic
}

/// Ask the Magic 8-Ball a yes/no question and get an unpredictable answer
#[poise::command(
  slash_command,
  install_context = "Guild|User",
  interaction_context = "Guild|BotDm|PrivateChannel",
  rename = "8ball"
)]
pub async fn eightball(
  ctx: RustbotContext<'_>,
  #[description = "Your yes/no question"] question: String,
  #[description = "Response modes"] mode: Option<ResponseMode>
) -> RustbotResult<()> {
  if question.to_ascii_lowercase().contains("niko, show list") {
    show_list(ctx, mode.clone().unwrap_or(ResponseMode::Normal)).await?;
    return Ok(())
  }

  let rand_resp = match mode {
    Some(ResponseMode::Chicken) => get_random_chicken_response(),
    Some(ResponseMode::Chaotic) => get_random_chaotic_response(),
    _ => get_random_response()
  };

  ctx.reply(format!("> {question}\n{rand_resp}")).await?;

  Ok(())
}

async fn show_list(
  ctx: RustbotContext<'_>,
  list_type: ResponseMode
) -> RustbotResult<()> {
  if ctx.author().id != UserId::new(BINARY_PROPERTIES.developers[0]) {
    ctx
      .reply("The list knows you're looking, but it's playing a game of hide and seek. For now, it wins.")
      .await?;
    return Ok(());
  }

  let chunks: Vec<String> = match list_type {
    ResponseMode::Normal => RESPONSES.chunks(10).map(|chunk| chunk.join("\n\n")).collect(),
    ResponseMode::Chicken => CHICKEN_RESPONSES.chunks(10).map(|chunk| chunk.join("\n\n")).collect(),
    ResponseMode::Chaotic => CHAOTIC_RESPONSES.chunks(10).map(|chunk| chunk.join("\n\n")).collect()
  };

  let pages: Vec<&str> = chunks.iter().map(|s| s.as_str()).collect();
  paginate(ctx, &pages).await?;

  Ok(())
}

const RESPONSES: [&str; 45] = [
  "Reply hazy. Look it up on Google.", // no
  "Meh — Figure it out yourself.",     // no
  "I don't know, what do you think?",  // no
  "Yes.",                              // yes
  "No.",                               // no
  "It is decidedly so",                // yes
  "Signs point to... maybe... depends on... hold on, let me get my glasses, this is getting pretty tiny... depends on whether you'd be up to \
   getting to know your Magic 8-Ball a little better.", // no
  "Signs point to... ~~yes~~ no.",     // no
  "Why do you want to know the answer? It's obviously a yes.", // yes
  "Outlook not so good.",              // no
  "Outlook hazy.",                     // no
  "What are you, stupid?",             // no
  "How the hell do you not know that?", // no
  "Really? Making a decision based on what the plastic 8-Ball says? Jesus...", // no
  "Try asking later...",               // no
  "I don't know, whip out the ouija board and try again?", // no
  "The answer is yes.",                // yes
  "Yes, actually no. Wait, nevermind.", // no
  "Maybeee...",                        // yes
  "Definitely!",                       // yes
  "It is decidedly so.",               // yes
  "My reply is no.",                   // no
  "My sources confirms that the answer is no.\nSource: :sparkles: *i made it up* :sparkles:", // no
  "As I see it, yes.",                 // yes
  "Don't count on it.",                // no
  "Whoa! Why do I have to answer this?", // no
  "Highly unlikely.",                  // no
  "Sure, but with extreme cautions.",  // yes
  "What kind of stupid question is that?? No! I'm not answering that!", // no
  "Try asking this to a chicken. Probably knows it better than I do!", // no
  "Not in a million years!",           // no
  "As a matter of fact, yes.",         // yes
  "It's a no, better go ask someone else.", // no
  "In the end, it's not a bad choice.", // yes
  "Nope, not today.",                  // no
  "Cross your fingers, the answer is yes!", // yes
  "Nope. *shakes head*",               // no
  "The fortune cookie said yes.",      // yes
  "Sorry, the fortune cookie over there said no.", // no
  "Sorry, not happening.",             // no
  "I'll have to consult my sources... *flips coin*... no.", // no
  "I'll have to consult the magic 8-ball... *shakes*... no.", // no
  "I'm not sure to be honest, let's ask your friend. Oh wait...", // no
  "This question flew over my head, I'll pass.", // no
  "Oops, the Magic 8-Ball shattered itself when you asked that! I'll take that as a no."  // no
];

const CHICKEN_RESPONSES: [&str; 54] = [
  "Cluck cluck... Reply hazy, try pecking Google.",          // no
  "Meh... Figure it out yourself, or scratch around a bit.", // no
  "I don't know... what do you think? *pecks at ground*",    // no
  "BAWK! YES!",                                              // yes
  "Cluck... no.",                                            // no
  "It is decidedly so! *flaps wings*",                       // yes
  "Signs point to... maybe... hold on, let me fluff my feathers... depends on whether you'd get to know your Magic Chicken a bit better.", // no
  "Signs point to... ~~yes~~ cluck no.",                     // no
  "Why do you want to know? It's a big cluckin' yes!",       // yes
  "Outlook not so clucking good.",                           // no
  "Outlook cluckin' hazy.",                                  // no
  "What are you, a lost chick? Cluck!",                      // no
  "How the cluck do you not know that?",                     // no
  "Really? Asking a chicken to decide your fate? *clucks judgmentally*", // no
  "Peck back later, I'm nesting...",                         // no
  "I don't know, try flapping your wings and ask again?",    // no
  "The answer is a big ol' yes! *flaps happily*",            // yes
  "Yes... wait, actually... no. Cluck, I'm confused.",       // no
  "Maaaaybe... *chicken waddle*?",                           // yes
  "Definitely! *struts confidently*",                        // yes
  "It is decidedly so. *struts with pride*",                 // yes
  "My reply is a solid *cluck* no.",                         // no
  "My sources confirm it's a cluckin' no.\nSource: 🐔 *I made it up* 🐔", // no
  "As I see it, yes! *pecks approvingly*",                   // yes
  "Don't count on it. *cluck cluck*",                        // no
  "Whoa, why do I have to answer this? *fluffs feathers*",   // no
  "Highly unlikely. *chicken stare*",                        // no
  "Sure, but with extreme cluckin' caution.",                // yes
  "What kind of stupid question is that?? No! *angry clucks*", // no
  "Try asking this to a fellow chicken. They probably know better than I do!", // no
  "Cluck yes! *does a happy chicken dance*",                 // yes
  "No way, not even for a big bag of feed.",                 // no
  "Yes! *lays egg of approval*",                             // yes
  "It's a no, better go scratch somewhere else.",            // no
  "Cluck-tastic! That's a definite yes.",                    // yes
  "Cluck yeah! *struts proudly*",                            // yes
  "Nope, not today. *shakes head*",                          // no
  "Feathers crossed, the answer is yes!",                    // yes
  "Chicken says nope. *tilts head*",                         // no
  "Absolutely! *clucks happily*",                            // yes
  "Not a chance. *fluffs feathers*",                         // no
  "Eggcellent choice! Yes!",                                 // yes
  "Not in a million clucks!",                                // no
  "As a matter of cluck, yes! *clucks approvingly*",         // yes
  "It's a nopity nope, better go ask another chicken.",      // no
  "In the end, it's not a bad cluck",                        // yes
  "Nope, not today. *clucks sadly*",                         // no
  "Cross your feathers, the answer is yes!",                 // yes
  "The fortune cookie said yes. *clucks in agreement*",      // yes
  "Sorry, the fortune cookie over there said no. *clucks in disagreement*", // no
  "I'll have to consult my sources... *flips corn*... no.",  // no
  "I'll have to consult the magic 8-cluck... *shakes*... no.", // no
  "I'm not sure to be honest, let's ask your chicken friend. Oh wait...", // no
  "This question floated over my head, I'll pass. *clucks dismissively*"  // no
];

const CHAOTIC_RESPONSES: [&str; 90] = [
  "Oops! The Magic 8-Ball shattered upon hearing your question. Coincidence?", // no
  "Reply hazy. Ask Google’s evil twin, Froogle.",                              // no
  "Meh — Consult the ancient texts of Netflix subtitles.",                     // no
  "I don't know, but your cat probably does.",                                 // no
  "Yes, but only if you wear a clown wig.",                                    // yes
  "No. Unless the moon winks at you first.",                                   // no
  "It is decidedly a resounding honk-honk!",                                   // yes
  "Signs point to... maybe... or not... or wait... oh look, a squirrel!",      // no
  "Signs point to... ~~yes~~ pancakes. Definitely pancakes.",                  // no
  "Why do you want to know? It’s obviously a yes — trust the donut prophecy.", // yes
  "Outlook not so good. Blame Mercury retrograde or your Wi-Fi.",              // no
  "Outlook hazy. Consult the nearest fortune-telling hamster.",                // no
  "What are you, a toaster in disguise?",                                      // no
  "How the heck do you not know this? Ask a sock puppet!",                     // no
  "Really? Making life choices based on a magic ball? Bold move, friend.",     // no
  "Try asking later... when I’m less busy binge-watching.",                    // no
  "I don't know, summon a raven and whisper your question into the void.",     // no
  "The answer is yes, as foretold by the mystical spaghetti.",                 // yes
  "Yes, actually no. Wait, yes? Let’s go with potato.",                        // no
  "Maybeee... if the stars align and your pizza has extra cheese.",            // yes
  "Definitely! Unless gravity stops working.",                                 // yes
  "It is decidedly so. So what? Buy a llama and see what happens.",            // yes
  "My reply is no, and also banana pudding.",                                  // no
  "My sources confirm that the answer is no.\nSource: A suspicious pigeon.",   // no
  "As I see it, yes. As the chicken sees it, no. Trust who you like.",         // yes
  "Don't count on it. Count on marshmallows instead.",                         // no
  "Whoa! Why do I have to answer this? Ask a rubber duck.",                    // no
  "Highly unlikely. Unless it’s Tuesday on Mars.",                             // no
  "Sure, but with extreme caution and a tinfoil hat.",                         // yes
  "What kind of silly question is that?? No! Also, here’s a kazoo.",           // no
  "Try asking this to a chicken. They’re the true oracles.",                   // no
  "Not in a million years! Unless the earth is made of cheese.",               // no
  "As a matter of fact, yes. And it’s raining tacos.",                         // yes
  "It's a no, but the raccoons might know better.",                            // no
  "In the end, it’s not a bad choice. Or is it? Mwahaha.",                     // yes
  "Nope, not today. Try tomorrow after coffee.",                               // no
  "Cross your fingers! Or better yet, cross the streams.",                     // yes
  "Nope. *shakes head like a very judgmental parrot*",                         // no
  "The fortune cookie said yes, but it was written in crayon.",                // yes
  "Sorry, the fortune cookie over there said no. Blame it.",                   // no
  "Sorry, not happening. But you get a virtual sticker for trying!",           // no
  "I'll have to consult my sources... *flips a pancake*... no.",               // no
  "I'll have to consult the magic 8-ball... *shakes it violently*... still no.", // no
  "I'm not sure, but your imaginary friend says yes.",                         // yes
  "This question flew over my head, so I’ll just say 'llama'.",                // no
  "The answer is yes, but only if you do it while wearing socks on your hands.", // yes
  "No, and I think you broke the space-time continuum by asking.",             // no
  "Why not? What’s the worst that could happen? Oh wait...",                   // no
  "The stars say yes, but the planets are still debating.",                    // yes
  "The universe just facepalmed at your question.",                            // no
  "Ask again while juggling flaming pineapples for a clearer answer.",         // no
  "Nope, not unless you bribe me with tacos.",                                 // no
  "I consulted the oracle... she’s out to lunch. Try later.",                  // no
  "Yes, but only if you can lick your elbow right now.",                       // yes
  "No, because I said so and I’m very wise. Also, I’m a plastic ball.",        // no
  "Yes. No. Wait, I’ve lost track. Did you hear that noise?",                  // no
  "Absolutely, as long as you bring me a rubber chicken as tribute.",          // yes
  "I asked a wizard, and they just laughed hysterically.",                     // no
  "The spirits say no, but the ghosts are nodding yes.",                       // no
  "Yes, if you believe in unicorns and the power of friendship.",              // yes
  "No, and also you might want to move. Something’s behind you.",              // no
  "Ask again, but this time with interpretive dance.",                         // no
  "Definitely! Unless the moon turns into cheese. Then no.",                   // yes
  "I see... wait, no, I don’t see. My crystal ball is buffering.",             // no
  "Sure! But only after a karaoke duet with a raccoon.",                       // yes
  "Yes, but only if you promise not to tell the ducks.",                       // yes
  "No way, unless you can recite the alphabet backwards in one breath.",       // no
  "Ask the magic mushroom. It’s way more in touch with reality than I am.",    // no
  "No, because gravity disagrees with your premise.",                          // no
  "Yes, but first you must complete the sacred quest for nachos.",             // yes
  "The answer is hidden in the folds of your laundry. Go check.",              // no
  "I would answer, but I’m legally obligated to stay mysterious.",             // no
  "Absolutely! If you can solve this riddle: What walks on four legs in the morning, two legs at noon, and... oh wait, wrong universe.", // yes
  "The council of frogs says yes, but only if you croak like one.",            // yes
  "No, but only because the Magic 8-Ball union forbids it.",                   // no
  "Yes, if the dog wags its tail twice before the clock strikes midnight.",    // yes
  "Try again after doing three cartwheels and making a wish.",                 // no
  "The ducks in my dreams say no. They’re rarely wrong.",                      // no
  "Not today, Satan. Not today.",                                              // no
  "Yes, but only on Wednesdays during a full moon.",                           // yes
  "No, because bananas don’t grow in winter.",                                 // no
  "The answer is locked in a time capsule. Check back in 50 years.",           // no
  "I don’t know, but it smells like trouble.",                                 // no
  "Why not? The penguins approve, and that’s good enough for me.",             // yes
  "Sure, but only if you say 'bubblegum' ten times fast.",                     // yes
  "No, unless you can outsmart a sentient toaster.",                           // no
  "The answer is yes, but it comes with a plot twist.",                        // yes
  "Flip a coin, spin three times, and consult your nearest cactus. Good luck!", // no
  "Only on the condition that you buy me a donut.",                            // yes
  "Yes, but proceed at your own risk. The llamas are watching."                // yes
];

fn get_random_response() -> &'static str { RESPONSES[random::<usize>() % RESPONSES.len()] }

fn get_random_chicken_response() -> &'static str { CHICKEN_RESPONSES[random::<usize>() % CHICKEN_RESPONSES.len()] }

fn get_random_chaotic_response() -> &'static str { CHAOTIC_RESPONSES[random::<usize>() % CHAOTIC_RESPONSES.len()] }
