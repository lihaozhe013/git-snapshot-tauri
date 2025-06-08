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
    let gitignore_result = fs::write(".gitignore", ".DS_Store\n");
    if gitignore_result.is_err() {
        return "Failed to create .gitignore file".to_string();
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

    format!("Changes committed successfully by {}", user)
  }

  // Sync repository (pull and push)
  #[command]
  fn sync_repository() -> String {
    // Check if remote is set
    let remote_check = Command::new("git")
        .args(&["remote", "-v"])
        .output();
    
    if let Ok(output) = remote_check {
        if output.stdout.is_empty() {
            return "No remote repository configured. Please set up a remote repository with 'git remote add origin <repository-url>'".to_string();
        }
    } else {
        return "Failed to check remote repository status".to_string();
    }

    // Pull with rebase
    let pull_result = Command::new("git")
        .args(&["pull", "-r"])
        .status();
    
    if let Err(e) = pull_result {
        return format!("Failed to pull changes: {}", e);
    }

    // Push changes
    let push_result = Command::new("git")
        .args(&["push"])
        .status();
    
    if let Err(e) = push_result {
        return format!("Failed to push changes: {}", e);
    }

    "Repository successfully synced".to_string()
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
        has_git_repo,
        init_repo,
        commit_changes,
        sync_repository
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
