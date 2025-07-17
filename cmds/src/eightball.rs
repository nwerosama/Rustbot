use {
  bytes::Bytes,
  poise::{
    CreateReply,
    serenity_prelude::{
      CreateAttachment,
      UserId
    }
  },
  rand::random,
  rustbot_lib::{
    RustbotContext,
    RustbotResult
  },
  std::fmt::{
    Display,
    Formatter,
    Result
  }
};

#[derive(poise::ChoiceParameter, Clone, Eq, PartialEq, Hash)]
enum ResponseMode {
  Normal,
  Chicken,
  #[name = "Chaotic & Unhinged"]
  Chaotic,
  #[name = "UwU"]
  Uwu,
  Femboy
}

impl Display for ResponseMode {
  fn fmt(
    &self,
    f: &mut Formatter<'_>
  ) -> Result {
    write!(f, "{}", self.display_name())
  }
}

impl ResponseMode {
  fn display_name(&self) -> &'static str {
    match self {
      Self::Normal => "Normal",
      Self::Chicken => "Chicken",
      Self::Chaotic => "Chaotic & Unhinged",
      Self::Uwu => "UwU",
      Self::Femboy => "Femboy"
    }
  }

  fn filename(&self) -> &'static str {
    match self {
      Self::Normal => "responses.txt",
      Self::Chicken => "chicken_responses.txt",
      Self::Chaotic => "chaotic_responses.txt",
      Self::Uwu => "uwu_responses.txt",
      Self::Femboy => "femboy_responses.txt"
    }
  }

  fn responses(&self) -> &'static [(&'static str, bool)] {
    match self {
      Self::Normal => &RESPONSES,
      Self::Chicken => &CHICKEN_RESPONSES,
      Self::Chaotic => &CHAOTIC_RESPONSES,
      Self::Uwu => &UWU_RESPONSES,
      Self::Femboy => &FEMBOY_RESPONSES
    }
  }

  fn get_random_response(&self) -> &'static str {
    let responses = self.responses();
    responses[random::<u32>() as usize % responses.len()].0
  }
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
  if question.to_ascii_lowercase().contains("rustbot, show list") {
    show_list(ctx, mode.clone().unwrap_or(ResponseMode::Normal)).await?;
    return Ok(())
  }

  let mode = mode.unwrap_or(ResponseMode::Normal);
  let rand_resp = mode.get_random_response();

  ctx.reply(format!("> {question}\n{rand_resp}")).await?;

  Ok(())
}

async fn show_list(
  ctx: RustbotContext<'_>,
  list_type: ResponseMode
) -> RustbotResult<()> {
  if ctx.author().id != UserId::new(ctx.data().config.developers[0]) {
    ctx
      .reply("The list knows you're looking, but it's playing a game of hide and seek. For now, it wins.")
      .await?;
    return Ok(());
  }

  let selected_responses = list_type.responses();

  let total_yes = selected_responses.iter().filter(|&&(_, yes)| yes).count();
  let total_no = selected_responses.len() - total_yes;
  let response_strings: Vec<String> = selected_responses.iter().map(|&(resp, _)| resp.to_string()).collect();
  let response_mode = format!("Response mode: {list_type}");

  let content = [
    response_mode.clone(),
    "Totals:".to_string(),
    format!(" > Yes: {total_yes}"),
    format!(" > No: {total_no}"),
    format!(" > Strings: {}", selected_responses.len()),
    "-".to_string().repeat(response_mode.len()),
    response_strings.join("\n")
  ]
  .join("\n");

  ctx
    .send(CreateReply::new().attachment(CreateAttachment::bytes(Bytes::from(content), list_type.filename())))
    .await?;

  Ok(())
}

const RESPONSES: [(&str, bool); 80] = [
  ("Reply hazy. Look it up on Google.", false),
  ("Meh — Figure it out yourself.", false),
  ("I don't know, what do you think?", false),
  ("Yes.", true),
  ("No.", false),
  ("It is decidedly so", true),
  (
    "Signs point to... maybe... depends on... hold on, let me get my glasses, this is getting pretty tiny... depends on whether you'd be up to \
     getting to know your Magic 8-Ball a little better.",
    false
  ),
  ("Signs point to... ~~yes~~ no.", false),
  ("Why do you want to know the answer? It's obviously a yes.", true),
  ("Outlook not so good.", false),
  ("Outlook hazy.", false),
  ("What are you, stupid?", false),
  ("How the hell do you not know that?", false),
  ("Really? Making a decision based on what the plastic 8-Ball says? Jesus...", false),
  ("Try asking later...", false),
  ("I don't know, whip out the ouija board and try again?", false),
  ("The answer is yes.", true),
  ("Yes, actually no. Wait, nevermind.", false),
  ("Maybeee...", true),
  ("Definitely!", true),
  ("It is decidedly so.", true),
  ("My reply is no.", false),
  (
    "My sources confirms that the answer is no.\nSource: :sparkles: *i made it up* :sparkles;",
    false
  ),
  ("As I see it, yes.", true),
  ("Don't count on it.", false),
  ("Whoa! Why do I have to answer this?", false),
  ("Highly unlikely.", false),
  ("Sure, but with extreme cautions.", true),
  ("What kind of stupid question is that?? No! I'm not answering that!", false),
  ("Try asking this to a chicken. Probably knows it better than I do!", false),
  ("Not in a million years!", false),
  ("As a matter of fact, yes.", true),
  ("It's a no, better go ask someone else.", false),
  ("In the end, it's not a bad choice.", true),
  ("Nope, not today.", false),
  ("Cross your fingers, the answer is yes!", true),
  ("Nope. *shakes head*", false),
  ("The fortune cookie said yes.", true),
  ("Sorry, the fortune cookie over there said no.", false),
  ("Sorry, not happening.", false),
  ("I'll have to consult my sources... *flips coin*... no.", false),
  ("I'll have to consult the magic 8-ball... *shakes*... no.", false),
  ("I'm not sure to be honest, let's ask your friend. Oh wait...", false),
  ("This question flew over my head, I'll pass.", false),
  (
    "Oops, the Magic 8-Ball shattered itself when you asked that! I'll take that as a no.",
    false
  ),
  ("Absolutely not, but keep trying!", false),
  ("Yes, in a galaxy far far away!", true),
  ("Ask yourself and find the answer.", false),
  ("The stars align for a yes.", true),
  ("Chances are slim, but possible.", false),
  ("Without a doubt, the universe agrees.", true),
  ("Better not tell you now.", false),
  ("Certainly in your favor.", true),
  ("Look within for the answer.", false),
  ("Everything points to yes!", true),
  ("Mmm... yeah, I would say so~", true),
  ("Pfft, nope. Not a chance.", false),
  ("Absolutely! Go for it, bestie~", true),
  ("Not feeling it... sorry bud.", false),
  ("Yes, but only if you believe it!", true),
  ("Oof... no. Big no.", false),
  ("Heck yeah!!", true),
  ("Nahhh, that's sus.", false),
  ("Totally! Like, 100%!", true),
  ("Ehhh... probs not.", false),
  ("With every fiber of my being: YES", true),
  ("LOL no. Just no.", false),
  ("Affirmative, captain!", true),
  ("Negative. Abort mission.", false),
  ("Yeppers!", true),
  ("Nopers!", false),
  ("Yasss queen, do it!!", true),
  ("Girl, no.", false),
  ("It's written in the stars: yes", true),
  ("The stars are silent... I'd take that as a no.", false),
  ("Yes, based on current information.", true),
  ("Yes, it's a logical conclusion.", true),
  ("No, it doesn't follow logically.", false),
  ("Yes. Proceed as planned.", true),
  ("No. Do not proceed.", false)
];

const CHICKEN_RESPONSES: [(&str, bool); 54] = [
  ("Cluck cluck... Reply hazy, try pecking Google.", false),
  ("Meh... Figure it out yourself, or scratch around a bit.", false),
  ("I don't know... what do you think? *pecks at ground*", false),
  ("BAWK! YES!", true),
  ("Cluck... no.", false),
  ("It is decidedly so! *flaps wings*", true),
  (
    "Signs point to... maybe... hold on, let me fluff my feathers... depends on whether you'd get to know your Magic Chicken a bit better.",
    false
  ),
  ("Signs point to... ~~yes~~ cluck no.", false),
  ("Why do you want to know? It's a big cluckin' yes!", true),
  ("Outlook not so clucking good.", false),
  ("Outlook cluckin' hazy.", false),
  ("What are you, a lost chick? Cluck!", false),
  ("How the cluck do you not know that?", false),
  ("Really? Asking a chicken to decide your fate? *clucks judgmentally*", false),
  ("Peck back later, I'm nesting...", false),
  ("I don't know, try flapping your wings and ask again?", false),
  ("The answer is a big ol' yes! *flaps happily*", true),
  ("Yes... wait, actually... no. Cluck, I'm confused.", false),
  ("Maaaaybe... *chicken waddle*?", true),
  ("Definitely! *struts confidently*", true),
  ("It is decidedly so. *struts with pride*", true),
  ("My reply is a solid *cluck* no.", false),
  ("My sources confirm it's a cluckin' no.\nSource: 🐔 *I made it up* 🐔", false),
  ("As I see it, yes! *pecks approvingly*", true),
  ("Don't count on it. *cluck cluck*", false),
  ("Whoa, why do I have to answer this? *fluffs feathers*", false),
  ("Highly unlikely. *chicken stare*", false),
  ("Sure, but with extreme cluckin' caution.", true),
  ("What kind of stupid question is that?? No! *angry clucks*", false),
  ("Try asking this to a fellow chicken. They probably know better than I do!", false),
  ("Cluck yes! *does a happy chicken dance*", true),
  ("No way, not even for a big bag of feed.", false),
  ("Yes! *lays egg of approval*", true),
  ("It's a no, better go scratch somewhere else.", false),
  ("Cluck-tastic! That's a definite yes.", true),
  ("Cluck yeah! *struts proudly*", true),
  ("Nope, not today. *shakes head*", false),
  ("Feathers crossed, the answer is yes!", true),
  ("Chicken says nope. *tilts head*", false),
  ("Absolutely! *clucks happily*", true),
  ("Not a chance. *fluffs feathers*", false),
  ("Eggcellent choice! Yes!", true),
  ("Not in a million clucks!", false),
  ("As a matter of cluck, yes! *clucks approvingly*", true),
  ("It's a nopity nope, better go ask another chicken.", false),
  ("In the end, it's not a bad cluck", true),
  ("Nope, not today. *clucks sadly*", false),
  ("Cross your feathers, the answer is yes!", true),
  ("The fortune cookie said yes. *clucks in agreement*", true),
  ("Sorry, the fortune cookie over there said no. *clucks in disagreement*", false),
  ("I'll have to consult my sources... *flips corn*... no.", false),
  ("I'll have to consult the magic 8-cluck... *shakes*... no.", false),
  ("I'm not sure to be honest, let's ask your chicken friend. Oh wait...", false),
  ("This question floated over my head, I'll pass. *clucks dismissively*", false)
];

const CHAOTIC_RESPONSES: [(&str, bool); 143] = [
  ("Oops! The Magic 8-Ball shattered upon hearing your question. Coincidence?", false),
  ("Reply hazy. Ask Google's evil twin, Froogle.", false),
  ("Meh — Consult the ancient texts of Netflix subtitles.", false),
  ("I don't know, but your cat probably does.", false),
  ("Yes, but only if you wear a clown wig.", true),
  ("No. Unless the moon winks at you first.", false),
  ("It is decidedly a resounding honk-honk!", true),
  ("Signs point to... maybe... or not... or wait... oh look, a squirrel!", false),
  ("Signs point to... ~~yes~~ pancakes. Definitely pancakes.", false),
  ("Why do you want to know? It's obviously a yes — trust the donut prophecy.", true),
  ("Outlook not so good. Blame Mercury retrograde or your Wi-Fi.", false),
  ("Outlook hazy. Consult the nearest fortune-telling hamster.", false),
  ("What are you, a toaster in disguise?", false),
  ("How the heck do you not know this? Ask a sock puppet!", false),
  ("Really? Making life choices based on a magic ball? Bold move, friend.", false),
  ("Try asking later... when I'm less busy binge-watching.", false),
  ("I don't know, summon a raven and whisper your question into the void.", false),
  ("The answer is yes, as foretold by the mystical spaghetti.", true),
  ("Yes, actually no. Wait, yes? Let's go with potato.", false),
  ("Maybeee... if the stars align and your pizza has extra cheese.", true),
  ("Definitely! Unless gravity stops working.", true),
  ("It is decidedly so. So what? Buy a llama and see what happens.", true),
  ("My reply is no, and also banana pudding.", false),
  ("My sources confirm that the answer is no.\nSource: A suspicious pigeon.", false),
  ("As I see it, yes. As the chicken sees it, no. Trust who you like.", true),
  ("Don't count on it. Count on marshmallows instead.", false),
  ("Whoa! Why do I have to answer this? Ask a rubber duck.", false),
  ("Highly unlikely. Unless it's Tuesday on Mars.", false),
  ("Sure, but with extreme caution and a tinfoil hat.", true),
  ("What kind of silly question is that?? No! Also, here's a kazoo.", false),
  ("Try asking this to a chicken. They're the true oracles.", false),
  ("Not in a million years! Unless the earth is made of cheese.", false),
  ("As a matter of fact, yes. And it's raining tacos.", true),
  ("It's a no, but the raccoons might know better.", false),
  ("In the end, it's not a bad choice. Or is it? Mwahaha.", true),
  ("Nope, not today. Try tomorrow after coffee.", false),
  ("Cross your fingers! Or better yet, cross the streams.", true),
  ("Nope. *shakes head like a very judgmental parrot*", false),
  ("The fortune cookie said yes, but it was written in crayon.", true),
  ("Sorry, the fortune cookie over there said no. Blame it.", false),
  ("Sorry, not happening. But you get a virtual sticker for trying!", false),
  ("I'll have to consult my sources... *flips a pancake*... no.", false),
  ("I'll have to consult the magic 8-ball... *shakes it violently*... still no.", false),
  ("I'm not sure, but your imaginary friend says yes.", true),
  ("This question flew over my head, so I'll just say 'llama'.", false),
  ("The answer is yes, but only if you do it while wearing socks on your hands.", true),
  ("No, and I think you broke the space-time continuum by asking.", false),
  ("Why not? What's the worst that could happen? Oh wait...", false),
  ("The stars say yes, but the planets are still debating.", true),
  ("The universe just facepalmed at your question.", false),
  ("Ask again while juggling flaming pineapples for a clearer answer.", false),
  ("Nope, not unless you bribe me with tacos.", false),
  ("I consulted the oracle... she's out to lunch. Try later.", false),
  ("Yes, but only if you can lick your elbow right now.", true),
  ("No, because I said so and I'm very wise. Also, I'm a plastic ball.", false),
  ("Yes. No. Wait, I've lost track. Did you hear that noise?", false),
  ("Absolutely, as long as you bring me a rubber chicken as tribute.", true),
  ("I asked a wizard, and they just laughed hysterically.", false),
  ("The spirits say no, but the ghosts are nodding yes.", false),
  ("Yes, if you believe in unicorns and the power of friendship.", true),
  ("No, and also you might want to move. Something's behind you.", false),
  ("Ask again, but this time with interpretive dance.", false),
  ("Definitely! Unless the moon turns into cheese. Then no.", true),
  ("I see... wait, no, I don't see. My crystal ball is buffering.", false),
  ("Sure! But only after a karaoke duet with a raccoon.", true),
  ("Yes, but only if you promise not to tell the ducks.", true),
  ("No way, unless you can recite the alphabet backwards in one breath.", false),
  ("Ask the magic mushroom. It's way more in touch with reality than I am.", false),
  ("No, because gravity disagrees with your premise.", false),
  ("Yes, but first you must complete the sacred quest for nachos.", true),
  ("The answer is hidden in the folds of your laundry. Go check.", false),
  ("I would answer, but I'm legally obligated to stay mysterious.", false),
  ("The council of cats says yes, but only if you meow like one.", true),
  ("No, but only because the Magic 8-Ball union forbids it.", false),
  ("Yes, if the dog wags its tail twice before the clock strikes midnight.", true),
  ("Try again after doing three cartwheels and making a wish.", false),
  ("The ducks in my dreams say no. They're rarely wrong.", false),
  ("Not today, Satan. Not today.", false),
  ("Yes, but only on Wednesdays during a full moon.", true),
  ("No, because bananas don't grow in winter.", false),
  ("The answer is locked in a time capsule. Check back in 50 years.", false),
  ("I don't know, but it smells like trouble.", false),
  ("Why not? The penguins approve, and that's good enough for me.", true),
  ("Sure, but only if you say 'bubblegum' ten times fast.", true),
  ("No, unless you can outsmart a sentient toaster.", false),
  ("The answer is yes, but it comes with a plot twist.", true),
  ("Flip a coin, spin three times, and consult your nearest cactus. Good luck!", false),
  ("Only on the condition that you buy me a donut.", true),
  ("Yes, but proceed at your own risk. The llamas are watching.", true),
  ("Yes, but only if you drive like a maniac.", true),
  ("Absolutely, and the crabs are hosting a rave.", true),
  ("No, but a mischievous gnome just winked at me, so.. yes.", true),
  ("Yes, but you'll have to fight off a pack of ninja turtles first.", true),
  ("Nope. The Magic 8-Ball union called for a strike, try later.", false),
  ("The answer is clear: definitely a turtle. Wait, what was the question again?", false),
  ("Yes, but only if you perform the forbidden kazoo solo.", true),
  ("No, because I just consulted a very grumpy cloud.", false),
  ("Sure, but beware of the ominous whispers in the wind.", true),
  ("Absolutely not! Unless you bribe me with pancakes... then maybe.", false),
  ("The prophecy foretells a yes, but only after you do a somersault.", true),
  ("Negative, captain! The squirrels have spoken.", false),
  ("Ask again after you defeat a level 70 pancake god.", false),
  ("Yes, but only if you shout 'pineapple' while hopping on one foot.", true),
  ("The stars say no, but the pigeons disagree. Trust your instincts.", false),
  ("Yes, but at great personal risk to your snack supply.", true),
  ("No way. Unless you're wearing a hat shaped like a pineapple.", false),
  ("Consult the oracle of toasters. They hold the real truth.", false),
  ("The vibes are off. Try again with a disco ball in hand.", false),
  ("Yes, but the llamas demand a dance-off first.", true),
  ("Nope. Also, why does your question smell like burnt toast?", false),
  ("Yes, but the fabric of reality might unravel. Worth it?", true),
  ("No, because an interdimensional ferret stole the answer.", false),
  ("Absolutely! But only if you chant 'banana' three times at sunrise.", true),
  ("No, unless you appease the gummy bear council with offerings.", false),
  ("Maybe, but the answer is hidden in the Great Taco of Wisdom.", false),
  ("Yes, but don't trust the robot vacuum. It knows too much.", true),
  ("I see a yes in your future... or a spaghetti monster. Hard to say.", true),
  ("Ask a sock puppet again, but this time in interpretive mime.", false),
  ("It's a no, but the goldfish thinks otherwise. Who do you trust?", false),
  ("Signs point to yes, but only if you name a star after a pineapple.", true),
  ("The answer is no. Unless it's opposite day, then yes.", false),
  ("Yes, but only if the answer doesn't involve marmalade.", true),
  ("I'm sorry, Dave, I can't do that. Oh wait, wrong universe. No.", false),
  ("The Magic 8-Ball has declared a state of confusion, try again.", false),
  ("Yes, but it might result in spontaneous interpretive breakdancing.", true),
  ("The frogs are undecided. Maybe bribe them with flies?", false),
  ("A resounding yes! Unless you're allergic to good luck.", true),
  ("No, and also stop shaking me so hard! I'm fragile!", false),
  ("Yes, but only if you can out-dance a crab at karaoke night.", true),
  ("Nope, the answer is currently orbiting Jupiter. Check back in a century.", false),
  ("ASK AGAIN—wait, no, don't! I'm on break", false),
  ("Hahahahaha.. no!", false),
  ("I'm an 8-ball, not your therapist.. But yes!", true),
  ("God has increased your difficulty to insane, 8-ball said go ahead!", true),
  ("**Error 404:** Answer not found", false),
  ("All signs to point to—ỳ̸̹͈́e̵̥̝͐̋s̶̺͍̉", true),
  ("The spirits are busy! Leave a message after the beep. *extremely loud beep*", false),
  ("Sorry, I accidentally threw your question into the trash.", false),
  ("Yes!! Now go and cause some mayhem!!!", true),
  ("Yes, you weirdo...", true),
  ("ya", true),
  (
    "Bro, you spoke some gibberish sentence to the god, he sighed and signalled **yes** back!",
    true
  ),
  ("Yes, but tell no one. THEY are listening!", true)
];

const UWU_RESPONSES: [(&str, bool); 90] = [
  ("Oopsie! The magic baww got fwightened by youw question, sowwy! >_<", false),
  ("Hmmm... wet me consuwt my pwushie cowwection... OwO it's a 'maybe?'", false),
  ("Oh nyo~ the staws awe too shy to answew >w<", false),
  ("Yes! UwU but onwy if you give me a headpat fiwst!", true),
  ("Nyope! But I bet chu' can twy again watew! >w<", false),
  ("Mmmm... I dink it's a 'pwobabwy?' OwO", true),
  ("The mysticaw fwuffbaww says 'no', but chu awe stiww amazing! >w<", false),
  ("Oh my fwuff! It's a big YES! UwU", true),
  ("No way, but chu can get a snuggwe instead! UwU", false),
  ("Hmmm... nyani?? The answew wawned away! O.O", false),
  ("Yippee! The answew is absowutewy YES, nyu~!", true),
  ("Nuuuuu, the wittwe magic is feewing shy... twy again watew, pwomise!", false),
  ("Hewwo? Hewwo? Magic baww says... oh, it feww asweep >w<", false),
  ("Maybe! But onwy if chu' twy wif youw happiest smiwe! UwU", true),
  ("No, but chu awe stiww pwetty bwoomin' paw-some!", false),
  ("The fwuffies awe undecided... ask a bun bun fwiend! >w<", false),
  ("Yes, but onwy if chu bwink wike a wittwe kitty! ^w^", true),
  ("Magic baww is wooking into it... uh-oh! It got stuck in fwuff o.O", false),
  ("Noooo, but chu can have a tummy wub to cheew chu up! UwU", false),
  ("Mmmm... I smeww a 'yes', wike fwuffy pancakies! >w<", true),
  ("Huwwo? Staws awe purring, chu' get a soft nyes~", true),
  ("Oops, chu bwew my mind wif dat one. Wet's caww it a nyope~", false),
  ("The answew is a fwuffy YES! Now go do a happy dance~ UwU", true),
  ("Nyani?? Magic baww says 'no', but chu awe stiww cute >_<", false),
  ("Hmm, I dink chu awe wooking fow... YESSS! UwU", true),
  ("Nyu-uh! But chu can twy asking a cheeky pidgeon! UwU", false),
  ("Yes! But chu must pwomise to make fwiends wif a cuddwy bunny~", true),
  ("Mmmm... nyope! Sowwy, but chu'we stiww bwoomin' pawsome! >w<", false),
  ("Yes, yes, aww the yesses in the wowwd fow chu! >w<", true),
  ("Nyooooope! But chu awe too fwuffy to be sad >w<", false),
  ("OwO it's a paw-sitive YES fwom me uwu!", true),
  ("UwU Magic fwuff says 'nyope', but gives chu a cozy bwankie!", false),
  ("M-m-maybe! Chu might need to twy a cute head tiwt fiwst~", false),
  ("Yes, UwU! But make suwe to cwink a toastie wif milkies~", true),
  ("Mmmm... I dink chu awe onto something gweat! It's YESSS! UwU", true),
  ("Nyaww, magic baww says no, but chu awe stiww adorable~ >w<", false),
  ("The staws awe wispiewing 'Yes' just fow chu! >w<", true),
  ("Oh nyo, the cosmic fwuff says 'nyope', but chu awe bwoomin' bwootiful!", false),
  ("Yes! Go fo' it wike a puppy chasing da sunshine~", true),
  ("Nuh-uh, not dis time. But chu'we so fwuffin' cute dat it's okay! UwU", false),
  ("Pawsitively YES! But pwomise chu'ww keep being pwecious~ >_<", true),
  ("Nyaww, it's a 'no', but chu awe stiww pwetty amazing! UwU", false),
  ("Yes! Go get it wike a kitty pwouncing on its favouwite toyyy! OwO", true),
  ("Hmmm... Magic fwuffies awe undeciwded. Twy a hug fiwst? >w<", false),
  ("Oh nyes! But chu must say 'Nyan nyan' to seaw da deaw!", true),
  ("Nyani?? Magic baww says nyope, but a kissy on da cheek fixes evewyfing! UwU", false),
  ("OwO The magic wainbows awe wifting chu to a 'yes'! Go fow it!", true),
  ("Oh nyo... the fwuffies awe shy dis time. Twy again wif an extwa cwute smiwe~ >w<", false),
  ("OwO yus yus! Definitewy a big YES!!", true),
  ("Uwu~ oh nyo, it wooks wike a no... sowwy!", false),
  ("OwO magic cwouds say... y-y-yesss!! *fwuffy jiggwes*", true),
  ("Oh nuuu... magic baww hid behind da couch! Gotta be a nyope~ >_<", false),
  ("Hehe~ the wittwe staws awe giggwing... dey say YES! UwU", true),
  ("Nuuu, da cosmic bunbun shaked its head... nyope.", false),
  ("UwU yesh yesh!! Da pwaynet spiwits cheew fow chu~", true),
  ("Oh nyo, da spawkwy wainbow faded... sowwy, it's a no >w<", false),
  ("OwO big uwu yesss! Go snatch yo dreamies, nya~", true),
  ("Hmmm... da fwuff got wost in a puff of gwittah... no answew yet ;w;", false),
  ("YES!!! Da snuggwy duckies agwee! Waddle on, fwend~ UwU", true),
  ("Nyope! But chu can boop my snoot anyway~ >w<", false),
  ("Da candy staws say yes! Sugaw magic activated UwU", true),
  ("Nuuuu... da tiny pawps say nyope, but chu'ww okie~", false),
  ("OwO yassss! Da meowgic inside glows bwickwy fow chu!", true),
  ("O-oh... magic went sweepy... nyope tiww watew >w<", false),
  ("Yesy yesy!! But chu must paw-mise to smiwe wide~ UwU", true),
  ("Mmm... da sky fwuffies awe undeciwded... bettew hug a teddy~", false),
  ("A paw-some YES fwom da fuwfy counciw!! ^w^", true),
  ("Nyaww... sowwy... da cookie cwuumbs said nooo... >_<", false),
  ("UwU it's a cosmic YES! Go chase yo dream like a pwayfuw kitteh~", true),
  ("Ah... da wittwe staw feww down... nyope dis time ;w;", false),
  ("OwO yesss! Da magic paw says chu got dis!", true),
  ("Nyope~ da wittwe staw shook its head~", false),
  ("Yip yip! It's a pawsitive YES!", true),
  ("Oh nuuu, da fwuffy magic says no dis time >w<", false),
  ("UwU it's a shiny YES fwom da heavens!", true),
  ("Hmm... da magic is snoozing... no answew 4 now~", false),
  ("A bouncy YES! Go chase yo dweamies, fwend~", true),
  ("Nuu, da magic said no but chu can twy again~", false),
  ("Owo da kitty purrs a soft yes!", true),
  ("Oh nyo... da bunbun hopped away wif yo answew. Guess it's a no >w<", false),
  ("Yesss! Da cosmic uwu spirits say go fow it!", true),
  ("Nuh-uh! Da stawss say wait a bit mowe~", false),
  ("UwU magic gwitters a yes just fow chu~", true),
  ("Nyope... da fwuffies awe shy dis time~", false),
  ("OwO it's a sparkwy YES, shine on fwend!", true),
  ("Awh... magic baww whispered a gentle no.", false),
  ("Yes yes yes! Da snuggwe staw gwants it!", true),
  ("Nu nu... da snoozy kitty says nyope today.", false),
  ("OwO da cloudies drew a big YES!", true),
  ("Oh... da tiny fwuffy voice said no... sowwy~", false)
];

const FEMBOY_RESPONSES: [(&str, bool); 40] = [
  ("Oh, it's cute when you ask like that~ yeah, I say yes", true),
  ("Mmm, maybe if you wink at me... but right now it's a no", false),
  ("You better do it looking that fine~ of course it's a yes", true),
  ("You're lucky I like you anyway, even if it's a no this time", false),
  ("Go show off, cutie~ absolutely yes", true),
  ("Aww~ no for now, but you can make it up to me later", false),
  ("You've already got me convinced~ that's a yes", true),
  ("Sweet talk me into it, maybe~ 'cause right now it's a no", false),
  ("Don't forget who's cheering you on~ yes, obviously", true),
  ("I'll let you buy me a drink instead~ no to the original question though", false),
  ("Come here and tell me all about it after~ yes, of course", true),
  ("At least you're still adorable~ but no, not this time", false),
  ("Big yes, babe~ no doubt about it", true),
  ("I'd still keep you around, even if it's a no", false),
  ("I expect a kiss on the cheek as thanks~ that's a yes from me", true),
  ("That smile's making me reconsider... but still a no for now", false),
  ("You're too cute for me to say no~ so yeah, it's a yes", true),
  ("You can make it up with a cuddle~ since it's a no today", false),
  ("Don't leave me hanging, sweetheart~ yes, absolutely", true),
  ("You're still my favorite troublemaker~ but it's a no right now", false),
  ("I'll be waiting to see how cute you look doing it~ that's a yes", true),
  ("If you ask nicely… maybe I'd say yes… but no for now", false),
  ("That's a yes, darling~ you didn't even need to ask", true),
  ("Hey, you're still the cutest in the room~ but no this time", false),
  ("Maybe I'll let you celebrate with me later~ because yes", true),
  ("I'll let you make it up to me with a hug~ since it's a no", false),
  ("Now go show them what you've got, babe~ absolutely yes", true),
  ("You're way too adorable to stay mad at~ but no, sorry", false),
  ("I'll be watching every second, sweetheart~ yes, you got this", true),
  ("Flash that smile again and I might change my answer… but for now, no", false),
  ("Wouldn't wanna disappoint you~ so yes", true),
  ("I'm still here cheering you on, cutie~ even if it's a no", false),
  ("Maybe I'll reward you later~ but yes for now", true),
  ("You're still doing amazing, promise~ even if it's a no", false),
  ("Only because you're so irresistible~ yeah, it's a yes", true),
  ("You're welcome to try and convince me~ but right now, it's a no", false),
  ("That's a solid yes, baby~ you've got this", true),
  ("Don't forget it~ you're still my favorite, even if it's a no", false),
  ("Now don't make me chase you for it~ yes, definitely", true),
  ("You're lucky I like you~ but still no", false)
];
