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

# Check if HOME directory ownership matches USER_ID
# if [[ -d "$HOME" ]]; then
# 	CURRENT_OWNER_ID=$(stat -c '%u' "$HOME")
# 	if [[ "$CURRENT_OWNER_ID" -ne "$USER_ID" ]]; then
# 		printf 'HOME directory ownership mismatch. Fixing ownership for %s\n' "$USER"
# 		sudo chown -R "$USER":"$USER" "$HOME"
# 	fi
# fi

# Create user with specified USER and USER_ID
sudo bash -c "
echo '$USER ALL=(ALL:ALL) NOPASSWD: ALL' | tee /etc/sudoers.d/users > /dev/null

useradd \
	--shell /usr/bin/zsh \
	--home-dir '$HOME' \
	--uid '$USER_ID' \
	'$USER'
mkdir -p '$HOME'
chown '$USER':'$USER' '$HOME' '$WORKDIR'
"

exec "$@"
