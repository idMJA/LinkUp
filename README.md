# LinkUp - Ngrok Manager 🚀

**LinkUp** is a Rust application to run multiple ngrok instances with different tokens automatically. The application supports notifications to various platforms like Discord, Slack, and generic webhooks.

## ✨ Features

- 🔄 **Multi-Token Support**: Run multiple ngrok instances with different tokens on a single system
- 🤖 **Auto-Start**: Automatically run on Linux system startup (systemd)
- 📢 **Multi-Platform Notifications**: Send notifications to Discord, Slack, or custom webhooks
- 🔁 **Auto-Restart**: Automatically restart if an ngrok instance encounters issues
- 💪 **Health Monitoring**: Monitor the status of all ngrok instances
- 📝 **Detailed Logging**: Comprehensive logs for debugging

## 📋 Requirements

- Rust 1.70+ (for compilation)
- ngrok installed and available in PATH
- Linux with systemd (for auto-start)

### Install ngrok

```bash
# Download ngrok
curl -s https://ngrok-agent.s3.amazonaws.com/ngrok.asc | sudo tee /etc/apt/trusted.gpg.d/ngrok.asc >/dev/null
echo "deb https://ngrok-agent.s3.amazonaws.com bookworm main" | sudo tee /etc/apt/sources.list.d/ngrok.list
sudo apt update && sudo apt install ngrok
```

## 🚀 Installation

### 1. Clone and Build

```bash
cd /mnt/HDD1/Projects/LinkUp
cargo build --release
```

Binary will be available at `target/release/LinkUp`

### 2. Setup Configuration

```bash
# Copy example configuration
cp config.toml.example config.toml

# Edit configuration
nano config.toml
```

Fill `config.toml` with your ngrok tokens and webhook URLs:

```toml
# Ngrok instances
[[ngrok_instances]]
name = "instance1"
authtoken = "your_ngrok_token_1"
port = 8080
protocol = "http"
region = "us"

[[ngrok_instances]]
name = "instance2"
authtoken = "your_ngrok_token_2"
port = 3000
protocol = "http"
region = "us"

# Webhooks
[[webhooks]]
name = "discord"
type = "discord"
url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"
enabled = true

[settings]
check_interval_seconds = 60
auto_restart = true
log_level = "info"
```

### 3. Test Run

```bash
# Set log level
export RUST_LOG=info

# Run
./target/release/LinkUp
```

## 🔧 Setup Auto-Start (systemd)

### 1. Install Service File

```bash
# Edit service file with correct path
nano linkup.service

# Replace placeholders:
# %USER% -> your username (e.g., user)
# %WORKING_DIR% -> /mnt/HDD1/Projects/LinkUp
# %BINARY_PATH% -> /mnt/HDD1/Projects/LinkUp/target/release/LinkUp

# Copy to systemd
sudo cp linkup.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload
```

### 2. Enable and Start Service

```bash
# Enable auto-start
sudo systemctl enable linkup

# Start service
sudo systemctl start linkup

# Check status
sudo systemctl status linkup

# View logs
sudo journalctl -u linkup -f
```

### 3. Manage Service

```bash
# Stop service
sudo systemctl stop linkup

# Restart service
sudo systemctl restart linkup

# Disable auto-start
sudo systemctl disable linkup
```

## 📝 Configuration Details

### Ngrok Instance Configuration

```toml
[[ngrok_instances]]
name = "my-service"        # Unique name for instance
authtoken = "xxx"          # Ngrok auth token
port = 8080               # Local port to expose
protocol = "http"         # Protocol: http or tcp
region = "us"             # Region: us, eu, ap, au, sa, jp, in
```

### Webhook Configuration

#### Discord

```toml
[[webhooks]]
name = "discord-main"
type = "discord"
url = "https://discord.com/api/webhooks/ID/TOKEN"
enabled = true
```

**How to create Discord Webhook:**
1. Open Discord Server Settings
2. Select Integrations > Webhooks
3. Click "New Webhook"
4. Copy Webhook URL

#### Slack

```toml
[[webhooks]]
name = "slack-team"
type = "slack"
url = "https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK"
enabled = true
```

#### Generic Webhook

```toml
[[webhooks]]
name = "custom-api"
type = "generic"
url = "https://your-api.com/webhook"
enabled = true
```

Generic webhook payload format:
```json
{
  "message": "notification message",
  "timestamp": "2025-11-18T10:30:00Z",
  "service": "LinkUp"
}
```

### Settings Configuration

```toml
[settings]
check_interval_seconds = 60  # Health check interval (seconds)
auto_restart = true          # Auto restart if ngrok fails
log_level = "info"          # Log level: debug, info, warn, error
```

## 📂 Config File Locations

LinkUp will search for config files in the following locations (priority order):

1. Path provided as argument: `./LinkUp /path/to/config.toml`
2. Current directory: `./config.toml`
3. Home config: `~/.config/linkup/config.toml`
4. System config: `/etc/linkup/config.toml`

## 🔍 Notifications

LinkUp will send notifications for the following events:

- 🚀 **Startup**: When instance starts
- ✅ **Tunnel Created**: When tunnel is successfully created (with URL)
- 🔄 **Restart**: When instance is restarted
- ❌ **Error**: When an error occurs
- 🛑 **Shutdown**: When instance is stopped

## 🛠️ Development

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run development
cargo run
```

### Testing

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests (if any)
cargo test
```

## 📊 Monitoring

### View Logs

```bash
# Service logs
sudo journalctl -u linkup -f

# Filter by level
sudo journalctl -u linkup -p err

# Last 100 lines
sudo journalctl -u linkup -n 100
```

### Check Ngrok Tunnels

```bash
# Ngrok provides a local API
curl http://localhost:4040/api/tunnels
```

## 🔒 Security Notes

- Do not commit `config.toml` to git (already in `.gitignore`)
- Keep ngrok tokens secure
- Use HTTPS for webhook URLs
- Restrict access to config file: `chmod 600 config.toml`

## 🐛 Troubleshooting

### Service won't start

```bash
# Check service status
sudo systemctl status linkup

# Check logs
sudo journalctl -u linkup -n 50

# Check permissions
ls -la /mnt/HDD1/Projects/LinkUp/target/release/LinkUp
```

### Ngrok won't connect

- Make sure ngrok is installed: `which ngrok`
- Check ngrok token is valid: `ngrok config check`
- Check port is not in use: `sudo netstat -tulpn | grep PORT`

### Webhook not being sent

- Check webhook URL is valid
- Test webhook with curl
- Check network connectivity
- View error logs

## 📄 License

MIT License - feel free to use and modify as needed.

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first.

## 📞 Support

If you have any questions or issues, please create an issue in this repository.

---

**Made with ❤️ using Rust**
