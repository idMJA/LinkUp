# LinkUp - Ngrok Manager 🚀

**LinkUp** is a Rust application that automatically manages multiple ngrok instances with different authentication tokens and sends notifications to Discord, Slack, or custom webhooks.

## ✨ Features

- 🔄 **Multi-Token Support**: Run multiple ngrok instances with different tokens on a single system
- 🤖 **Auto-Start**: Automatically run on Linux system startup (systemd)
- 📢 **Multi-Platform Notifications**: Send notifications to Discord, Slack, or custom webhooks
- 🔁 **Auto-Restart**: Automatically restart if an ngrok instance encounters issues
- 💪 **Health Monitoring**: Monitor the status of all ngrok instances
- 📝 **Detailed Logging**: Comprehensive logs for debugging

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/idMJA/LinkUp.git
cd LinkUp

# Build
cargo build --release

# Copy and edit configuration
cp config.toml.example config.toml
nano config.toml

# Run
./target/release/LinkUp
```

### Configuration

Edit `config.toml` with your ngrok tokens and webhook URLs:

```toml
[[ngrok_instances]]
name = "instance1"
authtoken = "your_ngrok_token"
port = 8080
protocol = "http"
region = "us"

[[webhooks]]
name = "discord"
type = "discord"
url = "https://discord.com/api/webhooks/YOUR_ID/YOUR_TOKEN"
enabled = true

[settings]
check_interval_seconds = 60
auto_restart = true
log_level = "info"
```

## 🔧 Auto-Start with Systemd

```bash
# Edit service file with your paths
nano linkup.service

# Copy to systemd
sudo cp linkup.service /etc/systemd/system/

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable linkup
sudo systemctl start linkup

# View status
sudo systemctl status linkup
sudo journalctl -u linkup -f
```

## 📝 Configuration Details

### Ngrok Instances

```toml
[[ngrok_instances]]
name = "my-service"
authtoken = "your_ngrok_token"
port = 8080
protocol = "http"  # http or tcp
region = "us"      # us, eu, ap, au, sa, jp, in
```

### Webhooks

**Discord:**
```toml
[[webhooks]]
name = "discord"
type = "discord"
url = "https://discord.com/api/webhooks/ID/TOKEN"
enabled = true
```

**Custom Webhook:**
```toml
[[webhooks]]
name = "custom"
type = "generic"
url = "https://your-api.com/webhook"
enabled = true
```

Payload format:
```json
{
  "message": "notification message",
  "timestamp": "2025-11-18T10:30:00Z",
  "service": "LinkUp"
}
```

### Settings

```toml
[settings]
check_interval_seconds = 60  # Health check interval
auto_restart = true          # Auto restart failed instances
log_level = "info"           # debug, info, warn, error
```

## 📍 Config File Locations

LinkUp searches in this order:
1. Argument: `./LinkUp /path/to/config.toml`
2. Current directory: `./config.toml`
3. Home: `~/.config/linkup/config.toml`
4. System: `/etc/linkup/config.toml`

## 🔔 Notifications

Events that trigger notifications:
- 🚀 Startup
- ✅ Tunnel created (with URL)
- 🔄 Restart
- ❌ Error
- 🛑 Shutdown

## 🐛 Troubleshooting

### Service won't start
```bash
sudo systemctl status linkup
sudo journalctl -u linkup -n 50
```

### Ngrok won't connect
- Check ngrok is installed: `which ngrok`
- Verify token: `ngrok config check`
- Check port available: `sudo netstat -tulpn | grep PORT`

### Webhook not sending
- Verify webhook URL is valid
- Check network connectivity
- View logs: `sudo journalctl -u linkup -f`

## 🔒 Security

- Don't commit `config.toml` (already in `.gitignore`)
- Keep ngrok tokens private
- Use HTTPS for webhooks
- Set file permissions: `chmod 600 config.toml`

## 📄 License

MIT License

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first.

## 📞 Support

If you have any questions or issues, please create an issue in this repository.

---

**Made with ❤️ using Rust**
