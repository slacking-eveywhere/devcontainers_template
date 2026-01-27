#!/usr/bin/env bash

set -e

# Zoxide repo
declare -r ZOXIDE_REPO=https://raw.githubusercontent.com/ajeetdsouza/zoxide/main/install.sh
declare -r DOTFILE_REPO=https://github.com/slacking-eveywhere/dotfiles.git

if [[ ! -f "$HOME"/.installed ]]; then
    # Install fonts
    FONT_DIR="$HOME"/.local/share/fonts
    mkdir -p "$FONT_DIR"/.local/share/fonts
    if [[ -f "$HOME"/JetBrainsMono.tar.xz ]]; then
        tar -xf "$HOME"/JetBrainsMono.tar.xz -C "$FONT_DIR"
        rm -rf "$HOME"/JetBrainsMono.tar.xz
    fi

    # Install zoxide globally
    curl -sSfL "$ZOXIDE_REPO" | sh

    cd "$HOME"
    fc-cache -fv

    if [[ -n "$DOTFILE_REPO" ]]; then
        if [[ ! -d .dotfiles ]]; then
            git clone "$DOTFILE_REPO" .dotfiles
        fi

        cd .dotfiles
        git pull
        stow --target="$HOME" --adopt -- */
        cd -
    fi

    touch "$HOME"/.installed
fi

if [[ -n "$DOCKER_GID" ]]; then
    if getent group docker >/dev/null; then
        sudo groupmod -g "$DOCKER_GID" docker || true
    else
        sudo groupadd -g "$DOCKER_GID" docker
    fi
    sudo usermod -aG docker "$USER"
fi

# Remove useless and strange symlink fucking up with docker buildx.
if [[ -f "$HOME"/.docker/cli-plugins/docker-buildx ]]; then
    sudo rm -rf "$HOME"/.docker/cli-plugins/docker-buildx
fi

if [[ -f "/usr/sbin/sshd" ]] && [[ "$ENABLE_SSH" == "true" ]]; then
    printf "Starting sshd as daemon for remote server on port %s ...\n" "$SSH_PORT"
    mkdir -p "$HOME"/.ssh
    touch "$HOME"/.ssh/authorized_keys

    if [[ -z $(ls -A /etc/ssh/ssh_host_* 2>/dev/null) ]]; then
        ssh-keygen -A
    fi

    sudo /usr/sbin/sshd
fi