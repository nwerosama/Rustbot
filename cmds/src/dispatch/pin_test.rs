use {
  poise::{
    CreateReply,
    serenity_prelude::{
      CreateAttachment,
      MessageId
    }
  },
  rustbot_lib::{
    RustbotContext,
    RustbotResult
  }
};

/// .
#[poise::command(slash_command, subcommands("pin", "unpin", "all"))]
pub async fn pin_test(_: RustbotContext<'_>) -> RustbotResult<()> { Ok(()) }

/// .
#[poise::command(slash_command)]
async fn pin(
  ctx: RustbotContext<'_>,
  msg_id: String
) -> RustbotResult<()> {
  ctx
    .http()
    .pin_message(ctx.channel_id(), MessageId::new(msg_id.parse::<u64>().unwrap()), None)
    .await?;
  ctx.reply("Pinned a message!").await?;
  Ok(())
}

/// .
#[poise::command(slash_command)]
async fn unpin(
  ctx: RustbotContext<'_>,
  msg_id: String
) -> RustbotResult<()> {
  ctx
    .http()
    .unpin_message(ctx.channel_id(), MessageId::new(msg_id.parse::<u64>().unwrap()), None)
    .await?;
  ctx.reply("Unpinned a message!").await?;
  Ok(())
}

/// .
#[poise::command(slash_command)]
async fn all(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  let pins = ctx.http().get_pins(ctx.channel_id()).await?;
  let mut verbose = Vec::new();
  for (count, msg) in pins.iter().enumerate() {
    verbose.push(format!("{count}: {msg:#?}"))
  }
  let f = verbose.join("\n\n");
  let bytes = f.as_bytes().to_vec();
  ctx
    .send(
      CreateReply::default()
        .content("Here's the verbose details in a file!")
        .attachment(CreateAttachment::bytes(bytes, "v.log"))
    )
    .await?;
  Ok(())
}
