# Operating system with basic tools
FROM --platform=linux/amd64 ubuntu@sha256:bbf3d1baa208b7649d1d0264ef7d522e1dc0deeeaaf6085bf8e4618867f03494 as base
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
RUN curl -fsSL https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path && \
    curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir $FNM_DIR/bin --skip-shell
# Add the fnm env var envaluation to the bashrc to enable it in bash by default
RUN echo "eval \"$(fnm env)\"" >> $HOME/.bashrc
# Add expected node version file and root package.json with expected pnpm version
COPY .nvmrc .
COPY package.json .
# Install the node version specified in the version file
RUN eval "$(fnm env)" && \
    fnm install && \
    fnm use && \
    corepack enable && \
    fnm alias default production
# Install the monorepo dependencies
COPY . .
RUN eval "$(fnm env)" && \
    fnm use && \ 
    pnpm install --frozen-lockfile

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
