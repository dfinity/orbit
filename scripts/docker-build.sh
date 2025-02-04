#!/usr/bin/env bash
set -eEuo pipefail

# Set the build mode to production by default
BUILD_MODE=${BUILD_MODE:-"production"}

#############################################
# USAGE                                     #
#############################################

function title() {
  echo "Orbit reproducible builds"
}

function usage() {
  cat <<EOF

Usage:
  $0

Options:
  --control-panel builds the control panel canister
  --station builds the station canister
  --upgrader builds the upgrader canister
  --wallet-dapp builds the wallet frontend assets
  --docs-portal builds the docs frontend assets

  -h, --help prints this help message
EOF
}

function help() {
  cat <<EOF

Helper script to facilitate the building of the canisters in the Orbit project in a reproducible way.

NOTE: This requires a working Docker installation.
EOF
}

function exec_function() {
  local function_name=$1
  shift
  echo "------------------------------------------------------"
  echo -e "\e[1m$ START:\e[0m $function_name"
  $function_name "$@"
  echo -e "\e[1m$ COMPLETED:\e[0m $function_name"
}

#############################################
# FEATURES                                  #
#############################################

function deterministic_build() {
  local project_name=$1
  local target=$2

  # Build the canister
  docker build --build-arg BUILD_MODE=$BUILD_MODE -t orbit-$project_name --target $target .
   

  # Create a container to extract the generated artifacts
  docker create --name orbit-$project_name-container orbit-$project_name

  # Ensure the artifacts directory exists
  mkdir -p ./artifacts/$project_name

  # Copy the generated artifacts to the host
  docker cp orbit-$project_name-container:/code/artifacts/$project_name/. ./artifacts/$project_name/

  # Remove the container
  docker rm orbit-$project_name-container

  # Remove the image
  docker rmi orbit-$project_name --force

  echo "The $project_name project artifacts have been copied to the host"
}

function build_control_panel() {
  deterministic_build control-panel build_control_panel
}

function build_station() {
  deterministic_build station build_station
}

function build_upgrader() {
  deterministic_build upgrader build_upgrader
}

function build_wallet_dapp() {
  deterministic_build wallet-dapp build_wallet_dapp
}

function build_docs_portal() {
  deterministic_build docs-portal build_docs_portal
}

#############################################
# SCRIPT OPTIONS                            #
#############################################

if [[ $# -eq 0 ]]; then
  title
  usage
  exit 0
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
  -h | --help)
    title
    usage
    help
    exit 0
    ;;
  --control-panel)
    shift
    exec_function build_control_panel
    echo
    ;;
  --station)
    shift
    exec_function build_station
    echo
    ;;
  --upgrader)
    shift
    exec_function build_upgrader
    echo
    ;;
  --wallet-dapp)
    shift
    exec_function build_wallet_dapp
    echo
    ;;
  --docs-portal)
    shift
    exec_function build_docs_portal
    echo
    ;;
  *)
    echo "ERROR: unknown argument $1"
    usage
    echo
    echo "Use '$0 --help' for more information"
    exit 1
    ;;
  esac
done
