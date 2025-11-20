use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgrokTunnel {
    pub public_url: String,
    pub proto: String,
    pub config: NgrokTunnelConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgrokTunnelConfig {
    pub addr: String,
}
