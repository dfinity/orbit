#!/usr/bin/env bash
set -eEuo pipefail

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

#############################################
# USAGE                                     #
#############################################

function title() {
  echo "Orbit deployment"
}

function usage() {
  cat <<EOF

Usage:
  $0

Options:
  --local Performs a local deployment of Orbit to your local dfx environment
  --testing Performs a testing deployment of Orbit to the IC
  --staging Performs a staging deployment of Orbit to the IC
  --prod Performs a production deployment of Orbit to the IC
EOF
}

function help() {
  cat <<EOF

Helper script to setup Orbit deployment utils.

NOTE: This requires a working rust toolchain, dfx and nodejs to operate correctly.
EOF
}

function exec_function() {
  local function_name=$1
  echo "------------------------------------------------------"
  echo -e "\e[1m$ START:\e[0m $function_name"
  $function_name
  echo -e "\e[1m$ COMPLETED:\e[0m $function_name"
}

function setup_enviroment() {
  . ./scripts/setup-node.sh

  if ! command -v pnpm >/dev/null 2>&1; then
    echo "pnpm not found, installing..."
    npm install -g pnpm
  fi

  pnpm install

  setup_cycles_wallet
}

function get_network() {
  local network=${IC_NETWORK:-local}
  echo "$network"
}

function set_network() {
  local network=$1
  export IC_NETWORK=$network
}

#############################################
# UTILS                                     #
#############################################

function build_wasms() {
  echo "Building the WASMs for the wallet and upgrader canisters."

  ./scripts/generate-wasm.sh wallet
  ./scripts/generate-wasm.sh upgrader
}

function setup_cycles_wallet() {
  local network="$(get_network)"

  if [ "$network" != "local" ]; then
    set +e # Disable 'exit on error'
    cycles_wallet_id_output=$(dfx identity get-wallet --network $network 2>&1)
    cycles_wallet_id_exit_code=$?
    set -e # Re-enable 'exit on error'

    if [ $cycles_wallet_id_exit_code -ne 0 ]; then
      echo "Cycles wallet does not exist, using the default mainnet wallet for the deployment."
      cycles_wallet_id=$(dfx identity get-wallet --network ic)
      dfx identity set-wallet "$cycles_wallet_id" --network $network
    fi
  fi
}

function deploy_control_panel() {
  local network=$(get_network)

  echo "Deploying the control_panel canister to the '$network' network."

  exec_function "build_wasms"

  # Read the WASM files and convert them to hex format
  upgrader_wasm_module_bytes=$(hexdump -ve '1/1 "%.2x"' ./wasms/upgrader.wasm.gz | sed 's/../\\&/g')
  wallet_wasm_module_bytes=$(hexdump -ve '1/1 "%.2x"' ./wasms/wallet.wasm.gz | sed 's/../\\&/g')

  set +e # Disable 'exit on error'
  canister_id_output=$(dfx canister id control_panel --network $network 2>&1)
  canister_id_exit_code=$?
  set -e # Re-enable 'exit on error'

  if [ $canister_id_exit_code -ne 0 ]; then
    echo "Canister 'control_panel' does not exist, creating and installing..."

    dfx canister create control_panel --network $network --with-cycles 500000000000
    dfx build control_panel --network $network
    dfx canister install control_panel --network $network --argument-file <(echo "(opt variant { Init = record { upgrader_wasm_module = blob \"$upgrader_wasm_module_bytes\"; wallet_wasm_module = blob \"$wallet_wasm_module_bytes\"; } })")
  else
    echo "Canister 'control_panel' already exists with ID: $canister_id_output"

    dfx build control_panel --network $network
    dfx canister install control_panel --network $network --mode upgrade --argument-file <(echo "(opt variant { Upgrade = record { upgrader_wasm_module = opt blob \"$upgrader_wasm_module_bytes\"; wallet_wasm_module = opt blob \"$wallet_wasm_module_bytes\"; } })")
  fi
}

function deploy_ui() {
  local network=$(get_network)

  echo "Deploying the UI canister to the '$network' network."

  if [ "$network" == "local" ]; then
    NODE_ENV=development dfx deploy --network $network ui
    return
  fi

  NODE_ENV=production dfx deploy --network $network ui --with-cycles 500000000000
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
  --prod | --production)
    shift
    set_network prod
    exec_function setup_enviroment
    exec_function deploy_control_panel
    exec_function deploy_ui
    echo
    ;;
  --staging)
    shift
    set_network staging
    exec_function setup_enviroment
    exec_function deploy_control_panel
    exec_function deploy_ui
    echo
    ;;
  --testing)
    shift
    set_network testing
    exec_function setup_enviroment
    exec_function deploy_control_panel
    exec_function deploy_ui
    echo
    ;;
  --local)
    shift
    set_network local
    exec_function setup_enviroment
    exec_function deploy_control_panel
    exec_function deploy_ui
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
