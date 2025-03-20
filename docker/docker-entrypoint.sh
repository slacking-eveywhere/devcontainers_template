#!/usr/bin/env bash
set -eu

# Check if USER and USER_ID variables if setup, exit 1 if not
[[ -z "${USER}" ]] && { echo "USER is not set. Use -e"; exit 1; }
[[ -z "${USER_ID}" ]] && { echo "USER_ID is not set. Use -e"; exit 1; }

# Custom user creation 
export HOME=/home/${USER}

# Check if the user exists
USER_EXISTS=$(id -u "${USER}" 2>/dev/null || true)

# Check if the UID exists
UID_EXISTS=$(getent passwd "${USER_ID}" || true)

if [[ -n "${USER_EXISTS}" ]]; then
    if [[ "${USER_EXISTS}" -ne "${USER_ID}" ]]; then
        usermod -u "${USER_ID}" "${USER}"
    fi
elif [[ -n "${UID_EXISTS}" ]]; then
    EXISTING_USER=$(getent passwd "${USER_ID}" | cut -d: -f1)
    usermod -l "${USER}" "${EXISTING_USER}"
    usermod -d "/home/${USER}" -m "${USER}"
else
    # Create new user if both username and UID do not exist
    useradd \
        --shell /usr/bin/zsh \
        --home-dir /home/"${USER}" \
        --uid "${USER_ID}" \
        "${USER}"
    mkdir -p "${HOME}"
    chown "${USER}":"${USER}" /workdir
fi


# Use root as skel to populate custom user
find /root/ \
    -mindepth 1 \
    -maxdepth 1 \
    -exec cp -r {} "${HOME}" \;

# Chown root to new user
find "${HOME}" \
    -path "${HOME}"/.cache -prune \
    -o -path "${HOME}"/.config -prune \
    -o -path "${HOME}"/.local -prune \
    -o -exec chown -R "${USER}":"${USER}" {} +

# Remove root folder content
rm -rf /root/*
rm -rf /root/.*

# Exec next commmand with gosu
exec /usr/local/bin/gosu ${USER} "$@"

exec "$@"