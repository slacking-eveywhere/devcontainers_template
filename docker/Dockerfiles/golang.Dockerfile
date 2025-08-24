# Install golang
ARG REGISTRY=""
FROM ${REGISTRY}common AS golang

ARG GO_VERSION=1.23.4

ENV GO_VERSION=${GO_VERSION}
ENV TMPDIR=/build

RUN curl -L -o go.tar.gz https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz ; \
    tar -C /usr/local -xzf go.tar.gz ; \
    rm -rf go.tar.gz ; \
    mkdir -p ${TMPDIR} ; \
    chmod 2777 ${TMPDIR}

FROM ${REGISTRY}common-ssh AS golang-ssh

COPY --from=golang /usr/local /usr/local
