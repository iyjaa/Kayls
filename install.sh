#!/usr/bin/env bash
set -eo pipefail

# Warna ANSI
C_INFO="\033[38;5;111m"
C_OK="\033[38;5;114m"
C_ERR="\033[38;5;203m"
C_WARN="\033[38;5;222m"
C_TITLE="\033[38;5;222m"
C_DIM="\033[38;5;242m"
C_RESET="\033[0m"

info() { printf "${C_INFO}[INFO]${C_RESET} %s\n" "$1"; }
success() { printf "${C_OK}[ OK ]${C_RESET} %s\n" "$1"; }
error() { printf "${C_ERR}[FAIL]${C_RESET} %s\n" "$1" >&2; exit 1; }
warn() { printf "${C_WARN}[WARN]${C_RESET} %s\n" "$1"; }

progress_bar() {
    local current=$1
    local total=$2
    local width=30
    local percent=$(( current * 100 / total ))
    local filled=$(( percent * width / 100 ))
    local empty=$(( width - filled ))

    printf "${C_INFO}["
    printf "%${filled}s" | tr ' ' '█'
    printf "%${empty}s" | tr ' ' '-'
    printf "]${C_RESET} %3d%%\r" "$percent"
}

download_file() {
    local url=$1
    local output=$2
    local total_size=$(curl -sIL "$url" | grep -i "content-length:" | tail -1 | tr -dc '0-9')
    
    if [ -z "$total_size" ] || [ "$total_size" -eq 0 ]; then
        printf "${C_INFO}[INFO]${C_RESET} Downloading (size unknown)...\n"
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
    printf "\n"
}

spinner() {
    local pid=$!
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " ${C_INFO}[%c]${C_RESET} Extracting...\r" "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep 0.1
    done
    printf "\r\033[K"
}

REPO="iyjaa/Kayls"
BINARY_NAME="kayls"

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
    printf "${C_DIM}──────────────────────────────────────────────────${C_RESET}\n"
    printf "  ${C_TITLE}K A Y L S${C_RESET}  Installer\n"
    printf "  ${C_DIM}Terminal Studio Code${C_RESET}\n"
    printf "${C_DIM}──────────────────────────────────────────────────${C_RESET}\n\n"

    info "Detecting platform..."
    local platform=$(detect_platform)
    local target="${platform%|*}"
    local install_dir="${platform#*|}"
    success "Target:      $target"
    success "Install dir: $install_dir"
    printf "\n"

    info "Fetching latest release..."
    local version=$(get_latest_version)
    success "Version:     $version"
    printf "\n"

    local archive="${BINARY_NAME}-${target}.tar.gz"
    local url="https://github.com/${REPO}/releases/download/${version}/${archive}"
    local tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT

    printf "${C_INFO}[INFO]${C_RESET} Downloading %s\n" "$archive"
    download_file "$url" "$tmp_dir/$archive"
    success "Downloaded."

    # Verifikasi checksum (jika tersedia)
    local checksum_url="https://github.com/${REPO}/releases/download/${version}/checksums.sha256"
    if curl -fsSL "$checksum_url" -o "$tmp_dir/checksums.sha256" 2>/dev/null; then
        local expected=$(grep "$archive" "$tmp_dir/checksums.sha256" | awk '{print $1}')
        if [ -n "$expected" ]; then
            local actual=$(sha256sum "$tmp_dir/$archive" | awk '{print $1}')
            if [ "$expected" != "$actual" ]; then
                error "Checksum mismatch for $archive!"
            fi
            success "Checksum verified."
        else
            warn "Checksum not found for $archive, skipping verification."
        fi
    else
        warn "Checksums file not available, skipping verification."
    fi
    printf "\n"

    printf "${C_INFO}[INFO]${C_RESET} Extracting binary...\n"
    (tar xzf "$tmp_dir/$archive" -C "$tmp_dir") &
    spinner
    chmod +x "$tmp_dir/$BINARY_NAME"
    success "Extracted."
    printf "\n"

    info "Installing to $install_dir..."
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
    printf "\n"

    printf "${C_DIM}──────────────────────────────────────────────────${C_RESET}\n"
    printf "  ${C_OK}Installation Complete${C_RESET}\n\n"
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        printf "  Run ${C_TITLE}kayls --help${C_RESET} to get started.\n"
        printf "  Installed version: ${C_OK}%s${C_RESET}\n" "$($BINARY_NAME --version)"
    fi
    printf "\n"
}

main "$@"
