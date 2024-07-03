#!/usr/bin/env bash
set -eEuo pipefail

#############################################
# INIT & TEARDOWN                           #
#############################################

GH_COMMIT=${GH_COMMIT:-}
GH_INITIAL_BRANCH=$(git rev-parse --abbrev-ref HEAD)

function init() {
  if [[ -z $GH_COMMIT ]]; then
    echo "INFO: GH_COMMIT is not set, building the latest commit"
  else
    echo "INFO: Building the commit $GH_COMMIT"
    git checkout $GH_COMMIT
  fi
}

function teardown() {
  if [[ -n $GH_COMMIT ]]; then
    echo "INFO: Restoring the initial branch $GH_INITIAL_BRANCH"
    git checkout $GH_INITIAL_BRANCH
  fi
}

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
  -h, --help prints this help message
EOF
}

function help() {
  cat <<EOF

Helper script to facilitate the building of the canisters in the Orbit project in a reproducible way.

Optionally the `GH_COMMIT` environment variable can be set to a specific commit hash to build the canisters 
at that specific commit.

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

function build_canister() {
  local canister_name=$1
  local target=$2

  # Build the canister
  docker build -t orbit-$canister_name --target $target .

  # Create a container to extract the generated artifacts
  docker create --name orbit-$canister_name-container orbit-$canister_name

  # Copy the generated artifacts to the host
  docker cp orbit-$canister_name-container:/code/artifacts/$canister_name ./artifacts

  # Remove the container
  docker rm orbit-$canister_name-container

  # Remove the image
  docker rmi orbit-$canister_name --force

  echo "The $canister_name canister artifacts have been copied to the host"
}

function build_control_panel() {
  build_canister control-panel build_control_panel
}

function build_station() {
  build_canister station build_station
}

function build_upgrader() {
  build_canister upgrader build_upgrader
}

#############################################
# SCRIPT OPTIONS                            #
#############################################

if [[ $# -eq 0 ]]; then
  title
  usage
  exit 0
fi

init

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
  *)
    echo "ERROR: unknown argument $1"
    usage
    echo
    echo "Use '$0 --help' for more information"
    exit 1
    ;;
  esac
done

teardown
