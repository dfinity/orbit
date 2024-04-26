#!/usr/bin/env bash

set -eEuo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

CANISTER_NAME=$1
FEATURES="${FEATURES:-}"
if [ -n "$FEATURES" ]; then
  FEATURES="--features $FEATURES"
fi
PACKAGE="${CANISTER_NAME}"
OSTYPE=$(uname -s) || OSTYPE=$OSTYPE
OSTYPE="${OSTYPE,,}"
RUNNER_OS="${RUNNER_OS:-}"

if [ -z "${CARGO_HOME:-}" ]; then
  export CARGO_HOME="${HOME}/.cargo"
fi

if [ -z "${GIT_COMMIT_ID:-}" ]; then
  export GIT_COMMIT_ID=$(git rev-parse HEAD)
fi

echo Building package $PACKAGE
export RUSTFLAGS="--remap-path-prefix $(readlink -f ${SCRIPT_DIR}/..)=/build --remap-path-prefix ${CARGO_HOME}/bin=/cargo/bin --remap-path-prefix ${CARGO_HOME}/git=/cargo/git"
for l in $(ls ${CARGO_HOME}/registry/src/); do
  export RUSTFLAGS="--remap-path-prefix ${CARGO_HOME}/registry/src/${l}=/cargo/registry/src/github ${RUSTFLAGS}"
done

candid_spec_file=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[] | select(.name == "'$PACKAGE'") | .manifest_path | gsub("/impl/Cargo.toml$"; "") + "/api/spec.did"')
package_version=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[] | select(.name == "'$PACKAGE'") | .version')

cargo build --locked --target wasm32-unknown-unknown --release --package $PACKAGE $FEATURES

if [[ "$OSTYPE" == "linux"* || "$RUNNER_OS" == "Linux" ]]; then
  URL="https://github.com/dfinity/ic-wasm/releases/download/0.6.0/ic-wasm-linux64"
elif [[ "$OSTYPE" == "darwin"* || "$RUNNER_OS" == "macOS" ]]; then
  URL="https://github.com/dfinity/ic-wasm/releases/download/0.6.0/ic-wasm-macos"
else
  echo "OS not supported: ${OSTYPE:-$RUNNER_OS}"
  exit 1
fi
curl -sL "${URL}" -o ic-wasm || exit 1
chmod +x ic-wasm

PACKAGE=$(echo $PACKAGE | tr - _)

# if candid file exists, generate metadata
if [ -f "$candid_spec_file" ]; then
  echo Adding wasm metadata: \"candid:service\"
  ./ic-wasm ./target/wasm32-unknown-unknown/release/$PACKAGE.wasm -o ./target/wasm32-unknown-unknown/release/$PACKAGE.wasm metadata candid:service -f $candid_spec_file -v public
fi

if [ -n "$package_version" ]; then
  echo Adding wasm metadata: \"app:version\"
  ./ic-wasm ./target/wasm32-unknown-unknown/release/$PACKAGE.wasm -o ./target/wasm32-unknown-unknown/release/$PACKAGE.wasm metadata app:version -d "$package_version" -v public
fi

echo Optimising wasm
./ic-wasm ./target/wasm32-unknown-unknown/release/$PACKAGE.wasm -o ./target/wasm32-unknown-unknown/release/$PACKAGE-opt.wasm shrink
./ic-wasm ./target/wasm32-unknown-unknown/release/$PACKAGE-opt.wasm -o ./target/wasm32-unknown-unknown/release/$PACKAGE-opt.wasm optimize O3

echo Compressing wasm
mkdir -p wasms
gzip -fckn9 target/wasm32-unknown-unknown/release/$PACKAGE-opt.wasm >./wasms/$CANISTER_NAME.wasm.gz
