## TODO WORK WITH DEBOOTSTRAP

# FROM debian:stable AS base
# RUN apt-get update && apt-get install -y debootstrap
# RUN debootstrap --variant=minbase stable /mnt http://deb.debian.org/debian/

# FROM base AS install-python
# RUN chroot /mnt apt-get update && chroot /mnt apt-get install -y python3

# FROM install-python AS install-python-ssh
# RUN chroot /mnt apt-get install -y openssh-server

# FROM scratch AS image-python
# COPY --from=install-python /mnt/ /

# FROM scratch AS image-python-ssh
# COPY --from=install-python-ssh /mnt/ /

# Debian image as base (unstable for newest software).
FROM debian:trixie-slim AS common

# Set image locale.
ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US:en
ENV TZ=Europe/Paris

ENV USER=
ENV USER_ID=
ENV GOSU_VERSION=1.17

WORKDIR /root

USER 0

# Create root local configs directories
RUN \
    mkdir -p /root/.cache ; \
    mkdir -p /root/.config ; \
    mkdir -p /root/.local ;

# Install common packages
RUN \
    apt-get update ; \
    apt-get -y install \
        curl \
        xz-utils \
        shellcheck \
        shfmt \
        fzf \
        git \
        gpg \
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
RUN \
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
        docker-cli \
        docker-compose

# Clean install 
RUN \
    apt-get clean ; \
    rm -rf /var/lib/apt/lists/*

# Download fonts
RUN \
    curl -OL --output-dir /root https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.tar.xz

# Gosu installation for host user id adaptation 
RUN \
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
RUN \
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

RUN \
    apt-get update ; \
    apt-get install -y \
        openssh-server ; \
    ssh-keygen -A ; \
    sed -i 's/PermitRootLogin prohibit-password/PermitRootLogin no/' /etc/ssh/sshd_config ; \
    sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config ; \
    sed -i 's/#PermitEmptyPasswords no/PermitEmptyPasswords no/' /etc/ssh/sshd_config ; \
    apt-get clean ; \
    rm -rf /var/lib/apt/lists/*

# Install golang 
FROM common AS golang

ARG GO_VERSION=1.23.4

ENV GO_VERSION=${GO_VERSION}
ENV TMPDIR=/build

RUN curl -L -o go.tar.gz https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz ; \
    tar -C /usr/local -xzf go.tar.gz ; \
    rm -rf go.tar.gz ; \
    mkdir -p ${TMPDIR} ; \
    chmod 2777 ${TMPDIR}

# Install rust 
FROM common AS rust

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install python
FROM common AS python

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

FROM common AS ansible

ARG ANSIBLE_VERSION=11.4.0

ENV ANSIBLE_VERSION=${ANSIBLE_VERSION}

RUN \
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
RUN \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

