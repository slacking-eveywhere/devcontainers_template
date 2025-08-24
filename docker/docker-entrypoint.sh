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
USER_EXISTS=$(id -u "$USER" 2>/dev/null || true)

# Check if the UID exists
UID_EXISTS=$(getent passwd "$USER_ID" || true)

if [[ -n "${USER_EXISTS}" ]]; then
	if [[ "${USER_EXISTS}" -ne "$USER_ID" ]]; then
		usermod -u "$USER_ID" "$USER"
	fi
elif [[ -n "${UID_EXISTS}" ]]; then
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

	# Install fonts
	mkdir -p "$HOME"/.fonts
	tar -xf /root/JetBrainsMono.tar.xz -C /"$HOME"/.fonts

	chown "$USER":"$USER" "$HOME" "$WORKDIR" /"$HOME"/.fonts

	# Exec next commmand with gosu
	/usr/local/bin/gosu "$USER" bash -c "\
        cd $HOME ; \
        fc-cache -fv ; \
        curl -sSfL $ZOXIDE_REPO | sh ; \
        git clone $DOTFILE_REPO .dotfiles ; \
        cd .dotfiles ; \
        stow --target=$HOME --adopt */ ;"
fi

if [[ ! -z "$DOCKER_GID" ]]; then
	if getent group "$DOCKER_GID" >/dev/null; then
		echo "Un groupe avec le GID $DOCKER_GID existe déjà : $(getent group "$DOCKER_GID" | cut -d: -f1)"
	else
		groupadd -g "$DOCKER_GID" docker
		usermod -aG docker $USER
	fi
fi

if [[ -f "/usr/sbin/sshd" ]]; then
	echo "Starting sshd as daemon for remote server"
	mkdir "$HOME"/.ssh
	touch "$HOME"/.ssh/authorized_keys

	chown -R "$USER":"$USER" "$HOME"/.ssh
	if [ -z "$(ls -A /etc/ssh/ssh_host_* 2>/dev/null)" ]; then
		ssh-keygen -A
	fi
	/usr/sbin/sshd
fi

echo 1 >/root/.ready

exec /usr/local/bin/gosu "$USER" "$@"
