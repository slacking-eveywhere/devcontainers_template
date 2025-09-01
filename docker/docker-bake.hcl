
variable "REGISTRY" {
  default = "registry.site-lambda.fr:5000"
}

variable "COMMON_VERSION" {
    default = "latest"
}

group "default" {
    targets = ["golang", "python", "rust"]
}

group "golang" {
    targets = ["go-1-23-4"]
}

group "python" {
    targets = ["python-3-13"]
}

target "common" {
    context = "docker"
    dockerfile = "Dockerfiles/common.Dockerfile"
    targets = ["common", "common-ssh"]
    args = {
        REGISTRY="docker.io"
        DEBIAN_VERSION="trixie-slim"
        GOSU_VERSION="1.17"
    }
    tags = ["${REGISTRY}/common:trixie-slim", "${REGISTRY}/common-ssh:trixie-slim"]
}

target "go-1-23-4" {
    context = "docker"
    dockerfile = "Dockerfiles/golang.Dockerfile"
    targets = ["base", "base-ssh"]
    args = {
        GO_VERSION = "1.23.4",
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
    }
    tags = ["${REGISTRY}/devcontainer-golang:1.23.4", "${REGISTRY}/devcontainer-golang-ssh:1.23.4"]
}

target "python-3-13" {
    context = "docker"
    dockerfile = "Dockerfiles/python.Dockerfile"
    targets = ["base", "base-ssh"]
    args = {
        PYTHON_VERSION = "3.13",
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
    }
    tags = ["${REGISTRY}/devcontainer-python:3.13", "${REGISTRY}/devcontainer-python-ssh:3.13"]
}

target "rust" {
    context = "docker"
    dockerfile = "Dockerfiles/rust.Dockerfile"
    targets = ["base", "base-ssh"]
    args = {
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
    }
    tags = ["${REGISTRY}/devcontainer-rust:latest", "${REGISTRY}/devcontainer-rust-ssh:latest"]
}
