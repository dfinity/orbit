#!/usr/bin/env bash

set -eEuo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

echo "Downloading asset canister"

mkdir -p wasms
cd wasms

HTTP_CODE=$(curl -L -so assetstorage.wasm.gz https://github.com/dfinity/sdk/raw/master/src/distributed/assetstorage.wasm.gz --write-out "%{http_code}")

if [[ ${HTTP_CODE} -ne 200 ]] ; then
    echo "Failed to download wasm. Response code: ${HTTP_CODE}"
    exit 1
fi

echo "Asset canister wasm downloaded"

HTTP_CODE=$(curl -L -so assetstorage.did https://github.com/dfinity/sdk/raw/master/src/distributed/assetstorage.did --write-out "%{http_code}")

if [[ ${HTTP_CODE} -ne 200 ]] ; then
    echo "Failed to download did file. Response code: ${HTTP_CODE}"
    exit 1
fi

echo "Asset canister did file downloaded"
