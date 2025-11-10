# Install golang
ARG REGISTRY=""
ARG COMMON_VERSION="latest"
FROM ${REGISTRY}common:${COMMON_VERSION} AS base

ARG GO_VERSION=1.23.4

ENV GO_VERSION=${GO_VERSION}
ENV TMPDIR=/build

RUN set-e ; \
    curl -L -o go.tar.gz https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz ; \
    tar -C /usr/local -xzf go.tar.gz ; \
    rm -rf go.tar.gz ; \
    mkdir -p ${TMPDIR} ; \
    chmod 2777 ${TMPDIR}

USER skell
