#!/usr/bin/env bash
set -eEuo pipefail

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
  --all builds all the canisters
  --control-panel builds the control panel canister
  --station builds the station canister
  --upgrader builds the upgrader canister
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

function build_all() {
  docker build -t orbit-all --target build_all .
}

function build_control_panel() {
  docker build -t orbit-control-panel --target build_control_panel .
}

function build_station() {
  docker build -t orbit-station --target build_station .
}

function build_upgrader() {
  docker build -t orbit-upgrader --target build_upgrader .
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
  --all)
    shift
    exec_function build_all
    echo
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
