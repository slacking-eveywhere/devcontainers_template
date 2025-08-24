#!/usr/bin/env bash

init() {
	cp .devcontainer "${PWD}"/.devcontainer
}

build() {
	local LANGUAGE_NAME REGISTRY TAG PUSH COMMON

	PUSH="false"
	COMMON="false"

	while [[ "$#" -ne 0 ]]; do
		case "$1" in
		--language-name)
			LANGUAGE_NAME="$2"
			shift 2
			;;
		--registry)
			REGISTRY="$2"

			if [[ "${REGISTRY: -1}" != "/" ]]; then
				REGISTRY="$REGISTRY"/
			fi
			shift 2
			;;
		--push)
			PUSH="true"
			shift 1
			;;
		--common)
			COMMON="true"
			shift 1
			;;
		*)
			echo "Unknown parameter $1"
			exit 1
			;;
		esac
	done

	if [[ "$COMMON" == "true" ]]; then
		for target in common common-ssh; do
			docker build \
				-t "$REGISTRY$target" \
				--target="$target" \
				--progress=plain \
				-f docker/Dockerfiles/common.Dockerfile \
				docker

			if [[ -n "$REGISTRY" ]] && [[ "$PUSH" == "true" ]]; then
				echo "Pushing to image $TAG"
				docker push "$TAG"
			fi
		done
	fi

	if [[ -z "$LANGUAGE_NAME" ]]; then
		echo "No language name set"
		exit 1
	fi

	if [[ ! -f docker/env/"$LANGUAGE_NAME".env ]]; then
		echo "No .env file for $LANGUAGE_NAME"
	fi

	for target in "$LANGUAGE_NAME" "$LANGUAGE_NAME"-ssh; do
		TAG="$REGISTRY"devcontainer-"$target"
		if docker build \
			-t "$TAG" \
			--target="$target" \
			--build-arg=REGISTRY="$REGISTRY" \
			$(cat docker/env/"$LANGUAGE_NAME".env | sed 's@^@--build-arg @g' | paste -s -d " " -) \
			-f docker/Dockerfiles/"$LANGUAGE_NAME".Dockerfile \
			--progress=plain \
			docker; then

			if [[ -n "$REGISTRY" ]] && [[ "$PUSH" == "true" ]]; then
				echo "Pushing to image $TAG"
				docker push "$TAG"
			fi
		fi

		echo "Image built with tag $TAG"
	done
}

run() {
	local ENV_FILE SSH_PUBLIC_KEY PROJECT_NAME RUNNING_DEV_CONTAINER_ID

	while [[ "$#" -ne 0 ]]; do
		case "$1" in
		--env-file)
			ENV_FILE="$2"
			shift 2
			;;
		--ssh-key)
			SSH_PUBLIC_KEY="$2"
			shift 2
			;;
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

	if [[ -z "$PROJECT_NAME" ]]; then
		echo "Set a project name -p"
		exit 1
	fi

	echo "Starting compose"
	docker compose -p "$PROJECT_NAME" up -d --pull always

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
	ssh
}

case $1 in
init)
	init
	;;
build)
	shift 1
	build "$@"
	;;
run)
	shift 1
	run "$@"
	;;
stop)
	shift 1
	stop "$@"
	;;
*)
	echo "Unknown parameter $1"
	exit 1
	;;
esac

# build common
# build golang
# build python
# build rust
# build ansible
