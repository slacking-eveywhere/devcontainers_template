variable "REGISTRY" {
  default = ""
}

variable "COMMON_VERSION" {
    default = "trixie-slim"
}

group "default" {
    targets = ["golang", "python", "rust", "bash"]
}

group "golang" {
    targets = ["go-1-25-4"]
}

group "python" {
    targets = ["python-3-13", "python-3-14"]
}

group "bash" {
    targets = ["bash"]
}

group "rust" {
    targets = ["rust"]
}

target "common" {
    context = "docker"
    dockerfile = "Dockerfiles/common.Dockerfile"
    targets = ["common"]
    args = {
        REGISTRY = REGISTRY
        DEBIAN_VERSION = "trixie-slim"
        GOSU_VERSION = "1.17"
    }
    tags = ["${REGISTRY}common:trixie-slim"]
}

target "bash" {
    context = "docker"
    dockerfile = "Dockerfiles/bash.Dockerfile"
    targets = ["base"]
    args = {
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
    }
    tags = ["${REGISTRY}devcontainer-bash:latest"]
}

target "go-1-25-4" {
    context = "docker"
    dockerfile = "Dockerfiles/golang.Dockerfile"
    targets = ["base"]
    args = {
        GO_VERSION = "1.25.4",
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
    }
    tags = ["${REGISTRY}devcontainer-golang:1.25.4"]
}

target "python-3-13" {
    context = "docker"
    dockerfile = "Dockerfiles/python.Dockerfile"
    targets = ["base"]
    args = {
        PYTHON_VERSION = "3.13",
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
    }
    tags = ["${REGISTRY}devcontainer-python:3.13"]
}

target "python-3-14" {
    context = "docker"
    dockerfile = "Dockerfiles/python.Dockerfile"
    targets = ["base"]
    args = {
        PYTHON_VERSION = "3.14",
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
    }
    tags = ["${REGISTRY}devcontainer-python:3.14"]
}

target "rust" {
    context = "docker"
    dockerfile = "Dockerfiles/rust.Dockerfile"
    targets = ["base"]
    args = {
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
    }
    tags = ["${REGISTRY}devcontainer-rust:latest"]
}

target "ansible" {
    context = "docker"
    dockerfile = "Dockerfiles/ansible.Dockerfile"
    targets = ["base"]
    args = {
        REGISTRY = REGISTRY,
        COMMON_VERSION = COMMON_VERSION
        ANSIBLE_VERSION="13.5.0"
    }
    tags = ["${REGISTRY}devcontainer-ansible:13.5.0"]
}