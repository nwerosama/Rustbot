use crate::Error;

use poise::serenity_prelude::UserId;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::io::Write;

const WHITELISTED_USERS: &[UserId] = &[
  UserId(190407856527376384)
];

/// Evaluate a piece of code
#[poise::command(slash_command)]
pub async fn eval(
  ctx: poise::Context<'_, (), Error>,
  #[description = "The Rust code to evaluate"] code: String
) -> Result<(), Error> {
  if !WHITELISTED_USERS.contains(&ctx.author().id) {
    ctx.reply("Whitelisted users can only use this command!").await?;
    return Ok(());
  }

  // Create a temp directory
  let dir = tempfile::tempdir()?;
  let file_path = dir.path().join("temp.rs");

  {
    let mut file = std::fs::File::create(&file_path)?;
    writeln!(file, "fn main() {{ {} }}", code)?;
  }

  // Compile
  let compiled_path = dir.path().join("temp");
  let output = Command::new("rustc").arg(&file_path).arg("-o").arg(&compiled_path).output()?;

  if !output.status.success() {
    ctx.reply(format!("Compilation failed:\n```{}```", String::from_utf8_lossy(&output.stderr))).await?;
    return Ok(());
  }

  // Update binary's permissions before execution stage
  let permissions = std::fs::Permissions::from_mode(0o755);
  let compiled_path = dir.path().join("temp");
  std::fs::set_permissions(&compiled_path, permissions)?;

  // If success, run it.
  let output = Command::new(compiled_path).output()?;

  if !output.status.success() {
    ctx.reply(format!("Execution failed:\n```{}```", String::from_utf8_lossy(&output.stderr))).await?;
    return Ok(());
  }

  ctx.reply(format!("Code output:\n```rs\n{}```", String::from_utf8_lossy(&output.stdout))).await?;
  Ok(())
}
