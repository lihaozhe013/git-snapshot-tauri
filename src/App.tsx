import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import logo from "./assets/logo.svg";

function App() {
  const [gitInstalled, setGitInstalled] = useState<boolean | null>(null);
  const [inRepo, setInRepo] = useState<boolean | null>(null);
  const [message, setMessage] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);
  const [action, setAction] = useState<string>("");

  useEffect(() => {
    checkGitStatus();
  }, []);

  const checkGitStatus = async () => {
    try {
      const installed = await invoke<boolean>("check_git_installed");
      setGitInstalled(installed);
      
      if (installed) {
        const repo = await invoke<boolean>("has_git_repo");
        setInRepo(repo);
      }
    } catch (e: any) {
      setMessage("Error checking git status: " + e.toString());
    }
  };

  const handleInit = async () => {
    setLoading(true);
    setAction("initializing");
    setMessage("Initializing repository...");
    
    try {
      const result = await invoke<string>("init_repo");
      setMessage(result);
      setInRepo(true);
    } catch (e: any) {
      setMessage("Error: " + e.toString());
    } finally {
      setLoading(false);
      setAction("");
    }
  };

  const handleCommit = async () => {
    setLoading(true);
    setAction("committing");
    setMessage("Committing changes...");
    
    try {
      const result = await invoke<string>("commit_changes");
      setMessage(result);
    } catch (e: any) {
      setMessage("Error: " + e.toString());
    } finally {
      setLoading(false);
      setAction("");
    }
  };

  const handleSync = async () => {
    setLoading(true);
    setAction("syncing");
    setMessage("Syncing repository...");
    
    try {
      const result = await invoke<string>("sync_repository");
      setMessage(result);
    } catch (e: any) {
      setMessage("Error: " + e.toString());
    } finally {
      setLoading(false);
      setAction("");
    }
  };

  if (gitInstalled === null) {
    return (
      <div className="container">
        <div className="logo-container">
          <img src={logo} alt="Git Snapshot Logo" className="app-logo" />
        </div>
        <h1>Git Snapshot</h1>
        <div className="content-area">
          <div className="status-box">
            <p>Checking git installation...</p>
          </div>
        </div>
      </div>
    );
  }

  if (!gitInstalled) {
    return (
      <div className="container">
        <div className="logo-container">
          <img src={logo} alt="Git Snapshot Logo" className="app-logo" />
        </div>
        <h1>Git Snapshot</h1>
        <div className="content-area">
          <div className="error-box">
            <h2>Git Not Installed</h2>
            <p>Git is not installed on your system or not in your PATH.</p>
            <p>Please install Git before using this tool.</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="container">
      <div className="logo-container">
        <img src={logo} alt="Git Snapshot Logo" className="app-logo" />
      </div>
      <h1>Git Snapshot</h1>
      
      <div className="content-area">
        <div className="status-box">
          <h2>Repository Status</h2>
          {inRepo ? (
            <p>Working with git repository in the current directory.</p>
          ) : (
            <p>Current directory is not a git repository.</p>
          )}
        </div>

        <div className="action-box">
          <h2>One-Tap Git Solution</h2>
          {!inRepo ? (
            <button 
              onClick={handleInit} 
              disabled={loading}
              className="action-button"
            >
              {action === "initializing" ? (
                <>
                  <span className="loading-spinner"></span>
                  Initializing...
                </>
              ) : (
                "Initialize Repository"
              )}
            </button>
          ) : (
            <div className="button-group">
              <button 
                onClick={handleCommit} 
                disabled={loading}
                className="action-button"
              >
                {action === "committing" ? (
                  <>
                    <span className="loading-spinner"></span>
                    Committing...
                  </>
                ) : (
                  "Commit Snapshot"
                )}
              </button>
              <button 
                onClick={handleSync} 
                disabled={loading}
                className="action-button"
              >
                {action === "syncing" ? (
                  <>
                    <span className="loading-spinner"></span>
                    Syncing...
                  </>
                ) : (
                  "Sync Repository"
                )}
              </button>
            </div>
          )}
        </div>

        {message && (
          <div className="message-box">
            <h2>Status</h2>
            <p>{message}</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
