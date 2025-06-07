#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  use tauri::{command, State};
  use std::env;
  use std::fs::{self, File};
  use std::io::{self, Write};
  use std::process::Command;
  use std::time::SystemTime;

  #[cfg(windows)]
  const USER_ENV: &str = "USERNAME";
  #[cfg(not(windows))]
  const USER_ENV: &str = "USER";

  // Check if git is installed
  #[command]
  fn check_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
  }

  // Check if current directory has a git repository
  #[command]
  fn has_git_repo() -> bool {
    fs::metadata(".git").is_ok()
  }

  // Initialize git repository
  #[command]
  fn init_repo() -> String {
    // Initialize git repository
    let git_init = Command::new("git")
        .arg("init")
        .status();
    
    if git_init.is_err() {
        return "Failed to initialize repository".to_string();
    }

    // Write .gitignore
    let gitignore_result = fs::write(".gitignore", "log/\n.DS_Store\nnul\n");
    if gitignore_result.is_err() {
        return "Failed to create .gitignore file".to_string();
    }

    // Create log directory
    #[cfg(windows)]
    let mkdir_result = Command::new("cmd").args(&["/C", "mkdir", "log"]).status();
    #[cfg(not(windows))]
    let mkdir_result = Command::new("mkdir").arg("-p").arg("log").status();
    
    if mkdir_result.is_err() {
        return "Failed to create log directory".to_string();
    }

    // Initial commit
    let add_result = Command::new("git").args(&["add", "."]).status();
    if add_result.is_err() {
        return "Failed to stage files".to_string();
    }

    let commit_result = Command::new("git")
        .args(&["commit", "--allow-empty", "-m", "Initial commit"])
        .status();
    
    if commit_result.is_err() {
        return "Failed to create initial commit".to_string();
    }

    "Repository successfully initialized".to_string()
  }

  // Get formatted timestamp
  fn get_timestamp() -> String {
    let now = SystemTime::now();
    let datetime = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let secs = datetime.as_secs();
    
    // Convert to local time components
    let mut secs_remaining = secs;
    let year = 1970 + (secs_remaining / 31536000);
    secs_remaining %= 31536000;
    let month = 1 + (secs_remaining / 2592000);
    secs_remaining %= 2592000;
    let day = 1 + (secs_remaining / 86400); 
    secs_remaining %= 86400;
    let hour = secs_remaining / 3600;
    secs_remaining %= 3600;
    let min = secs_remaining / 60;
    let sec = secs_remaining % 60;

    format!("{:04}-{:02}-{:02}_{:02}-{:02}-{:02}", 
        year, month, day, hour, min, sec)
  }

  // Get username
  fn get_username() -> String {
    env::var(USER_ENV).unwrap_or_else(|_| "unknown".to_string())
  }

  // Commit changes
  #[command]
  fn commit_changes() -> String {
    let user = get_username();
    let timestamp = get_timestamp();

    // Add all changes to staging
    let add_result = Command::new("git").args(&["add", "."]).status();
    if add_result.is_err() {
        return "Failed to stage changes".to_string();
    }

    // Check if there are any changes
    let status_output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output();
    
    if let Ok(output) = status_output {
        if output.stdout.is_empty() {
            return "No changes detected in the repository".to_string();
        }
    } else {
        return "Failed to check repository status".to_string();
    }

    // Commit changes
    let commit_msg = format!("{} {}", user, timestamp);
    let commit_result = Command::new("git")
        .args(&["commit", "-m", &commit_msg])
        .status();
    
    if commit_result.is_err() {
        return "Failed to commit changes".to_string();
    }

    // Write to log file
    let logfile = format!("log/{}.log", timestamp);
    let log_result = File::create(&logfile)
        .and_then(|mut file| {
            writeln!(file, "Commit by {} at {}", user, timestamp)
        });
    
    if log_result.is_err() {
        return format!("Changes committed successfully by {}, but failed to write log", user);
    }

    format!("Changes committed successfully by {}", user)
  }

  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        check_git_installed,
        has_git_repo,
        init_repo,
        commit_changes
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
