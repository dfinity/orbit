#!/usr/bin/env bash

set -eEuo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

BUILD_WASMS=${1:-true}
TEST_THREADS=${2:-2}
TESTNAME=${3:-}
OSTYPE=$(uname -s) || OSTYPE=$OSTYPE
OSTYPE="${OSTYPE,,}"
RUNNER_OS="${RUNNER_OS:-}"


if [[ "$OSTYPE" == "linux"* || "$RUNNER_OS" == "Linux" ]]
then
    PLATFORM=linux
elif [[ "$OSTYPE" == "darwin"* || "$RUNNER_OS" == "macOS" ]]
then
    PLATFORM=darwin
else
    echo "OS not supported: ${OSTYPE:-$RUNNER_OS}"
    exit 1
fi

if [ $BUILD_WASMS == "true" ]
then
    ./scripts/generate-all-canister-wasms.sh
fi

cd canisters/integration_tests
echo "PocketIC download starting"
curl -sO https://download.dfinity.systems/ic/29ec86dc9f9ca4691d4d4386c8b2aa41e14d9d16/openssl-static-binaries/x86_64-$PLATFORM/pocket-ic.gz || exit 1
gzip -df pocket-ic.gz
chmod +x pocket-ic
echo "PocketIC download completed"
cd ../..

./scripts/download-nns-canister-wasm.sh icp_ledger ledger-canister
./scripts/download-nns-canister-wasm.sh icp_index ic-icp-index-canister

cargo test --package integration_tests $TESTNAME -- --test-threads $TEST_THREADS
