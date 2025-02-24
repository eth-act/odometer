use reqwest::Client;
use std::path::Path;
use std::process::{exit, Command, Output};
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::time::sleep;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("Docker command failed: {0}")]
    CommandFailed(String),
    #[error("Failed to execute docker command: {0}")]
    ExecutionFailed(#[from] std::io::Error),
    #[error("Health check timeout")]
    HealthCheckTimeout,
}

pub struct DockerCompose {
    compose_file: String,
}

impl DockerCompose {
    pub fn new(compose_file: &str) -> Self {
        Self {
            compose_file: compose_file.to_string(),
        }
    }

    fn run_command(&self, args: &[&str]) -> Result<Output, DockerError> {
        let output = Command::new("docker")
            .current_dir("clients")
            .args(["compose", "-f", &self.compose_file])
            .args(args)
            .output();

        match output {
            Ok(output) => {
                // Format docker output with emojis
                if !output.stdout.is_empty() {
                    println!(
                        "🔵 Docker: {}",
                        String::from_utf8_lossy(&output.stdout).trim()
                    );
                }
                if !output.stderr.is_empty() {
                    let stderr_str = String::from_utf8_lossy(&output.stderr);
                    let stderr_trimmed = stderr_str.trim();
                    if stderr_trimmed.contains("error") {
                        eprintln!("❌ Docker Error: {}", stderr_trimmed);
                    } else {
                        println!("ℹ️  Docker: {}", stderr_trimmed);
                    }
                }

                if !output.status.success() {
                    let error_message =
                        format!("❌ Docker command failed with status {}", output.status);
                    eprintln!("{}", error_message);
                    exit(1);
                }
                Ok(output)
            }
            Err(e) => {
                let error_message = format!("❌ Error executing docker command: {}", e);
                eprintln!("{}", error_message);
                exit(1);
            }
        }
    }

    fn get_project_name(&self) -> String {
        let file_name = Path::new(&self.compose_file)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("client");

        format!("odometer-{}", file_name)
    }

    pub fn up(&self) -> Result<(), DockerError> {
        self.run_command(&["-p", &self.get_project_name(), "up", "-d"])?;
        Ok(())
    }

    pub fn down(&self) -> Result<(), DockerError> {
        self.run_command(&["-p", &self.get_project_name(), "down", "--volumes"])?;
        Ok(())
    }

    pub async fn wait_for_healthy(&self, timeout_secs: u64) -> Result<(), DockerError> {
        let client = Client::new();
        let start = Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        while start.elapsed() < timeout {
            match client.get("http://localhost:8551").send().await {
                Ok(_) => return Ok(()),
                Err(_) => sleep(Duration::from_millis(500)).await,
            }
        }

        Err(DockerError::HealthCheckTimeout)
    }
}
