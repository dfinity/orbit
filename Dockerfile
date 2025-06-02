# Define the BUILD_MODE argument with a default value of "production"
ARG BUILD_MODE=production
ARG TARGETPLATFORM=linux/amd64

# Operating system with basic tools
FROM ubuntu:24.04 AS base
SHELL ["bash", "-c"]
ENV TZ=UTC
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && \
    apt -yq -o Acquire::http::Pipeline-Depth=0 -o Acquire::Retries=3 update && \
    apt -yqq -o Acquire::http::Pipeline-Depth=0 -o Acquire::Retries=3 install --no-install-recommends curl ca-certificates \
        build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake \
        git jq npm xxd file curl unzip

# Code specific dependencies
FROM base AS builder
SHELL ["bash", "-c"]
WORKDIR /code
ARG BUILD_MODE
ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    FNM_DIR=/opt/fnm \
    BUILD_MODE=${BUILD_MODE}
# Path modifications need to be done in separate ENV statements
ENV PATH=$CARGO_HOME/bin:$PATH
ENV PATH=$FNM_DIR/bin:$PATH
# Install Rust and the Node.js version manager
COPY rust-toolchain.toml .
RUN curl -O https://static.rust-lang.org/rustup/archive/1.27.1/x86_64-unknown-linux-gnu/rustup-init && chmod +x rustup-init && ./rustup-init -y
RUN curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir $FNM_DIR/bin --skip-shell
# Add the fnm env var envaluation to the bashrc to enable it in bash by default
RUN echo "eval \"$(fnm env)\"" >> $HOME/.bashrc
# Add expected node version file and root package.json with expected pnpm version
COPY .nvmrc .
COPY package.json .
# Install the node version specified in the version file
RUN eval "$(fnm env)" && \
    fnm install && \
    fnm use && \
    npm install -g corepack@0.31.0 && \
    corepack enable && \
    fnm alias default production
# Install the monorepo dependencies
COPY . .
RUN eval "$(fnm env)" && \
    fnm use && \
    pnpm install --frozen-lockfile

# Build the Orbit Upgrader Canister
FROM builder AS build_upgrader
SHELL ["bash", "-c"]
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="upgrader"
RUN eval "$(fnm env)" && \
    fnm use && \
    npx nx run upgrader:create-artifacts

# Build the Orbit Station Canister
FROM builder AS build_station
SHELL ["bash", "-c"]
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="station"
RUN eval "$(fnm env)" && \
    fnm use && \
    npx nx run station:create-artifacts

# Build the Orbit Control Panel
FROM builder AS build_control_panel
SHELL ["bash", "-c"]
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="control-panel"
RUN eval "$(fnm env)" && \
    fnm use && \
    npx nx run control-panel:create-artifacts

# Build the Orbit Wallet Frontend Assets
FROM builder AS build_wallet_dapp
SHELL ["bash", "-c"]
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="wallet-dapp"
RUN eval "$(fnm env)" && \
    fnm use && \
    npx nx run wallet-dapp:create-artifacts

# Build the Orbit Wallet Frontend Assets
FROM builder AS build_marketing_dapp
SHELL ["bash", "-c"]
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="marketing-dapp"
RUN eval "$(fnm env)" && \
    fnm use && \
    npx nx run marketing-dapp:create-artifacts

# Build the Orbit Docs Frontend Assets
FROM builder AS build_docs_portal
SHELL ["bash", "-c"]
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="docs-portal"
RUN eval "$(fnm env)" && \
    fnm use && \
    npx nx show projects && \
    npx nx run docs-portal:create-artifacts
