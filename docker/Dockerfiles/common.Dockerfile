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

ENV YQ_VERSION=v4.47.2
ENV YQ_BINARY=yq_linux_amd64

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
    touch /tmp/.ready

# Install common packages
RUN set -e ; \
    apt-get update ; \
    apt-get -y install \
    age \
    curl \
    fontconfig \
    jq \
    lazygit \
    locales \
    openssh-server \
    fzf \
    git \
    gpg \
    make \
    shellcheck \
    shfmt \
    stow \
    sudo \
    tree \
    tzdata \
    unzip \
    vim \
    wget \
    xz-utils \
    zip \
    zsh ;

RUN set -e ; \
    wget https://github.com/mikefarah/yq/releases/download/${YQ_VERSION}/${YQ_BINARY} -O /usr/local/bin/yq && chmod +x /usr/local/bin/yq

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

# Configure ssh server
RUN set -e  ; \
    sed -i 's/PermitRootLogin prohibit-password/PermitRootLogin no/' /etc/ssh/sshd_config ; \
    sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config ; \
    sed -i 's/#PermitEmptyPasswords no/PermitEmptyPasswords no/' /etc/ssh/sshd_config ; \
    ssh-keygen -A

# Allow sudo without password for all users (it's a devcontainer after all, so don't care much about security here)
RUN set -e ; \
    useradd \
		--shell /usr/bin/zsh \
		--home-dir "/var/local/skell" \
		--uid 1000500 \
		"skell" ; \
    mkdir -p /var/local/skell ; \
	chown skell:skell /var/local/skell ; \
    echo "skell ALL=(ALL:ALL) NOPASSWD: ALL" | tee "/etc/sudoers.d/users" > /dev/null ; \
    chmod 440 /etc/sudoers.d/ 

# # Download fonts
# RUN set -e ; \
#     curl -OL --output-dir /var/local/skell https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.tar.xz

# entrypoint minimaliste qui adapte l'uid/gid
# docker exec pour post install tous les elements en plus pour que l'utilisateur ait son beau shell
# USER  et USER_ID deviennent optionnels dans l'entrypoint. ces droits sont à prendre en compte que dans un devcontainer sur vscode.

# Copy custom locales because some shits can happens
COPY locale.conf /etc/default/locale.conf

# Copy docker entrypoint to the root
COPY docker-entrypoint.sh /docker-entrypoint.sh
COPY post-install.sh /usr/local/bin/post-install.sh

# Add docker entrypoint and set locale
RUN set -e ; \
    mkdir /workdir ; \
    dpkg-reconfigure locales ; \
    sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen ; \
    locale-gen ;

# Set entrypoint with gosu command
ENTRYPOINT [ "/docker-entrypoint.sh" ]

# Expose some ports to host by default.
EXPOSE 8080 8081 8082 8083 8084 8085
