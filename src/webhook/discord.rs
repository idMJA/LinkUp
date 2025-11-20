use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use serde_json::json;

const SUCCESS_COLOR: u32 = 3_066_993;

pub struct DiscordWebhook {
    client: Client,
}

impl DiscordWebhook {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn send_message(
        &self,
        url: &str,
        title: &str,
        description: &str,
        color: u32,
    ) -> Result<()> {
        let timestamp = Utc::now().to_rfc3339();

        let payload = json!({
            "embeds": [{
                "title": title,
                "description": description,
                "color": color,
                "timestamp": timestamp,
                "footer": {
                    "text": "LinkUp Ngrok Manager",
                }
            }]
        });

        self.post_payload(url, payload).await
    }

    pub async fn send_tunnels(
        &self,
        url: &str,
        instance_name: &str,
        fields: Vec<serde_json::Value>,
    ) -> Result<()> {
        let timestamp = Utc::now().to_rfc3339();

        let payload = json!({
            "embeds": [{
                "title": "Ngrok Tunnels are Ready!",
                "description": format!("**{instance_name}**\n\nTunnels are ready and accessible:"),
                "color": SUCCESS_COLOR,
                "fields": fields,
                "timestamp": timestamp,
                "footer": {
                    "text": "LinkUp Ngrok Manager",
                }
            }]
        });

        self.post_payload(url, payload).await
    }

    async fn post_payload(&self, url: &str, payload: serde_json::Value) -> Result<()> {
        let response = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send Discord webhook")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Discord webhook failed with status {status}: {body}"
            ));
        }

        Ok(())
    }
}
