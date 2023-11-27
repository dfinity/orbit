#!/usr/bin/env bash

set -eEuo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

CANISTER_NAME=$1
PACKAGE="${CANISTER_NAME}"
OSTYPE=$(uname -s) || OSTYPE=$OSTYPE
OSTYPE="${OSTYPE,,}"
RUNNER_OS="${RUNNER_OS:-}"

if [ -z "${CARGO_HOME:-}" ]
then
  export CARGO_HOME="${HOME}/.cargo"
fi

if [ -z "${GIT_COMMIT_ID:-}" ]
then
  export GIT_COMMIT_ID=$(git rev-parse HEAD)
fi

echo Building package $PACKAGE
export RUSTFLAGS="--remap-path-prefix $(readlink -f ${SCRIPT_DIR}/..)=/build --remap-path-prefix ${CARGO_HOME}/bin=/cargo/bin --remap-path-prefix ${CARGO_HOME}/git=/cargo/git"
for l in $(ls ${CARGO_HOME}/registry/src/)
do
  export RUSTFLAGS="--remap-path-prefix ${CARGO_HOME}/registry/src/${l}=/cargo/registry/src/github ${RUSTFLAGS}"
done
cargo build --locked --target wasm32-unknown-unknown --release --package $PACKAGE

echo Optimising wasm
if [[ "$OSTYPE" == "linux"* || "$RUNNER_OS" == "Linux" ]]
then
    URL="https://github.com/dfinity/ic-wasm/releases/download/0.6.0/ic-wasm-linux64"
elif [[ "$OSTYPE" == "darwin"* || "$RUNNER_OS" == "macOS" ]]
then
    URL="https://github.com/dfinity/ic-wasm/releases/download/0.6.0/ic-wasm-macos"
else
    echo "OS not supported: ${OSTYPE:-$RUNNER_OS}"
    exit 1
fi
curl -sL "${URL}" -o ic-wasm || exit 1
chmod +x ic-wasm
./ic-wasm ./target/wasm32-unknown-unknown/release/$PACKAGE.wasm -o ./target/wasm32-unknown-unknown/release/$PACKAGE-opt.wasm shrink

echo Compressing wasm
mkdir -p wasms
gzip -fckn target/wasm32-unknown-unknown/release/$PACKAGE-opt.wasm > ./wasms/$CANISTER_NAME.wasm.gz
