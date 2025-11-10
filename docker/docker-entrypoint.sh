#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

# Check if USER and USER_ID variables if setup, exit 1 if not
[[ -z "$USER" ]] && {
	echo "USER is not set. Use -e"
	exit 1
}
[[ -z "$USER_ID" ]] && {
	echo "USER_ID is not set. Use -e"
	exit 1
}

declare -r SSH_PORT=${SSH_PORT:-2223}
declare -r ENABLE_SSH=${ENABLE_SSH:-false}

declare -r DOCKER_GID=${DOCKER_GID:-""}

# Custom user creation
declare -x HOME="/home/$USER"
declare -x WORKDIR="/workdir"

# Export all variables
export HOME
export WORKDIR

# Check if the user exists
declare USER_EXISTS_ID
USER_EXISTS_ID=$(id -u "$USER" 2>/dev/null || true)

# Check if the UID exists
declare UID_EXISTS
UID_EXISTS=$(getent passwd "$USER_ID" || true)

if [[ -n "$USER_EXISTS_ID" ]]; then
	if [[ "$USER_EXISTS_ID" -ne "$USER_ID" ]]; then
		printf 'User has not the same ID, %s\n' "$USER_ID"
		sudo usermod -u "$USER_ID" "$USER"
	else
	    printf 'Doing nothing for user %s\n' "$USER"
	fi
elif [[ -n "$UID_EXISTS" ]]; then
	EXISTING_USER=$(getent passwd "$USER_ID" | cut -d: -f1)
	# Do some shits to adapts skell user to the new username and uid
	sudo bash -c "
	echo '$USER ALL=(ALL:ALL) NOPASSWD: ALL' | tee /etc/sudoers.d/users > /dev/null
	usermod -l '$USER' '$EXISTING_USER'
	usermod -d '/home/$USER' -m '$USER'
	usermod -s /usr/bin/zsh '$USER'
	usermod -aG sudo '$USER'
	groupmod  --new-name '$USER' '$EXISTING_USER'
	groupmod -g '$USER_ID' '$USER'

	cp /home/skell/JetBrainsMono.tar.xz $HOME/JetBrainsMono.tar.xz
	chown '$USER':'$USER' '$HOME'/JetBrainsMono.tar.xz

	find '$HOME' \
		-path '$HOME/.cache' -prune \
		-o -path '$HOME/.config' -prune \
		-o -path '$HOME/.local' -prune \
		-o -exec chown -R '$USER:$USER' {} +
	
	"
else
	# Create new user if both username and UID do not exist
	sudo useradd \
		--shell /usr/bin/zsh \
		--home-dir "$HOME" \
		--uid "$USER_ID" \
		"$USER"
	sudo mkdir -p "$HOME"

	sudo chown "$USER":"$USER" "$HOME" "$WORKDIR"
fi

exec "$@"
