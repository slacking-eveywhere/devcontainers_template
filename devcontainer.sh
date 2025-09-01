#!/usr/bin/env bash

init() {
	cp .devcontainer "${PWD}"/.devcontainer
}

run() {
	local ENV_FILE SSH_PUBLIC_KEY PROJECT_NAME RUNNING_DEV_CONTAINER_ID

	while [[ "$#" -ne 0 ]]; do
		case "$1" in
		--env-file | -e)
			ENV_FILE="$2"
			shift 2
			;;
		--ssh-key | -s)
			SSH_PUBLIC_KEY="$2"
			shift 2
			;;
		--project-name | -p)
			PROJECT_NAME="$2"
			shift 2
			;;
		*)
			echo "Unknown parameter $1"
			exit 1
			;;
		esac
	done

	if [[ -z "$PROJECT_NAME" ]]; then
		echo "Set a project name -p"
		exit 1
	fi

	echo "Starting compose"
	docker compose \
		-f ssh/docker-compose.yml \
		-p "$PROJECT_NAME" \
		up \
		-d \
		--pull always

	RUNNING_DEV_CONTAINER_ID=$(docker ps -q --filter=name="$PROJECT_NAME"-maindevcontainer-1)
	if [[ -n "$RUNNING_DEV_CONTAINER_ID" ]]; then
		if [[ -f "$SSH_PUBLIC_KEY" ]]; then
			timeout=30
			echo "Waiting for container to start"
			while [ $timeout -gt 0 ]; do
				result=$(docker exec "$RUNNING_DEV_CONTAINER_ID" bash -c 'cat /root/.ready')
				if [ "$result" = "1" ]; then
					break
				fi
				sleep 1
				timeout=$((timeout - 1))
			done
			echo "The container is running at ID: $RUNNING_DEV_CONTAINER_ID"
			USER=$(docker exec "$RUNNING_DEV_CONTAINER_ID" bash -c 'echo $USER')
			echo "Devcontainer user $USER"

			if [[ -f ~/.gitconfig ]]; then
				docker cp ~/.gitconfig "$RUNNING_DEV_CONTAINER_ID":/home/"$USER"/.gitconfig
			fi

			docker cp "$SSH_PUBLIC_KEY" "$RUNNING_DEV_CONTAINER_ID":/home/"$USER"/.ssh/id_rsa.pub
			docker exec "$RUNNING_DEV_CONTAINER_ID" bash -c 'chown $USER:$USER /home/"$USER"/.ssh/id_rsa.pub; \cat /home/"$USER"/.ssh/id_rsa.pub > /home/"$USER"/.ssh/authorized_keys'
		fi
	else
		echo No container running
	fi
}

stop() {
	while [[ "$#" -ne 0 ]]; do
		case "$1" in
		-p)
			PROJECT_NAME="$2"
			shift 2
			;;
		*)
			echo "Unknown parameter $1"
			exit 1
			;;
		esac
	done
	docker compose -p "$PROJECT_NAME" down
}

connect() {
	local PORT SSH_HOST

	while [[ "$#" -ne 0 ]]; do
		case "$1" in
		--port)
			PORT="$2"
			shift 2
			;;
		--host)
			SSH_HOST="$2"
			shift 2
			;;
		*)
			echo "Unknown parameter $1"
			exit 1
			;;
		esac
	done
	ssh -A -p "$PORT" "$SSH_HOST"
}

open_file() {
	local FILE="$1"
	if command -v xdg-open >/dev/null 2>&1; then
		xdg-open "$FILE"
	elif command -v open >/dev/null 2>&1; then
		open "$FILE"
	else
		echo "No file editor found."
	fi
}

new() {
	local PROJECT_NAME="$1"

	local ENV_FILE_TEMPLATE COMPOSE_FILE_TEMPLATE PROJECT_DIR FORCE

	PROJECT_DIR="$PROJECTS_TOP_DIR"/"$PROJECT_NAME"
	FORCE="false"

	ENV_FILE_TEMPLATE="$PROJECT_DIR"/.env
	COMPOSE_FILE_TEMPLATE="$PROJECT_DIR"/docker-compose.yml

	shift 1
	while [[ "$#" -ne 0 ]]; do
		case "$1" in
		-f)
			FORCE="true"
			shift 1
			;;
		esac
	done

	if [[ ! -d "$PROJECT_DIR" ]]; then
		mkdir "$PROJECT_DIR"
	fi

	if [[ ! -f "$ENV_FILE_TEMPLATE" ]] || [[ "$FORCE" == "true" ]]; then
		cat <<-EOF >"$ENV_FILE_TEMPLATE"
			USER=
			USER_ID=

			DEVCONTAINER_IMAGE_NAME=

			RESOURCES_RAM=
		EOF
	else
		echo "The file $ENV_FILE_TEMPLATE exists. Not overwritting"
	fi

	if [[ ! -f "$COMPOSE_FILE_TEMPLATE" ]] || [[ "$FORCE" == "true" ]]; then
		cat <<-EOF >"$COMPOSE_FILE_TEMPLATE"
			services:
			    maindevcontainer:
			        ports:
			            - "2202:22"
			            - "8081:8080"
		EOF
	else
		echo "The file $COMPOSE_FILE_TEMPLATE exists. Not overwritting"
	fi
}

edit() {
	local PROJECT_NAME="$1"

	local PROJECT_DIR ENV_FILE_TEMPLATE COMPOSE_FILE_TEMPLATE EDIT_COMPOSE

	PROJECT_DIR="$PROJECTS_TOP_DIR"/"$PROJECT_NAME"

	EDIT_COMPOSE="false"

	shift 1
	while [[ "$#" -ne 0 ]]; do
		case "$1" in
		-c)
			EDIT_COMPOSE="true"
			shift 1
			;;
		esac
	done

	if [[ ! -d "$PROJECT_DIR" ]]; then
		echo "No project dir named : $PROJECT_NAME"
		exit 1
	fi
	ENV_FILE_TEMPLATE="$PROJECT_DIR"/.env
	COMPOSE_FILE_TEMPLATE="$PROJECT_DIR"/docker-compose.yml

	if [[ "$EDIT_COMPOSE" == "true" ]]; then
		open_file "$COMPOSE_FILE_TEMPLATE"
	else
		open_file "$ENV_FILE_TEMPLATE"
	fi

}

PROJECTS_TOP_DIR="$HOME/.local/share/devcontainer/"
DEVCONTAINER_ROOT_FOLDER="devcontainers_template"

case $1 in
init)
	init
	;;
list)
	docker compose ls --format json |
		jq -c --arg folder "$DEVCONTAINER_ROOT_FOLDER" '.[] | select( .ConfigFiles | contains($folder))' |
		while read -r object; do
			name=$(echo "$object" | jq -r '.Name')
			status=$(echo "$object" | jq -r '.Status')
			echo "$name | $status"
		done
	;;
new)
	shift 1
	new "$@"
	;;
edit)
	shift 1
	edit "$@"
	;;
run)
	shift 1
	run "$@"
	;;
stop)
	shift 1
	stop "$@"
	;;
connect)
	shift 1
	connect "$@"
	;;
*)
	echo "Unknown parameter $1"
	exit 1
	;;
esac
