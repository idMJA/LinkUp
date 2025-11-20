use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use serde_json::json;

pub struct GenericWebhook {
    client: Client,
}

impl GenericWebhook {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn send_message(&self, url: &str, message: &str) -> Result<()> {
        let payload = json!({
            "message": message,
            "timestamp": Utc::now().to_rfc3339(),
            "service": "LinkUp"
        });

        let response = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send generic webhook")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Generic webhook failed with status {status}: {body}"
            ));
        }

        Ok(())
    }
}
