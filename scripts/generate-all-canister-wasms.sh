#!/usr/bin/env bash

set -eEuo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR/..

./scripts/generate-wasm.sh control-panel
./scripts/generate-wasm.sh upgrader
./scripts/generate-wasm.sh station
./scripts/generate-wasm.sh test_canister
