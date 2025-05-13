#!/usr/bin/env bash
set -eEuo pipefail

# Path to run `canbench` from.
CANISTER_PATH=$1
CANBENCH_RESULTS_FILE="results.yml"
CANBENCH_TMP_OUTPUT=".canbench_output"

# Function to print messages with a dynamic border
. "$(dirname "$0")/utils.sh"

if [ -z "$CANISTER_PATH" ]; then
  print_message "Error: CANISTER_PATH is not set. Exiting..."

  exit 1
fi

if [ ! -d "$CANISTER_PATH" ]; then
  print_message "Error: Canister path $CANISTER_PATH does not exist. Exiting..."
  exit 1
fi

print_message "Benchmarking canister at $CANISTER_PATH"

# Install canbench if not already installed
if ! cargo install --list | grep -q canbench; then
  print_message "Installing canbench..."
  cargo install canbench --version 0.1.11 --locked
fi

# Changes to the canister path
pushd "$CANISTER_PATH"

# Verify that canbench results are available
if [ ! -f "$CANBENCH_RESULTS_FILE" ]; then
  print_message "Error: $CANBENCH_RESULTS_FILE not found. Did you forget to run 'canbench --persist'?"
  exit 1
fi

# Detect if canbench results file is up to date
canbench --less-verbose >"$CANBENCH_TMP_OUTPUT"
if grep -q "(regress\|(improved by \|(new)" "$CANBENCH_TMP_OUTPUT"; then
  # Check if running in GitHub Actions and print the CANBENCH_TMP_OUTPUT file if so
  if [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    print_message "Review the benchmark results below:"
    cat "$CANBENCH_TMP_OUTPUT"
  fi

  print_message "Benchmarking completed. 

Results are not up to date ❌

Run 'canbench --persist' for expected changes."
  exit 1
fi

print_message "Benchmarking completed.

Results are up to date ✅"
