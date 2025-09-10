# Install python
ARG REGISTRY=""
ARG COMMON_VERSION="latest"
FROM ${REGISTRY}common:${COMMON_VERSION} AS base

ARG PYTHON_VERSION=3.13

ENV PYTHON_VERSION=${PYTHON_VERSION}

RUN set -e ; \
    PY_MAJOR=$(echo $PYTHON_VERSION | cut -d. -f1) ; \
    apt-get update ; \
    apt-get install -y \
    python${PYTHON_VERSION}-dev \
    python${PYTHON_VERSION}-venv \
    python${PY_MAJOR}-pip

# Clean install
RUN set -e ; \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

FROM ${REGISTRY}common-ssh:${COMMON_VERSION} AS base-ssh
