#!/usr/bin/env bash
#
# Tscode installer script
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/iyjaa/Tscode/main/install.sh | sh
#
# Or:
#   curl -fsSL ... | bash
#

set -euo pipefail

REPO="iyjaa/Tscode"
BINARY_NAME="tscode"
INSTALL_DIR=""

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' 

info() {
    printf "${BLUE}[INFO]${NC} %s\n" "$1"
}

success() {
    printf "${GREEN}[OK]${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1"
}

error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1" >&2
    exit 1
}

detect_platform() {
    local os arch target install_dir

    os=$(uname -s)
    arch=$(uname -m)

    case "$os" in
        Linux)
            case "$arch" in
                x86_64|amd64)
                    target="x86_64-unknown-linux-gnu"
                    ;;
                aarch64|arm64)
                    
                    if [ -n "${TERMUX_VERSION:-}" ] || [ -d "/data/data/com.termux" ]; then
                        target="aarch64-linux-android"
                        install_dir="$PREFIX/bin"
                    elif [ -d "/system/bin" ] && [ -f "/system/bin/linker64" ]; then
                        
                        target="aarch64-linux-android"
                        install_dir="/public/bin"
                    else
                        
                        target="aarch64-unknown-linux-gnu"
                    fi
                    ;;
                armv7l)
                    error "ARM32 not supported yet"
                    ;;
                *)
                    error "Unsupported architecture: $arch"
                    ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64|amd64)
                    target="x86_64-apple-darwin"
                    ;;
                arm64|aarch64)
                    target="aarch64-apple-darwin"
                    ;;
                *)
                    error "Unsupported architecture: $arch"
                    ;;
            esac
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac

    if [ -z "$install_dir" ]; then
        if [ -w "/usr/local/bin" ]; then
            install_dir="/usr/local/bin"
        else
            install_dir="$HOME/.local/bin"
            mkdir -p "$install_dir"
        fi
    fi

    echo "$target|$install_dir"
}

get_latest_version() {
    local version

    if command -v gh >/dev/null 2>&1; then
        version=$(gh release view --repo "$REPO" --json tagName --jq .tagName 2>/dev/null || true)
    fi

    if [ -z "$version" ]; then
        
        version=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
            | grep '"tag_name":' \
            | sed -E 's/.*"([^"]+)".*/\1/' \
            | head -1 || true)
    fi

    if [ -z "$version" ]; then
        error "Could not determine latest version"
    fi

    echo "$version"
}

install_tscode() {
    local target="$1"
    local install_dir="$2"
    local version="$3"

    local archive="tscode-${target}.tar.gz"
    local url="https://github.com/${REPO}/releases/download/${version}/${archive}"
    local tmp_dir

    tmp_dir=$(mktemp -d)
    trap "rm -rf $tmp_dir" EXIT

    info "Downloading $archive..."
    if ! curl -fsSL "$url" -o "$tmp_dir/$archive"; then
        error "Failed to download $url"
    fi

    info "Extracting..."
    tar xzf "$tmp_dir/$archive" -C "$tmp_dir"

    if [ ! -f "$tmp_dir/$BINARY_NAME" ]; then
        error "Binary not found in archive"
    fi

    chmod +x "$tmp_dir/$BINARY_NAME"

    info "Installing to $install_dir..."
    if [ -w "$install_dir" ]; then
        mv "$tmp_dir/$BINARY_NAME" "$install_dir/"
    else
        sudo mv "$tmp_dir/$BINARY_NAME" "$install_dir/"
    fi

    success "Installed $BINARY_NAME to $install_dir/$BINARY_NAME"

    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        local installed_version
        installed_version=$("$BINARY_NAME" --version 2>/dev/null || echo "unknown")
        success "Version: $installed_version"
    else
        warn "Binary installed but not in PATH"
        warn "Add $install_dir to your PATH"
    fi
}

main() {
    echo "======================================"
    echo "  Tscode Installer"
    echo "  Terminal Studio Code"
    echo "======================================"
    echo ""

    info "Detecting platform..."
    local platform
    platform=$(detect_platform)
    local target="${platform%|*}"
    local install_dir="${platform#*|}"
    info "Target: $target"
    info "Install dir: $install_dir"

    info "Getting latest version..."
    local version
    version=$(get_latest_version)
    info "Version: $version"

    echo ""
    install_tscode "$target" "$install_dir" "$version"

    echo ""
    success "Done! Run 'tscode --help' to get started."
}

main "$@"