# Operating system with basic tools
FROM --platform=linux/amd64 ubuntu:20.04 as base
SHELL ["bash", "-c"]
ENV TZ=UTC
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && \
    apt -yq update && \
    apt -yqq install --no-install-recommends curl ca-certificates \
        build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake \
        git jq npm xxd file curl unzip

# Code specific dependencies
FROM base as builder
WORKDIR /code
ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    FNM_DIR=/opt/fnm
# Path modifications need to be done in separate ENV statements
ENV PATH=$CARGO_HOME/bin:$PATH
ENV PATH=$FNM_DIR/bin:$PATH
# Install Rust and the Node.js version manager
COPY rust-toolchain.toml .
COPY .nvmrc .
RUN curl -fsSL https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path && \
    curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir $FNM_DIR/bin --skip-shell
# Add the fnm env var envaluation to the bashrc to enable it in bash by default
RUN echo "eval \"$(fnm env)\"" >> $HOME/.bashrc
# Install the correct version of Node.js and the dependencies
COPY . .
RUN eval "$(fnm env)" && \
    fnm install && \
    fnm use && \
    corepack enable && \
    pnpm install

# Build the Orbit Upgrader Canister
FROM builder as build_upgrader
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="upgrader"
RUN eval "$(fnm env)" && \
    fnm use && \
    BUILD_MODE=production npx nx run upgrader:create-artifacts

# Build the Orbit Station Canister
FROM builder as build_station
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="station"
RUN eval "$(fnm env)" && \
    fnm use && \
    BUILD_MODE=production npx nx run station:create-artifacts

# Build the Orbit Control Panel
FROM builder as build_control_panel
WORKDIR /code
LABEL io.icp.artifactType="canister" \
      io.icp.artifactName="control-panel"
RUN eval "$(fnm env)" && \
    fnm use && \
    BUILD_MODE=production npx nx run control-panel:create-artifacts
