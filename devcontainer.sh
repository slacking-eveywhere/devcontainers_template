#!/usr/bin/env bash

init() {
	cp .devcontainer "${PWD}"/.devcontainer
}

run() {
	local SSH_PUBLIC_KEY PROJECT_NAME RUNNING_DEV_CONTAINER_ID USER

	while getopts ":s:p:" opt; do
		case ${opt} in
		s) SSH_PUBLIC_KEY=${OPTARG} ;;
		p) PROJECT_NAME=${OPTARG} ;;
		\?)
			echo "Invalid option: -${OPTARG}" >&2
			exit 1
			;;
		:)
			echo "Option -${OPTARG} requires an argument." >&2
			exit 1
			;;
		esac
	done

	if [[ -z "$PROJECT_NAME" ]]; then
		echo "Set a project name -p"
		exit 1
	fi
	local project_compose_file="$PROJECTS_TOP_DIR"/"$PROJECT_NAME"/docker-compose.yml
	if [[ ! -f "$project_compose_file" ]]; then
		echo "Project compose file not found at $project_compose_file"
		exit 1
	fi

	echo "Starting compose"
	docker compose \
		-f "$TOP_DIR"/templates/docker-compose-ssh.yml \
		-f "$PROJECTS_TOP_DIR"/"$PROJECT_NAME"/docker-compose.yml \
		-p "$PROJECT_NAME" \
		up \
		-d \
		--pull always

	RUNNING_DEV_CONTAINER_ID=$(docker ps -q --filter=name="$PROJECT_NAME"-maindevcontainer-1)
	if [[ -n "$RUNNING_DEV_CONTAINER_ID" ]]; then
		if [[ -f "$SSH_PUBLIC_KEY" ]]; then
			timeout=30
			is_running="false"

			echo -n "Waiting for container to start "
			while [ $timeout -gt 0 ]; do
				result=$(docker exec "$RUNNING_DEV_CONTAINER_ID" bash -c 'cat /root/.ready' 2>/dev/null)
				if [ "$result" = "1" ]; then
					is_running="true"
					break
				fi
				printf "|"
				sleep 1
				timeout=$((timeout - 1))
			done
			echo ""

			if [[ "$is_running" == "false" ]]; then
				echo "Container did not start. Something went wrong."
				echo "Check your entrypoint's logs."
				exit 1
			fi

			echo "The container is running at ID: $RUNNING_DEV_CONTAINER_ID"
			USER=$(docker exec "$RUNNING_DEV_CONTAINER_ID" bash -c 'echo $USER')

			if [[ -z "$USER" ]]; then
				echo "Can't retrieve user from container"
				exit 1
			fi

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
	local PROJECT_NAME
	while getopts ":p:" opt; do
		case ${opt} in
		p) PROJECT_NAME=${OPTARG} ;;
		\?)
			echo "Invalid option: -${OPTARG}" >&2
			exit 1
			;;
		:)
			echo "Option -${OPTARG} requires an argument." >&2
			exit 1
			;;
		esac
	done
	docker compose -p "$PROJECT_NAME" down
}

connect() {
	local PORT SSH_HOST

	while getopts ":p:h:" opt; do
		case ${opt} in
		p) PORT=${OPTARG} ;;
		h) SSH_HOST=${OPTARG} ;;
		\?)
			echo "Invalid option: -${OPTARG}" >&2
			exit 1
			;;
		:)
			echo "Option -${OPTARG} requires an argument." >&2
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

	if [[ -z "$PROJECT_NAME" ]]; then
		echo "Missing projet name"
		exit 1
	fi

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
			        env_file:
			          - $ENV_FILE_TEMPLATE
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

inspect() {
	local PROJECT_NAME="$1"
	docker compose \
		-f "$TOP_DIR"/templates/docker-compose-ssh.yml \
		-f "$PROJECTS_TOP_DIR"/"$PROJECT_NAME"/docker-compose.yml \
		config
}

help() {
	cat <<-EOF
		Usage: devcontainer.sh <command> [<args>]

		Commands:
		  init                      Copy .devcontainer folder to the current directory
		  new <project_name> [-f]   Create a new project
		  edit <project_name> [-c]  Edit a project's .env or docker-compose.yml file
		  inspect <project_name>    Display aggregated docker compose (eg. template and modified compose) with config method
		  run                       Run a devcontainer
		  stop                      Stop a devcontainer
		  list                      List running devcontainers
		  connect                   Connect to a running devcontainer via SSH
		  help                      Show this help message
	EOF
}

check_prerequisites() {
	if ! command -v jq >/dev/null 2>&1; then
		echo "jq is not installed. Please install it to use the list command."
		exit 1
	fi
}

TOP_DIR="$HOME/.local/share/devcontainers_template"
PROJECTS_TOP_DIR="$HOME/.local/share/devcontainers_template/projects"
DEVCONTAINER_ROOT_FOLDER="devcontainers_template"

if [[ ! -d "$PROJECTS_TOP_DIR" ]]; then
	mkdir "$PROJECTS_TOP_DIR"
fi

case $1 in
init)
	init
	;;
list)
	check_prerequisites
	docker compose ls --format json |
		jq -c --arg folder "$DEVCONTAINER_ROOT_FOLDER" '.[] | select( .ConfigFiles | contains($folder))' |
		while read -r object; do
			name=$(echo "$object" | jq -r '.Name')
			status=$(echo "$object" | jq -r '.Status')
			echo "$name | $status"
		done
	;;
inspect)
	shift 1
	check_prerequisites
	inspect "$@"
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
help)
	help
	;;
*)
	help
	exit 1
	;;
esac
