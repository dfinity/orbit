#!/usr/bin/env bash

set -eEuo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd "$SCRIPT_DIR/.."

# Pin to a specific commit to ensure compatibility with PocketIC server version.
# The asset canister Wasm must not import system APIs unsupported by the PocketIC version
# used in integration tests (see scripts/run-integration-tests.sh for the server version).
SDK_COMMIT=${1:-d65717bd6d0c172247c37dd23395c9fb13b2ba20}

echo "Downloading asset canister at commit $SDK_COMMIT"

mkdir -p wasms
cd wasms

HTTP_CODE=$(curl -L -so assetstorage.wasm.gz "https://github.com/dfinity/sdk/raw/${SDK_COMMIT}/src/distributed/assetstorage.wasm.gz" --write-out "%{http_code}")

if [[ ${HTTP_CODE} -ne 200 ]] ; then
    echo "Failed to download wasm. Response code: ${HTTP_CODE}"
    exit 1
fi

echo "Asset canister wasm downloaded"
