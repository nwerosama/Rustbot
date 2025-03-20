mod events;

pub use events::RustbotEvents;

// use serde_json::json;
/* use poise::serenity_prelude::{
  Context,
  WebhookId
}; */

/* async fn hook_logger(
  ctx: &Context,
  hook_id: WebhookId,
  token: &str,
  content: String
) -> Result<bool, rustbot_lib::RustbotError> {
  let current_app = ctx.http.get_current_user().await.unwrap();
  let bot_avatar = current_app.avatar_url().unwrap();
  let bot_username = &current_app.name;

  if let Err(e) = ctx.http.execute_webhook(
    hook_id,
    None,
    token,
    true,
    vec![],
    &json!({
      "content": content,
      "avatar_url": bot_avatar,
      "username": bot_username
    })
  ).await {
    println!("{}[EventWebhook]: Failed to send webhook message: {e}", events::RUSTBOT_EVENT);
    Ok(false)
  } else {
    Ok(true)
  }
} */
