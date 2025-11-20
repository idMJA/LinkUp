use anyhow::Result;
use log::{error, info};
use serde_json::{Value, json};
use std::fmt::Write;

use super::discord::DiscordWebhook;
use super::generic::GenericWebhook;
use crate::config::Webhook;
use crate::ngrok::NgrokTunnel;

const COLOR_ERROR: u32 = 15_158_332;
const COLOR_RESTARTING: u32 = 16_776_960;
const COLOR_STOPPED: u32 = 9_807_270;
const COLOR_STARTED: u32 = 5_814_783;

pub struct WebhookNotifier {
    webhooks: Vec<Webhook>,
    discord: DiscordWebhook,
    generic: GenericWebhook,
}

impl WebhookNotifier {
    pub fn new(webhooks: Vec<Webhook>) -> Self {
        Self {
            webhooks: webhooks.into_iter().filter(|w| w.enabled).collect(),
            discord: DiscordWebhook::new(),
            generic: GenericWebhook::new(),
        }
    }

    pub async fn notify_tunnel_created(
        &self,
        instance_name: &str,
        tunnels: &[NgrokTunnel],
    ) -> Result<()> {
        for webhook in &self.webhooks {
            match webhook.kind.as_str() {
                "discord" => {
                    let fields: Vec<Value> = tunnels
                        .iter()
                        .map(|tunnel| {
                            let clean_url = Self::clean_url(&tunnel.public_url);
                            json!({
                                "name": format!("🔗 {}", tunnel.proto.to_uppercase()),
                                "value": format!("```\n{clean_url}\n```"),
                                "inline": false,
                            })
                        })
                        .collect();

                    if let Err(e) = self
                        .discord
                        .send_tunnels(&webhook.url, instance_name, fields)
                        .await
                    {
                        error!("Failed to send Discord webhook '{}': {e}", webhook.name);
                    } else {
                        info!("Sent Discord tunnel notification to '{}'", webhook.name);
                    }
                }
                "generic" => {
                    let mut message =
                        format!("Ngrok tunnels for '{instance_name}' are ready:\n");
                    for tunnel in tunnels {
                        let _ =
                            writeln!(message, "• {} → {}", tunnel.public_url, tunnel.config.addr);
                    }
                    if let Err(e) = self.generic.send_message(&webhook.url, &message).await {
                        error!("Failed to send generic webhook '{}': {e}", webhook.name);
                    } else {
                        info!("Sent generic notification to '{}'", webhook.name);
                    }
                }
                _ => {
                    error!("Unknown webhook type: {}", webhook.kind);
                }
            }
        }
        Ok(())
    }

    pub async fn notify_error(&self, instance_name: &str, error: &str) -> Result<()> {
        let message = format!("❌ LinkUp Error: Instance '{instance_name}' - {error}");
        self.send_notification(&message).await
    }

    pub async fn notify_restart(&self, instance_name: &str) -> Result<()> {
        let message = format!("🔄 LinkUp: Restarting ngrok instance '{instance_name}'");
        self.send_notification(&message).await
    }

    async fn send_notification(&self, message: &str) -> Result<()> {
        for webhook in &self.webhooks {
            match webhook.kind.as_str() {
                "discord" => {
                    let (title, color) = Self::discord_style(message);
                    if let Err(e) = self
                        .discord
                        .send_message(&webhook.url, title, message, color)
                        .await
                    {
                        error!("Failed to send Discord webhook '{}': {e}", webhook.name);
                    } else {
                        info!("Sent Discord notification to '{}'", webhook.name);
                    }
                }
                "generic" => {
                    if let Err(e) = self.generic.send_message(&webhook.url, message).await {
                        error!("Failed to send generic webhook '{}': {e}", webhook.name);
                    } else {
                        info!("Sent generic notification to '{}'", webhook.name);
                    }
                }
                _ => {
                    error!("Unknown webhook type: {}", webhook.kind);
                }
            }
        }
        Ok(())
    }

    fn discord_style(message: &str) -> (&'static str, u32) {
        if message.contains("Error") {
            ("❌ Error", COLOR_ERROR)
        } else if message.contains("Restarting") {
            ("🔄 Restarting", COLOR_RESTARTING)
        } else if message.contains("stopped") {
            ("🛑 Stopped", COLOR_STOPPED)
        } else {
            ("🚀 Started", COLOR_STARTED)
        }
    }

    fn clean_url(url: &str) -> String {
        url.split_once("://")
            .map_or(url, |(_, rest)| rest)
            .to_string()
    }
}
