use crate::RustbotError;
use super::PoiseContext;

use sysinfo::System;
use uptime_lib::get;
use std::{
  env::var,
  fs::File,
  path::Path,
  time::{
    Duration,
    SystemTime,
    UNIX_EPOCH
  },
  io::{
    BufRead,
    BufReader
  }
};
use rustbot_lib::utils::{
  BOT_VERSION,
  GIT_COMMIT_HASH,
  GIT_COMMIT_BRANCH,
  format_duration
};

fn get_os_info() -> String {
  let path = Path::new("/etc/os-release");
  let mut name = "BoringOS".to_string();
  let mut version = "v0.0".to_string();

  if let Ok(file) = File::open(path) {
    let reader = BufReader::new(file);
    let set_value = |s: String| s.split('=').nth(1).unwrap_or_default().trim_matches('"').to_string();
    reader.lines().map_while(Result::ok).for_each(|line| {
      match line {
        l if l.starts_with("NAME=") => name = set_value(l),
        l if l.starts_with("VERSION=") => version = set_value(l),
        l if l.starts_with("VERSION_ID=") => version = set_value(l),
        _ => {}
      }
    });
  }

  format!("{name} {version}")
}

fn fmt_mem(bytes: u64) -> String {
  let units = ["B", "KB", "MB", "GB"];
  let mut bytes = bytes as f64;
  let mut unit = units[0];

  for &u in &units {
    if bytes < 1024.0 {
      unit = u;
      break;
    }
    bytes /= 1024.0;
  }

  format!("{bytes:.2} {unit}")
}

/// Retrieve host and bot uptimes
#[poise::command(slash_command)]
pub async fn uptime(ctx: PoiseContext<'_>) -> Result<(), RustbotError> {
  let bot = ctx.http().get_current_user().await.unwrap();
  let mut sys = System::new_all();
  sys.refresh_all();

  // Fetch system's uptime
  let sys_uptime = get().unwrap().as_secs();

  // Fetch system's processor
  let cpu = sys.cpus();

  // Fetch system memory usage
  let sram = fmt_mem(sys.used_memory());
  let sram_total = fmt_mem(sys.total_memory());

  // Fetch process memory usage
  let pram = match sys.process(sysinfo::get_current_pid().unwrap()) {
    Some(proc) => fmt_mem(proc.memory()),
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
  let docker_node = match var("DOCKER_HOSTNAME") {
    Ok(h) => h.to_string(),
    Err(_) => "DOCKER_HOSTNAME is empty!".to_string()
  };

  let stat_msg = [
    format!("**{} v{}** `{}:{}`", bot.name, BOT_VERSION.as_str(), GIT_COMMIT_HASH, GIT_COMMIT_BRANCH),
    format!(">>> System: `{}`", format_duration(sys_uptime)),
    format!("Process: `{}`", format_duration(proc_uptime)),
    format!("Node: `{docker_node}`"),
    format!("CPU: `{}`", cpu[0].brand()),
    format!("RAM: `{pram}` (`{sram}/{sram_total}`)"),
    format!("OS: `{}`", get_os_info())
  ];
  ctx.reply(stat_msg.join("\n")).await?;

  Ok(())
}
