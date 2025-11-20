#!/bin/bash
# Installation script for LinkUp

set -e

echo "🚀 Installing LinkUp..."

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    echo "❌ Please don't run as root. We'll ask for sudo when needed."
    exit 1
fi

# Check if ngrok is installed
if ! command -v ngrok &> /dev/null; then
    echo "⚠️  ngrok not found. Installing..."
    curl -s https://ngrok-agent.s3.amazonaws.com/ngrok.asc | sudo tee /etc/apt/trusted.gpg.d/ngrok.asc >/dev/null
    echo "deb https://ngrok-agent.s3.amazonaws.com buster main" | sudo tee /etc/apt/sources.list.d/ngrok.list
    sudo apt update && sudo apt install -y ngrok
fi

# Build the project
echo "🔨 Building LinkUp..."
cargo build --release

# Create config directories
echo "📁 Creating config directories..."
mkdir -p ~/.config/linkup
mkdir -p /tmp/linkup-ngrok

# Copy example config if config doesn't exist
if [ ! -f ~/.config/linkup/config.toml ]; then
    if [ -f config.toml.example ]; then
        echo "📝 Copying example config to ~/.config/linkup/config.toml"
        cp config.toml.example ~/.config/linkup/config.toml
        echo "⚠️  Please edit ~/.config/linkup/config.toml with your settings"
    fi
fi

# Get current user and paths
CURRENT_USER=$(whoami)
WORKING_DIR=$(pwd)
BINARY_PATH="$WORKING_DIR/target/release/LinkUp"

# Create systemd service file
echo "🔧 Creating systemd service..."
cat > /tmp/linkup.service << EOF
[Unit]
Description=LinkUp - Ngrok Manager Service
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=$CURRENT_USER
WorkingDirectory=$WORKING_DIR
ExecStart=$BINARY_PATH
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
EOF

# Install systemd service
echo "📦 Installing systemd service..."
sudo cp /tmp/linkup.service /etc/systemd/system/
sudo systemctl daemon-reload

echo ""
echo "✅ Installation complete!"
echo ""
echo "Next steps:"
echo "1. Edit config: nano ~/.config/linkup/config.toml"
echo "2. Enable service: sudo systemctl enable linkup"
echo "3. Start service: sudo systemctl start linkup"
echo "4. Check status: sudo systemctl status linkup"
echo "5. View logs: sudo journalctl -u linkup -f"
echo ""
echo "Or test manually first: $BINARY_PATH"
