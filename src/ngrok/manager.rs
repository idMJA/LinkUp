use anyhow::{Context, Result, anyhow};
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use tokio::time::{Duration, sleep};

use super::process::NgrokProcess;
use super::tunnel::NgrokTunnel;
use crate::config::NgrokInstance;

#[derive(Deserialize)]
struct TunnelsResponse {
    tunnels: Vec<NgrokTunnel>,
}

pub struct NgrokManager {
    instances: HashMap<String, NgrokProcess>,
}

impl NgrokManager {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }

    pub fn add_instance(&mut self, config: NgrokInstance) -> Result<()> {
        let name = config.name.clone();

        if config.authtoken.is_empty() || config.authtoken.contains("your_") {
            return Err(anyhow!(
                "Invalid ngrok token for instance '{name}'. Please provide a valid authtoken."
            ));
        }

        let process = NgrokProcess {
            config,
            process: None,
        };

        self.instances.insert(name.clone(), process);
        info!("Added ngrok instance: {name}");
        Ok(())
    }

    pub async fn start_all(&mut self) -> Result<()> {
        let names: Vec<String> = self.instances.keys().cloned().collect();
        for name in names {
            match self.start_instance(&name).await {
                Ok(()) => info!("Started ngrok instance: {name}"),
                Err(e) => error!("Failed to start ngrok instance {name}: {e}"),
            }
        }
        Ok(())
    }

    async fn start_instance_internal(&self, process: &mut NgrokProcess) -> Result<()> {
        let child = Command::new("ngrok")
            .arg(process.config.protocol.as_str())
            .arg(process.config.port.to_string())
            .arg("--authtoken")
            .arg(&process.config.authtoken)
            .arg("--region")
            .arg(&process.config.region)
            .arg("--log")
            .arg("stdout")
            .arg("--log-format")
            .arg("json")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to start ngrok process")?;

        process.process = Some(child);

        sleep(Duration::from_secs(3)).await;

        Ok(())
    }

    pub async fn start_instance(&mut self, name: &str) -> Result<()> {
        let config = self
            .instances
            .get(name)
            .ok_or_else(|| anyhow!("Instance not found: {name}"))?
            .config
            .clone();

        let mut process = NgrokProcess {
            config,
            process: None,
        };

        self.start_instance_internal(&mut process).await?;

        if let Some(entry) = self.instances.get_mut(name) {
            entry.process = process.process;
        }

        info!("Started ngrok instance: {name}");
        Ok(())
    }

    pub fn stop_instance(&mut self, name: &str) -> Result<()> {
        let process = self
            .instances
            .get_mut(name)
            .ok_or_else(|| anyhow!("Instance not found: {name}"))?;

        process.kill()?;
        info!("Stopped ngrok instance: {name}");
        Ok(())
    }

    pub async fn get_tunnels(&self, _name: &str) -> Result<Vec<NgrokTunnel>> {
        let client = reqwest::Client::new();
        let response = client
            .get("http://127.0.0.1:4040/api/tunnels")
            .send()
            .await
            .context("Failed to query ngrok API")?;

        let status: TunnelsResponse = response
            .json()
            .await
            .context("Failed to parse ngrok API response")?;

        Ok(status.tunnels)
    }

    pub async fn restart_instance(&mut self, name: &str) -> Result<()> {
        info!("Restarting ngrok instance: {name}");
        self.stop_instance(name)?;
        sleep(Duration::from_secs(2)).await;
        self.start_instance(name).await?;
        Ok(())
    }

    pub fn check_health(&self) -> HashMap<String, bool> {
        self.instances
            .iter()
            .map(|(name, process)| {
                let is_running = process
                    .process
                    .as_ref()
                    .is_some_and(|child| child.id() != 0);
                (name.clone(), is_running)
            })
            .collect()
    }
}

impl Drop for NgrokManager {
    fn drop(&mut self) {
        for (name, process) in &mut self.instances {
            if let Err(e) = process.kill() {
                error!("Error cleaning up ngrok instance {name}: {e}");
            }
        }
    }
}
