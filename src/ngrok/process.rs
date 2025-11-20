use anyhow::{Context, Result};
use log::info;
use std::process::Child;

use crate::config::NgrokInstance;

pub struct NgrokProcess {
    pub config: NgrokInstance,
    pub process: Option<Child>,
}

impl NgrokProcess {
    pub fn kill(&mut self) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            child.kill().context("Failed to kill ngrok process")?;
            info!("Killed ngrok process for instance: {}", self.config.name);
        }
        Ok(())
    }
}
