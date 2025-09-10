# Debian image as base (unstable for newest software).

ARG REGISTRY=""
ARG DEBIAN_VERSION="trixie-slim"
ARG GOSU_VERSION="1.17"
FROM debian:${DEBIAN_VERSION} AS common

ARG GOSU_VERSION

# Set image locale.
ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US:en
ENV TZ=Europe/Paris

ENV USER=
ENV USER_ID=
ENV GOSU_VERSION=${GOSU_VERSION}

WORKDIR /root

USER 0

# Create root local configs directories
RUN set -e ; \
    mkdir -p /root/.cache ; \
    mkdir -p /root/.config ; \
    mkdir -p /root/.local ; \
    touch /root/.ready

# Install common packages
RUN set -e ; \
    apt-get update ; \
    apt-get -y install \
    curl \
    fontconfig \
    xz-utils \
    shellcheck \
    shfmt \
    fzf \
    git \
    gpg \
    make \
    locales \
    stow \
    tree \
    tzdata \
    unzip \
    vim \
    wget \
    zip \
    zsh ;

# Install docker for dod bevause it's very neat to dod
RUN set -e ; \
    # Add Docker's official GPG key:
    curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc ; \
    chmod a+r /etc/apt/keyrings/docker.asc ; \
    # Add the repository to Apt sources:
    echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/debian \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    tee /etc/apt/sources.list.d/docker.list > /dev/null ; \
    apt-get update ; \
    apt-get install -y --no-install-recommends \
    docker-ce-cli \
    containerd.io \
    docker-buildx-plugin \
    docker-compose-plugin

# Clean install
RUN set -e ; \
    apt-get clean ; \
    rm -rf /var/lib/apt/lists/*

# Download fonts
RUN set -e ; \
    curl -OL --output-dir /root https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.tar.xz

# Gosu installation for host user id adaptation
RUN set -e ; \
    dpkgArch="$(dpkg --print-architecture | awk -F- '{ print $NF }')"; \
    wget -O /usr/local/bin/gosu "https://github.com/tianon/gosu/releases/download/${GOSU_VERSION}/gosu-$dpkgArch"; \
    wget -O /usr/local/bin/gosu.asc "https://github.com/tianon/gosu/releases/download/${GOSU_VERSION}/gosu-$dpkgArch.asc"; \
    \
    # verify the signature
    export GNUPGHOME="$(mktemp -d)"; \
    gpg --batch --keyserver hkps://keys.openpgp.org --recv-keys B42F6819007F00F88E364FD4036A9C25BF357DD4; \
    gpg --batch --verify /usr/local/bin/gosu.asc /usr/local/bin/gosu; \
    gpgconf --kill all; \
    rm -rf "$GNUPGHOME" /usr/local/bin/gosu.asc; \
    \
    chmod +x /usr/local/bin/gosu; \
    # verify that the binary works
    gosu --version; \
    gosu nobody true

# Copy custom locales because some shits can happens
COPY locale.conf /etc/default/locale.conf

# Copy docker entrypoint to the root
COPY docker-entrypoint.sh /docker-entrypoint.sh

# Add docker entrypoint and set locale
RUN set -e ; \
    chmod +x /docker-entrypoint.sh ; \
    mkdir /workdir ; \
    dpkg-reconfigure locales ; \
    sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen ; \
    locale-gen ;

# Set entrypoint with gosu command
ENTRYPOINT [ "/docker-entrypoint.sh" ]

# Expose some ports to host by default.
EXPOSE 8080 8081 8082 8083 8084 8085

# Install with SSH server
FROM common AS common-ssh

RUN set -e ; \
    apt-get update ; \
    apt-get install -y \
    openssh-server ; \
    ssh-keygen -A ; \
    sed -i 's/PermitRootLogin prohibit-password/PermitRootLogin no/' /etc/ssh/sshd_config ; \
    sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config ; \
    sed -i 's/#PermitEmptyPasswords no/PermitEmptyPasswords no/' /etc/ssh/sshd_config ; \
    apt-get clean ; \
    rm -rf /var/lib/apt/lists/*
