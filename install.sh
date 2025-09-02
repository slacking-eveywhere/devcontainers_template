#!/usr/bin/env bash

PROJECTS_TOP_DIR="$HOME/.local/share/devcontainers_template/"
PROJECTS_BIN_DIR="$HOME/.local/bin"

if [[ -d "$PROJECTS_TOP_DIR" ]]; then
	cd "$PROJECTS_TOP_DIR" || exit
	git pull
else
	cd "$HOME"/.local/share/ || exit
	git clone https://github.com/slacking-eveywhere/devcontainers_template.git
fi

cp devcontainer.sh "$PROJECTS_BIN_DIR"/devcontainer
