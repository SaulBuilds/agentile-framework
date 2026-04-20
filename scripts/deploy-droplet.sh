#!/usr/bin/env bash
set -e

# Create working directory
mkdir -p /opt/musicbox/runtime /opt/musicbox/presets

# Generate a random API key
API_KEY=$(openssl rand -hex 32)
echo "MUSIC_BOX_API_KEY=$API_KEY" > /opt/musicbox/.env
chmod 600 /opt/musicbox/.env

# Create systemd service
cat > /etc/systemd/system/music-box-api.service << 'SVCEOF'
[Unit]
Description=state-space-music-box HTTP API
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/musicbox
EnvironmentFile=/opt/musicbox/.env
ExecStart=/usr/local/bin/music-box-api http --port 3001 --api-key ${MUSIC_BOX_API_KEY} --preset-dir /opt/musicbox/presets --runtime-dir /opt/musicbox/runtime
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
SVCEOF

# Set up firewall
ufw allow 22/tcp
ufw allow 3001/tcp
echo "y" | ufw enable || true

# Start the service
systemctl daemon-reload
systemctl enable music-box-api
systemctl start music-box-api

# Wait and check
sleep 2
systemctl status music-box-api --no-pager | head -15

echo ""
echo "=== API KEY ==="
cat /opt/musicbox/.env
echo ""
echo "=== HEALTH CHECK ==="
curl -s http://localhost:3001/api/health || echo "waiting for startup..."
