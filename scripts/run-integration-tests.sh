#!/usr/bin/env bash

set -eEuo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

TESTNAME=${1:-}
DOWNLOAD_NNS_CANISTERS="${DOWNLOAD_NNS_CANISTERS:-true}"
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
curl -sLO https://download.dfinity.systems/ic/bb2387f174b2509dd7b75a9ba2d6db5ea33ec0d6/binaries/x86_64-$PLATFORM/pocket-ic.gz || exit 1
gzip -df pocket-ic.gz
chmod +x pocket-ic
echo "PocketIC download completed"
cd ../..

if [ $DOWNLOAD_NNS_CANISTERS == "true" ]; then
    ./scripts/download-nns-canister-wasm.sh icp_ledger ledger-canister
    ./scripts/download-nns-canister-wasm.sh icp_index ic-icp-index-canister
fi

cargo test --package integration-tests $TESTNAME -- --test-threads $TEST_THREADS --nocapture
