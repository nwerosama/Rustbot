use {
  asahi::utils::{
    format_duration,
    os::{
      format_bytes,
      get_kernel_info,
      get_os_info
    }
  },
  rustbot_lib::{
    RustbotContext,
    RustbotResult,
    config::BINARY_PROPERTIES,
    utils::{
      BOT_VERSION,
      GIT_COMMIT_BRANCH,
      GIT_COMMIT_HASH
    }
  },
  std::{
    env::var,
    time::{
      Duration,
      SystemTime,
      UNIX_EPOCH
    }
  },
  sysinfo::System,
  uptime_lib::get
};

/// Retrieve host and bot uptimes
#[poise::command(slash_command)]
pub async fn uptime(ctx: RustbotContext<'_>) -> RustbotResult<()> {
  let bot = ctx.http().get_current_user().await.unwrap();
  let mut sys = System::new_all();
  sys.refresh_all();

  // Fetch system's uptime
  let sys_uptime = get().unwrap().as_secs();

  // Fetch system's processor
  let cpu = sys.cpus();

  // Fetch system memory usage
  let sram = format_bytes(sys.used_memory());
  let sram_total = format_bytes(sys.total_memory());

  // Fetch process memory usage
  let pram = match sys.process(sysinfo::get_current_pid().unwrap()) {
    Some(proc) => format_bytes(proc.memory()),
    None => String::from("Unavailable")
  };

  // Fetch process uptime
  let curr_pid = sysinfo::get_current_pid().unwrap();
  let now = SystemTime::now();
  let mut proc_uptime = 0;
  if let Some(process) = sys.process(curr_pid) {
    let time_started = UNIX_EPOCH + Duration::from_secs(process.start_time());
    proc_uptime = now.duration_since(time_started).unwrap().as_secs();
  }

  // Fetch the node hostname from envvar
  let node_hostname = if BINARY_PROPERTIES.env.contains("prod") {
    match var("DOCKER_HOSTNAME") {
      Ok(h) => h.to_string(),
      Err(_) => "DOCKER_HOSTNAME is empty!".to_string()
    }
  } else {
    let hostname = std::process::Command::new("hostname").output().unwrap().stdout;
    String::from_utf8(hostname).unwrap().trim().to_string()
  };

  let stat_msg = [
    format!("**{} v{}** `{GIT_COMMIT_HASH}:{GIT_COMMIT_BRANCH}`", bot.name, *BOT_VERSION),
    format!(">>> System: `{}`", format_duration(sys_uptime)),
    format!("Process: `{}`", format_duration(proc_uptime)),
    format!("Node: `{node_hostname}`"),
    format!("CPU: `{}`", cpu[0].brand()),
    format!("RAM: `{pram}` (`{sram}/{sram_total}`)"),
    format!("OS: `{}`", get_os_info()),
    format!("Kernel: `{}`", get_kernel_info())
  ];
  ctx.reply(stat_msg.join("\n")).await?;

  Ok(())
}
