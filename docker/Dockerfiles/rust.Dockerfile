# Install rust
ARG REGISTRY=""
ARG COMMON_VERSION="latest"
FROM ${REGISTRY}common:${COMMON_VERSION} AS base

RUN set -e ; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

FROM ${REGISTRY}common-ssh:${COMMON_VERSION} AS base-ssh

RUN set -e ; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
