#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

function exit_trap() {
    printf 'Test failed: %s\n' "$1"
    rm -rf devcontainers/projects/test1
}

trap 'exit_trap "Script interrupted."' INT TERM EXIT

./devcontainer new test1

sed -i "s/USER=/USER=$(id -u)/g" devcontainers/projects/test1/.env
sed -i 's/USER_ID=/USER_ID=1000/g' devcontainers/projects/test1/.env
sed -i 's/DEVCONTAINER_IMAGE_NAME=/DEVCONTAINER_IMAGE_NAME=devcontainer-golang:1.23.4/g' devcontainers/projects/test1/.env
sed -i 's/ENABLE_SSH=true/ENABLE_SSH=false/g' devcontainers/projects/test1/.env
sed -i 's/SSH_PORT=/SSH_PORT=22228/g' devcontainers/projects/test1/.env

./devcontainer run -p test1

sleep 15
LIST_RESULT=$(./devcontainer list)
if echo "$LIST_RESULT" | grep -q 'Project: test1 | Status: running(1)'; then
    printf 'Test passed: Devcontainer is running.\n'
else
    exit_trap "Devcontainer is not running as expected."
fi

ECHO_RESULT=$(docker exec -it test1-maindevcontainer-1 bash -c 'echo HELLO')

if [[ "$ECHO_RESULT" == "HELLO" ]]; then
    printf 'Test passed: Command executed successfully inside the container.\n'
else
    exit_trap "Command execution inside the container failed."
fi

./devcontainer stop test1

