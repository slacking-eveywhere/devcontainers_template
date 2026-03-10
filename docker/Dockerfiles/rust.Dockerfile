# Install rust
ARG REGISTRY=""
ARG COMMON_VERSION="latest"
FROM ${REGISTRY}common:${COMMON_VERSION} AS base

RUN set -e ; \
    apt-get update ; \
    apt-get install -y \
        gcc \
        build-essential ; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN set -e ; \
    apt-get clean ; \
    rm -rf /var/lib/apt/lists/*

USER skell
