fn main() {
  #[cfg(feature = "production")]
  {
    if let Ok(git_commit_hash) = std::env::var("GIT_COMMIT_HASH") {
      println!("cargo:rustc-env=GIT_COMMIT_HASH={}", &git_commit_hash[..7]);
    } else {
      println!("cargo:warning=GIT_COMMIT_HASH not found, falling back to 'not_found'");
      println!("cargo:rustc-env=GIT_COMMIT_HASH=not_found");
    }
  }

  {
    let git_branch = std::process::Command::new("git")
      .args(["rev-parse", "--abbrev-ref", "HEAD"])
      .output()
      .expect("Command execution failed: git");

    if git_branch.status.success() {
      let git_branch = String::from_utf8(git_branch.stdout).expect("Invalid UTF-8 sequence").trim().to_string();
      println!("cargo:rustc-env=GIT_COMMIT_BRANCH={}", &git_branch);
    } else {
      println!("cargo:warning=GIT_COMMIT_BRANCH not found, falling back to 'not_found'");
      println!("cargo:rustc-env=GIT_COMMIT_BRANCH=not_found");
    }
  }

  {
    let hostname = std::process::Command::new("hostname")
      .output()
      .expect("Command execution failed: hostname");

    if hostname.status.success() {
      let hostname = String::from_utf8(hostname.stdout).expect("Invalid UTF-8 sequence").trim().to_string();
      println!("cargo:rustc-env=DOCKER_HOSTNAME={}", &hostname);
    }
  }
}
