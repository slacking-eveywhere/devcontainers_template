#!/usr/bin/env bash
set -eux

export HOME=/home/${USER}

useradd \
    --shell /usr/bin/zsh \
    --home-dir /home/"${USER}" \
    --uid "${USER_ID}" \
    "${USER}" ;


mkdir -p /home/"${USER}"/.config ;
mkdir -p /home/"${USER}"/.cache ;
mkdir -p /home/"${USER}"/.local ;
chown "${USER}" /home/"${USER}"/.config ;
chown "${USER}" /home/"${USER}"/.cache ;
chown "${USER}" /home/"${USER}"/.local ;
find /skel -maxdepth 1 -mindepth 1 -exec cp -r {} /home/"${USER}" \;
find /home/"${USER}" \
    -path /home/"${USER}"/projects -prune \
    -o -path /home/"${USER}"/.cache -prune \
    -o -path /home/"${USER}"/.local -prune \
    -o -exec chown -R "${USER}" {} +

"$@"
