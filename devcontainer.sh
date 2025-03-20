#!/usr/bin/env bash

init() {
    cp .devcontainer ${PWD}/.devcontainer

    
}

build() {
    local langage_name=$1

    docker build \
    -t devcontainer-${langage_name} \
    --target=${langage_name} \
    $(cat docker/env/${langage_name}.env | sed 's@^@--build-arg @g' | paste -s -d " ") \
    -f docker/Dockerfile \
    --progress=plain \
    docker

    echo "Image built with tag 'devcontainer-${langage_name}'"
}

run() {
    local langage_name=$1
    local user=$2
    local user_id=$3

    docker run \
    -it \
    --rm \
    -e USER=${user} \
    -e USER_ID=${user_id} \
    devcontainer-${langage_name}
}


build common
build golang
build python
build rust
build ansible

