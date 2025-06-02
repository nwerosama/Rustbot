use {
  asahi::error,
  poise::FrameworkError,
  rustbot_lib::{
    RustbotData,
    RustbotError,
    utils::mention_dev
  }
};

pub async fn fw_errors(error: FrameworkError<'_, RustbotData, RustbotError>) {
  match error {
    FrameworkError::Command { error, ctx, .. } => {
      error!("PoiseCommandError({}): {error}", ctx.command().qualified_name);
      ctx
        .reply(format!(
          "Encountered an error during command execution, ask {} to check console for more details!",
          mention_dev(ctx).unwrap_or_default()
        ))
        .await
        .expect("Error sending message");
    },
    FrameworkError::NotAnOwner { ctx, .. } => {
      error!(
        "PoiseNotAnOwner: {} tried to execute a developer-level command ({})",
        ctx.author().name,
        ctx.command().qualified_name
      );
      ctx
        .reply("Whoa, you discovered a hidden command! Too bad, I can't allow you to execute it as you're not my creator.")
        .await
        .expect("Error sending message");
    },
    FrameworkError::UnknownInteraction { interaction, .. } => error!(
      "PoiseUnknownInteractionError: {} tried to execute an unknown interaction ({})",
      interaction.user.name, interaction.data.name
    ),
    FrameworkError::UnknownCommand { msg, .. } => error!(
      "PoiseUnknownCommandError: {} tried to execute an unknown command ({})",
      msg.author.name, msg.content
    ),
    FrameworkError::ArgumentParse { ctx, error, .. } => {
      error!("PoiseArgumentParseError: {error}");
      ctx
        .reply(format!("Error parsing argument(s): {error}"))
        .await
        .expect("Error sending message");
    },
    FrameworkError::CommandPanic { ctx, payload, .. } => {
      if let Some(payload) = payload.clone() {
        error!("PoiseCommandPanic: {payload}");
        ctx
          .reply(format!(
            "The command panicked, please tell my developer about this!\n**Error:**```\n{payload}\n```"
          ))
          .await
          .expect("Error sending message");
      } else {
        error!("PoiseCommandPanic: No payload provided");
        ctx
          .reply(
            [
              "Well, this is concerning... Hopefully you notified my developer about this!",
              "The command panicked, but didn't leave any trace behind... Suspicious!"
            ]
            .join("\n")
          )
          .await
          .expect("Error sending message");
      }
    },
    other => error!("PoiseOtherError: {other}")
  }
}
