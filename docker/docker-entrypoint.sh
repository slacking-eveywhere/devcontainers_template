#!/usr/bin/env bash
set -eu

# Check if USER and USER_ID variables if setup, exit 1 if not
[[ -z "${USER}" ]] && { echo "USER is not set. Use -e"; exit 1; }
[[ -z "${USER_ID}" ]] && { echo "USER_ID is not set. Use -e"; exit 1; }

# Zoxide repo
ZOXIDE_REPO=https://raw.githubusercontent.com/ajeetdsouza/zoxide/main/install.sh
DOTFILE_REPO=https://github.com/slacking-eveywhere/dotfiles.git

# Custom user creation
export HOME=/home/${USER}
export WORKDIR=/workdir

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

    # Chown uid to new user
    find "${HOME}" \
        -path "${HOME}"/.cache -prune \
        -o -path "${HOME}"/.config -prune \
        -o -path "${HOME}"/.local -prune \
        -o -exec chown -R "${USER}":"${USER}" {} +
else
    # Create new user if both username and UID do not exist
    useradd \
        --shell /usr/bin/zsh \
        --home-dir ${HOME} \
        --uid ${USER_ID} \
        ${USER}
    mkdir -p ${HOME}
    chown ${USER}:${USER} ${HOME} ${WORKDIR}

    # Exec next commmand with gosu
    /usr/local/bin/gosu ${USER} bash -c "\
        cd ${HOME} ; \
        curl -sSfL ${ZOXIDE_REPO} | sh ; \
        git clone ${DOTFILE_REPO} .dotfiles ; \
        cd .dotfiles ; \
        stow --target=${HOME} --adopt */ ;"
fi

exec /usr/local/bin/gosu ${USER} "$@"

exec "$@"