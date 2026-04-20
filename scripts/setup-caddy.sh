#!/usr/bin/env bash
set -e

DOMAIN="music-box-api.165-22-3-145.sslip.io"

# Install Caddy via official repo
apt install -y debian-keyring debian-archive-keyring apt-transport-https curl gnupg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list
apt update
apt install -y caddy

# Write Caddyfile
cat > /etc/caddy/Caddyfile << CADDYEOF
${DOMAIN} {
    reverse_proxy localhost:3001

    # Security headers
    header {
        Strict-Transport-Security "max-age=31536000; includeSubDomains"
        X-Content-Type-Options "nosniff"
        X-Frame-Options "DENY"
        Referrer-Policy "strict-origin-when-cross-origin"
    }

    # Basic rate limiting via Caddy's built-in tools happens at the app layer
    encode gzip

    log {
        output file /var/log/caddy/access.log
        format console
    }
}
CADDYEOF

# Open ports 80 and 443, close 3001 from outside
ufw allow 80/tcp
ufw allow 443/tcp
ufw delete allow 3001/tcp || true
ufw reload

# Reload Caddy (it's already started by the package)
systemctl reload caddy
systemctl status caddy --no-pager | head -12

echo ""
echo "=== Testing HTTPS ==="
sleep 5
curl -s https://${DOMAIN}/api/health && echo "" && echo "HTTPS works!"
