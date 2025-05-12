#!/usr/bin/env bash

set -eEuo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

TESTNAME=${1:-}
DOWNLOAD_NNS_CANISTERS="${DOWNLOAD_NNS_CANISTERS:-true}"
DOWNLOAD_ASSET_CANISTER="${DOWNLOAD_ASSET_CANISTER:-true}"
BUILD_WASMS="${BUILD_WASMS:-true}"
TEST_THREADS="${TEST_THREADS:-2}"
OSTYPE="$(uname -s)" || OSTYPE="$OSTYPE"
OSTYPE="${OSTYPE,,}"
RUNNER_OS="${RUNNER_OS:-}"

if [[ "$OSTYPE" == "linux"* || "$RUNNER_OS" == "Linux" ]]; then
    PLATFORM=linux
elif [[ "$OSTYPE" == "darwin"* || "$RUNNER_OS" == "macOS" ]]; then
    PLATFORM=darwin
else
    echo "OS not supported: ${OSTYPE:-$RUNNER_OS}"
    exit 1
fi

if [ $BUILD_WASMS == "true" ]; then
    ./scripts/generate-all-canister-wasms.sh
fi

cd tests/integration
echo "PocketIC download starting"
curl -sLO https://github.com/dfinity/pocketic/releases/download/9.0.1/pocket-ic-x86_64-$PLATFORM.gz || exit 1
gzip -df pocket-ic-x86_64-$PLATFORM.gz
mv pocket-ic-x86_64-$PLATFORM pocket-ic
export POCKET_IC_BIN="$(pwd)/pocket-ic"
chmod +x pocket-ic
echo "PocketIC download completed"
cd ../..

if [ $DOWNLOAD_NNS_CANISTERS == "true" ]; then
    ./scripts/download-nns-canister-wasm.sh icp_ledger ledger-canister
    ./scripts/download-nns-canister-wasm.sh icp_index ic-icp-index-canister
    ./scripts/download-nns-canister-wasm.sh cmc cycles-minting-canister
    ./scripts/download-nns-canister-wasm.sh icrc1_ledger ic-icrc1-ledger
fi

if [ $DOWNLOAD_ASSET_CANISTER == "true" ]; then
    ./scripts/download-asset-canister-wasm.sh
fi

# ungzip station wasm to make it a large WASM for integration tests
gzip -d wasms/station.wasm.gz
mv wasms/station.wasm wasms/station.wasm.gz

cargo test --package integration-tests $TESTNAME -- --test-threads $TEST_THREADS --nocapture
