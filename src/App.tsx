import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [gitInstalled, setGitInstalled] = useState<boolean | null>(null);
  const [inRepo, setInRepo] = useState<boolean | null>(null);
  const [message, setMessage] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);

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
    setMessage("Initializing repository...");
    
    try {
      const result = await invoke<string>("init_repo");
      setMessage(result);
      setInRepo(true);
    } catch (e: any) {
      setMessage("Error: " + e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleCommit = async () => {
    setLoading(true);
    setMessage("Committing changes...");
    
    try {
      const result = await invoke<string>("commit_changes");
      setMessage(result);
    } catch (e: any) {
      setMessage("Error: " + e.toString());
    } finally {
      setLoading(false);
    }
  };

  if (gitInstalled === null) {
    return (
      <div className="container">
        <h1>Git Snapshot</h1>
        <p>Checking git installation...</p>
      </div>
    );
  }

  if (!gitInstalled) {
    return (
      <div className="container">
        <h1>Git Snapshot</h1>
        <div className="error-box">
          <h2>Error</h2>
          <p>Git is not installed on your system.</p>
          <p>Please install Git before using this tool.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="container">
      <h1>Git Snapshot</h1>
      
      <div className="status-box">
        <h2>Repository Status</h2>
        {inRepo ? (
          <p>Working with git repository in the current directory.</p>
        ) : (
          <p>Current directory is not a git repository.</p>
        )}
      </div>

      <div className="action-box">
        <h2>Actions</h2>
        {!inRepo ? (
          <button 
            onClick={handleInit} 
            disabled={loading}
            className="action-button"
          >
            {loading ? "Initializing..." : "Initialize Repository"}
          </button>
        ) : (
          <button 
            onClick={handleCommit} 
            disabled={loading}
            className="action-button"
          >
            {loading ? "Committing..." : "Commit Snapshot"}
          </button>
        )}
      </div>

      {message && (
        <div className="message-box">
          <h2>Message</h2>
          <p>{message}</p>
        </div>
      )}
    </div>
  );
}

export default App;
