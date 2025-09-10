#!/usr/bin/env bash
set -e

# Check if USER and USER_ID variables if setup, exit 1 if not
[[ -z "$USER" ]] && {
	echo "USER is not set. Use -e"
	exit 1
}
[[ -z "$USER_ID" ]] && {
	echo "USER_ID is not set. Use -e"
	exit 1
}

# Zoxide repo
ZOXIDE_REPO=https://raw.githubusercontent.com/ajeetdsouza/zoxide/main/install.sh
DOTFILE_REPO=https://github.com/slacking-eveywhere/dotfiles.git

# Custom user creation
export HOME=/home/$USER
export WORKDIR=/workdir

# Check if the user exists
USER_EXISTS_ID=$(id -u "$USER" 2>/dev/null || true)

# Check if the UID exists
UID_EXISTS=$(getent passwd "$USER_ID" || true)

if [[ -n "$USER_EXISTS_ID" ]]; then
	if [[ "$USER_EXISTS_ID" -ne "$USER_ID" ]]; then
		echo "User has not the same ID, $USER_ID"
		usermod -u "$USER_ID" "$USER"
	fi
elif [[ -n "$UID_EXISTS" ]]; then
	EXISTING_USER=$(getent passwd "$USER_ID" | cut -d: -f1)
	usermod -l "$USER" "${EXISTING_USER}"
	usermod -d "/home/$USER" -m "$USER"

	# Chown uid to new user
	find "$HOME" \
		-path "$HOME"/.cache -prune \
		-o -path "$HOME"/.config -prune \
		-o -path "$HOME"/.local -prune \
		-o -exec chown -R "$USER":"$USER" {} +
else
	# Create new user if both username and UID do not exist
	useradd \
		--shell /usr/bin/zsh \
		--home-dir "$HOME" \
		--uid "$USER_ID" \
		"$USER"
	mkdir -p "$HOME"

	chown "$USER":"$USER" "$HOME" "$WORKDIR"

fi

if [[ ! -f "$HOME"/.installed ]]; then
	# Install fonts
	FONT_DIR="$HOME"/.local/share/fonts
	mkdir -p "$FONT_DIR"/.local/share/fonts
	tar -xf /root/JetBrainsMono.tar.xz -C "$FONT_DIR"

	chown "$USER":"$USER" -R "$HOME"

	# Install zoxide globally
	curl -sSfL "$ZOXIDE_REPO" | sh

	# Exec next commmand with gosu
	# shellcheck disable=SC2016
	/usr/local/bin/gosu "$USER" bash -c '
        set -e

        dotfile_repo="$1"
        home="$2"

        cd "$home"
        fc-cache -fv

        if [ ! -d .dotfiles ]; then
            git clone "$dotfile_repo" .dotfiles
        fi

        cd .dotfiles
        git pull
        stow --target="$home" --adopt */
    ' bash "$DOTFILE_REPO" "$HOME"

	touch "$HOME"/.installed
fi

if [[ ! -z "$DOCKER_GID" ]]; then
	if getent group docker >/dev/null; then
		groupmod -g "$DOCKER_GID" docker || true
	else
		groupadd -g "$DOCKER_GID" docker
	fi
	usermod -aG docker $USER
fi

# Remove useless and strange symlink fucking up with docker buildx.
if [[ -f "$HOME"/.docker/cli-plugins/docker-buildx ]]; then
	rm -rf "$HOME"/.docker/cli-plugins/docker-buildx
fi

if [[ -f "/usr/sbin/sshd" ]]; then
	echo "Starting sshd as daemon for remote server"
	mkdir -p "$HOME"/.ssh
	touch "$HOME"/.ssh/authorized_keys

	chown -R "$USER":"$USER" "$HOME"/.ssh
	if [ -z "$(ls -A /etc/ssh/ssh_host_* 2>/dev/null)" ]; then
		ssh-keygen -A
	fi
	/usr/sbin/sshd
fi

echo 1 >/root/.ready

exec /usr/local/bin/gosu "$USER" "$@"
