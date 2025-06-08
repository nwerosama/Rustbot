use {
  poise::{
    CreateReply,
    serenity_prelude::{
      ButtonStyle,
      ComponentInteractionCollector,
      CreateActionRow,
      CreateButton,
      CreateInteractionResponse,
      CreateInteractionResponseMessage
    }
  },
  rustbot_lib::{
    RustbotContext,
    RustbotResult
  }
};

/// .
#[poise::command(slash_command)]
pub async fn edit_test(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  ctx
    .send(
      CreateReply::new()
        .content("Ephemeral message text")
        .components(&[CreateActionRow::Buttons(
          vec![CreateButton::new("edit-btn").label("Edit").style(ButtonStyle::Primary)].into()
        )])
        .ephemeral(true)
    )
    .await?;

  while let Some(int) = ComponentInteractionCollector::new(ctx.serenity_context())
    .timeout(std::time::Duration::from_secs(120))
    .filter(move |int| int.data.custom_id == "edit-btn")
    .next()
    .await
  {
    int
      .create_response(
        ctx.http(),
        CreateInteractionResponse::UpdateMessage(
          CreateInteractionResponseMessage::new()
            .content("The newly edited ephemeral message text")
            .components(Vec::new())
        )
      )
      .await?;
  }

  Ok(())
}
