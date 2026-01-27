#!/usr/bin/env bash

#
# Installation script for devcontainer tool
#
# This script installs the devcontainer tool to:
#   - Binary: ~/.local/bin/devcontainer
#   - Data:   ~/.local/share/devcontainer/{templates,projects}
#

set -o errexit
set -o nounset
set -o pipefail

# --- Constants ---
declare -r INSTALL_BIN_DIR="${HOME}/.local/bin"
declare -r INSTALL_SHARE_DIR="${HOME}/.local/share/devcontainer"
declare -r REPO_URL="https://raw.githubusercontent.com/slacking-eveywhere/devcontainers_template/refs/heads/main"
declare -r SOURCE_BIN="bin/devcontainer"
declare -r SOURCE_DEVCONTAINER_DIR="devcontainer"

# --- Helper Functions ---
# Check if we have required commands
has_command() {
    command -v "$1" >/dev/null 2>&1
}

# Download a file using curl or wget
download_file() {
    local url="$1"
    local dest="$2"

    if has_command curl; then
        curl -sSfL "$url" -o "$dest"
    elif has_command wget; then
        wget -q "$url" -O "$dest"
    else
        echo "ERROR: Neither curl nor wget found. Cannot download files." >&2
        exit 1
    fi
}

# Check if we're running from a local repository
is_local_install() {
    if [[ -f "${SOURCE_BIN}" && -d "${SOURCE_DEVCONTAINER_DIR}" ]]; then
        return 0
    else
        return 1
    fi
}

# --- Main Installation ---
main() {
    # Create installation directories
    mkdir -p "${INSTALL_BIN_DIR}"
    mkdir -p "${INSTALL_SHARE_DIR}"

    if is_local_install; then
        echo "Installing from local repository..."

        # Install binary from local source
        cp -f "${SOURCE_BIN}" "${INSTALL_BIN_DIR}/devcontainer"
        chmod +x "${INSTALL_BIN_DIR}/devcontainer"

        # Install data directories (templates and projects) from local source
        cp -r "${SOURCE_DEVCONTAINER_DIR}/templates" "${INSTALL_SHARE_DIR}/"

    else
        echo "Installing from remote repository..."

        # Download and install binary
        download_file "${REPO_URL}/${SOURCE_BIN}" "${INSTALL_BIN_DIR}/devcontainer"
        chmod +x "${INSTALL_BIN_DIR}/devcontainer"

        # Download and install templates
        download_file "${REPO_URL}/${SOURCE_DEVCONTAINER_DIR}/templates/devcontainer.json" "${INSTALL_SHARE_DIR}/devcontainer.json"
        download_file "${REPO_URL}/${SOURCE_DEVCONTAINER_DIR}/templates/docker-compose-ssh.yml" "${INSTALL_SHARE_DIR}/docker-compose-ssh.yml"
        download_file "${REPO_URL}/${SOURCE_DEVCONTAINER_DIR}/templates/docker-compose-vscode.yml" "${INSTALL_SHARE_DIR}/docker-compose-vscode.yml"

        # Create templates directory and move files there
        mkdir -p "${INSTALL_SHARE_DIR}/templates"
        mv "${INSTALL_SHARE_DIR}/devcontainer.json" "${INSTALL_SHARE_DIR}/templates/"
        mv "${INSTALL_SHARE_DIR}/docker-compose-ssh.yml" "${INSTALL_SHARE_DIR}/templates/"
        mv "${INSTALL_SHARE_DIR}/docker-compose-vscode.yml" "${INSTALL_SHARE_DIR}/templates/"
    fi

    # Check PATH
    if [[ ":${PATH}:" != *":${INSTALL_BIN_DIR}:"* ]]; then
        echo ""
        echo "WARNING: ${INSTALL_BIN_DIR} is not in your PATH"
        echo "Add this to your ~/.bashrc or ~/.zshrc: or whatever ..."
        echo ""
        echo "    export PATH=\"\${HOME}/.local/bin:\${PATH}\""
        echo ""
    fi

    echo "Installation complete"
    echo "Run: ./uninstall.sh to remove"
}

main "$@"
