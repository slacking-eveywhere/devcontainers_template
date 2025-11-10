#!/usr/bin/env bash

PROJECTS_TOP_DIR="$HOME/.local/share/devcontainers"
PROJECTS_BIN_DIR="$HOME/.local/bin"

if [[ -d "$PROJECTS_TOP_DIR" ]]; then
	cd "$PROJECTS_TOP_DIR" || exit
	git pull
else
	cd "$HOME"/.local/share || exit
	git clone --no-checkout https://github.com/slacking-eveywhere/devcontainers_template.git devcontainers
	cd devcontainers || exit
	git sparse-checkout init
	git sparse-checkout set devcontainers bin
	git checkout main
	cd - || exit
fi


cp "$PROJECTS_TOP_DIR"/bin/devcontainer "$PROJECTS_BIN_DIR"/devcontainer
