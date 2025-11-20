use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NgrokInstance {
    pub name: String,
    pub authtoken: String,
    pub port: u16,
    pub protocol: String,
    pub region: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Webhook {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub url: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub check_interval_seconds: u64,
    pub auto_restart: bool,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub ngrok_instances: Vec<NgrokInstance>,
    pub webhooks: Vec<Webhook>,
    pub settings: Settings,
}
