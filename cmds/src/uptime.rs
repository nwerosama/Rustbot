use {
  asahi::utils::{
    format_bytes,
    format_duration,
    os::{
      get_kernel_info,
      get_memory,
      get_os_info,
      get_uptime
    }
  },
  rustbot_lib::{
    RustbotContext,
    RustbotResult,
    utils::{
      BOT_VERSION,
      GIT_COMMIT_BRANCH,
      GIT_COMMIT_HASH
    }
  },
  std::{
    env::var,
    process::Command
  },
  sysinfo::System
};

/// Retrieve host and bot uptimes
#[poise::command(slash_command)]
pub async fn uptime(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  let bot_name = ctx.cache().current_user().name.clone();
  let mut sys = System::new_all();
  sys.refresh_all();

  // Fetch system's processor
  let cpu = sys.cpus();

  // Fetch system and process memory usage
  let memory = get_memory();
  let (pram, sram, sram_total) = (
    format_bytes(memory.process),
    format_bytes(memory.system.used),
    format_bytes(memory.system.total)
  );

  // Fetch the node hostname from envvar
  let node_hostname = match var("DOCKER_HOSTNAME") {
    Ok(h) => h.to_string(),
    Err(_) => String::from_utf8(Command::new("hostname").output().unwrap().stdout)
      .unwrap()
      .trim()
      .to_string()
  };

  let stat_msg = [
    format!("**{} {}** `{GIT_COMMIT_HASH}:{GIT_COMMIT_BRANCH}`", bot_name, BOT_VERSION.as_str()),
    format!(">>> System: `{}`", format_duration(get_uptime().system)),
    format!("Process: `{}`", format_duration(get_uptime().process)),
    format!("Node: `{node_hostname}`"),
    format!("CPU: `{}`", cpu[0].brand()),
    format!("RAM: `{pram}` (`{sram}/{sram_total}`)"),
    format!("OS: `{}`", get_os_info()),
    format!("Kernel: `{}`", get_kernel_info())
  ];
  ctx.reply(stat_msg.join("\n")).await?;

  Ok(())
}
