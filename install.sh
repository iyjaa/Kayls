#!/usr/bin/env bash

set -eo pipefail

# --- Palet Warna Vesper ---
BLUE='\033[38;5;111m'
GREEN='\033[38;5;114m'
CYAN='\033[38;5;116m'
GRAY='\033[38;5;242m'
YELLOW='\033[38;5;222m'
BOLD='\033[1m'
NC='\033[0m'

info() { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[ OK ]${NC} $1"; }
error() { echo -e "${RED}[FAIL]${NC} $1" >&2; exit 1; }

progress_bar() {
    local current=$1
    local total=$2
    local width=30
    local percent=$(( current * 100 / total ))
    local filled=$(( percent * width / 100 ))
    local empty=$(( width - filled ))

    printf "${CYAN}["
    printf "%${filled}s" | tr ' ' '█'
    printf "%${empty}s" | tr ' ' '-'
    printf "]${NC} %3d%% (${GRAY}%d/%d bytes${NC})\r" "$percent" "$current" "$total"
}

download_file() {
    local url=$1
    local output=$2
    local total_size=$(curl -sI "$url" | grep -i "content-length:" | tail -1 | tr -dc '0-9')
    
    if [ -z "$total_size" ] || [ "$total_size" -eq 0 ]; then
        info "Downloading (unknown size)..."
        curl -fsSL "$url" -o "$output"
        return
    fi

    curl -fsSL "$url" -o "$output" &
    local pid=$!
    
    while kill -0 $pid 2>/dev/null; do
        if [ -f "$output" ]; then
            local current_size=$(stat -c%s "$output" 2>/dev/null || echo 0)
            progress_bar "$current_size" "$total_size"
        fi
        sleep 0.1
    done
    
    wait $pid
    progress_bar "$total_size" "$total_size"
    echo ""
}

spinner() {
    local text=$1
    local pid=$!
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " ${CYAN}[%c]${NC} %s\r" "$spinstr" "$text"
        local spinstr=$temp${spinstr%"$temp"}
        sleep 0.1
    done
    printf "\r\033[K"
}

REPO="iyjaa/Kayls"
BINARY_NAME="kayls"
INSTALL_DIR=""

detect_platform() {
    local os arch target install_dir
    os=$(uname -s)
    arch=$(uname -m)
    case "$os" in
        Linux)
            case "$arch" in
                x86_64|amd64) target="x86_64-unknown-linux-gnu" ;;
                aarch64|arm64)
                    if [ -n "${TERMUX_VERSION:-}" ] || [ -d "/data/data/com.termux" ]; then
                        target="aarch64-linux-android"
                        install_dir="${PREFIX:-/data/data/com.termux/files/usr}/bin"
                    else
                        target="aarch64-unknown-linux-gnu"
                    fi ;;
                *) error "Unsupported architecture: $arch" ;;
            esac ;;
        Darwin)
            case "$arch" in
                x86_64|amd64) target="x86_64-apple-darwin" ;;
                arm64|aarch64) target="aarch64-apple-darwin" ;;
                *) error "Unsupported OS: $os" ;;
            esac ;;
        *) error "Unsupported OS: $os" ;;
    esac
    if [ -z "$install_dir" ]; then
        if [ -w "/usr/local/bin" ]; then install_dir="/usr/local/bin"
        else install_dir="$HOME/.local/bin"; mkdir -p "$install_dir"; fi
    fi
    echo "$target|$install_dir"
}

get_latest_version() {
    local version=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' | head -1)
    if [ -z "$version" ]; then error "Could not determine latest version"; fi
    echo "$version"
}

main() {
    clear
    echo -e "${CYAN}${BOLD}======================================${NC}"
    echo -e "${CYAN}${BOLD}      Kayls Installer                 ${NC}"
    echo -e "${CYAN}${BOLD}======================================${NC}"
    echo ""
    echo -e "  ${YELLOW}${BOLD}Kayls-Terminal-Editor${NC}"
    echo ""
    echo -e "${GRAY}──────────────────────────────────────${NC}"
    echo ""

    info "Detecting platform..."
    local platform=$(detect_platform)
    local target="${platform%|*}"
    local install_dir="${platform#*|}"
    success "Target:      ${BOLD}$target${NC}"
    success "Install dir: ${BOLD}$install_dir${NC}"
    echo ""

    info "Fetching latest release..."
    local version=$(get_latest_version)
    success "Version:     ${BOLD}$version${NC}"
    echo ""

    local archive="${BINARY_NAME}-${target}.tar.gz"
    local url="https://github.com/${REPO}/releases/download/${version}/${archive}"
    local tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT

    info "Downloading ${BOLD}$archive${NC}..."
    echo ""
    download_file "$url" "$tmp_dir/$archive"
    success "Downloaded."
    echo ""

    info "Extracting binary..."
    (tar xzf "$tmp_dir/$archive" -C "$tmp_dir") &
    spinner "Extracting..."
    success "Extracted."
    echo ""

    info "Installing to ${BOLD}$install_dir${NC}..."
    mkdir -p "$install_dir" 2>/dev/null || true
    if [ -w "$install_dir" ] || [ "$(id -u)" = "0" ]; then
        mv "$tmp_dir/$BINARY_NAME" "$install_dir/"
    elif command -v sudo >/dev/null 2>&1; then
        sudo mkdir -p "$install_dir"
        sudo mv "$tmp_dir/$BINARY_NAME" "$install_dir/"
    else
        install_dir="$HOME/bin"
        mkdir -p "$install_dir"
        mv "$tmp_dir/$BINARY_NAME" "$install_dir/"
    fi
    success "Installed $BINARY_NAME."
    echo ""

    echo -e "${GRAY}──────────────────────────────────────${NC}"
    echo -e "${GREEN}${BOLD} Installation Complete!${NC}"
    echo ""
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        echo -e "  Run ${BOLD}kayls --help${NC} to get started."
        echo -e "  Installed version: ${GREEN}$($BINARY_NAME --version)${NC}"
    fi
    echo ""
}

main "$@"