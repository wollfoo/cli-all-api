# =============================================================================
# ProxyPal RPM Spec File
# RPM package specification cho RHEL/CentOS/Fedora
# =============================================================================

Name:           proxypal
Version:        0.1.0
Release:        1%{?dist}
Summary:        AI Proxy Server for Claude, OpenAI, Gemini, and Copilot

License:        MIT
URL:            https://github.com/wollfoo/cli-all-api
Source0:        %{name}-%{version}.tar.gz

# Build requirements
BuildRequires:  systemd-rpm-macros

# Runtime requirements
Requires:       systemd
Requires:       openssl-libs
Requires:       ca-certificates

# Don't try to build from source - we use pre-built binaries
AutoReqProv:    no

%description
ProxyPal is a unified proxy server that provides a single endpoint
for multiple AI providers with automatic authentication management,
load balancing, and quota handling.

Features:
- Unified proxy for Claude, OpenAI, Gemini, Copilot
- Automatic API key rotation and quota management
- Device code OAuth flow for authentication
- Systemd integration for daemon mode
- Health check endpoints for monitoring

Works seamlessly with AI coding tools like Cursor, Continue, and Claude Code.

# =============================================================================
# Preparation and Build
# =============================================================================

%prep
# Nothing to prep - using pre-built binaries

%build
# Nothing to build - using pre-built binaries

# =============================================================================
# Installation
# =============================================================================

%install
rm -rf %{buildroot}

# Create directories
mkdir -p %{buildroot}/usr/local/bin
mkdir -p %{buildroot}%{_unitdir}
mkdir -p %{buildroot}%{_mandir}/man1
mkdir -p %{buildroot}/etc/proxypal

# Install binaries
install -m 755 %{_sourcedir}/proxypal %{buildroot}/usr/local/bin/proxypal
install -m 755 %{_sourcedir}/cliproxyapi %{buildroot}/usr/local/bin/cliproxyapi || true

# Install systemd service
install -m 644 %{_sourcedir}/proxypal.service %{buildroot}%{_unitdir}/proxypal.service

# Install man page
install -m 644 %{_sourcedir}/proxypal.1.gz %{buildroot}%{_mandir}/man1/proxypal.1.gz || true

# =============================================================================
# Files
# =============================================================================

%files
%defattr(-,root,root,-)
/usr/local/bin/proxypal
/usr/local/bin/cliproxyapi
%{_unitdir}/proxypal.service
%{_mandir}/man1/proxypal.1.gz
%dir /etc/proxypal

# =============================================================================
# Scriptlets
# =============================================================================

%pre
# Pre-install: Create user if not exists
if ! id -u proxypal > /dev/null 2>&1; then
    useradd --system --no-create-home --shell /usr/sbin/nologin proxypal
fi

%post
# Post-install: Setup and enable service
# Create directories
mkdir -p /etc/proxypal
mkdir -p /var/log/proxypal
mkdir -p /var/lib/proxypal/auth

# Set ownership
chown -R proxypal:proxypal /etc/proxypal
chown -R proxypal:proxypal /var/log/proxypal
chown -R proxypal:proxypal /var/lib/proxypal

# Set permissions
chmod 755 /etc/proxypal
chmod 755 /var/log/proxypal
chmod 700 /var/lib/proxypal/auth

# Create default config if not exists
if [ ! -f /etc/proxypal/config.yaml ]; then
    cat > /etc/proxypal/config.yaml << 'EOF'
# ProxyPal Configuration
# See: https://github.com/wollfoo/cli-all-api

server:
  port: 8317
  host: "127.0.0.1"

logging:
  level: info
  format: json

providers: {}
EOF
    chown proxypal:proxypal /etc/proxypal/config.yaml
    chmod 640 /etc/proxypal/config.yaml
fi

# Reload systemd and enable service
%systemd_post proxypal.service

echo ""
echo "============================================"
echo "  ProxyPal installed successfully!"
echo "============================================"
echo ""
echo "Next steps:"
echo "  1. Edit config: /etc/proxypal/config.yaml"
echo "  2. Add auth:    proxypal auth add --provider gemini --device-code"
echo "  3. Start:       sudo systemctl start proxypal"
echo "  4. Status:      sudo systemctl status proxypal"
echo ""

%preun
# Pre-uninstall: Stop and disable service
%systemd_preun proxypal.service

%postun
# Post-uninstall: Cleanup systemd
%systemd_postun_with_restart proxypal.service

# =============================================================================
# Changelog
# =============================================================================

%changelog
* Tue Dec 10 2024 ProxyPal Team <team@proxypal.dev> - 0.1.0-1
- Initial RPM package release
- Includes proxypal and cliproxyapi binaries
- Systemd integration
- Man page documentation
