# Install python
ARG REGISTRY=""
FROM ${REGISTRY}common AS python

ARG PYTHON_VERSION=3.13

ENV PYTHON_VERSION=${PYTHON_VERSION}

RUN PY_MAJOR=$(echo $PYTHON_VERSION | cut -d. -f1) ; \
    apt-get update ; \
    apt-get install -y \
    python${PYTHON_VERSION}-dev \
    python${PYTHON_VERSION}-venv \
    python${PY_MAJOR}-pip

# Clean install
RUN \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ARG REGISTRY=""
FROM ${REGISTRY}common-ssh AS python-ssh
