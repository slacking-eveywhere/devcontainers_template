FROM common AS base

ARG ANSIBLE_VERSION=11.4.0

ENV ANSIBLE_VERSION=${ANSIBLE_VERSION}

RUN set -e ; \
    apt-get update ; \
    apt-get install -y \
    python3-pip \
    pipx ; \
    pipx ensurepath ; \
    pipx ensurepath --global ; \
    pipx install --global --include-deps ansible==${ANSIBLE_VERSION} ; \
    pipx inject --global --include-apps ansible ansible-dev-tools ; \
    pipx inject --global --include-apps ansible ansible-lint

# Clean install
RUN set -e ; \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

USER skell
