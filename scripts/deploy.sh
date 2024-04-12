#!/usr/bin/env bash
set -eEuo pipefail

#############################################
# USAGE                                     #
#############################################

function title() {
  echo "Orbit deployment"
}

function identity_warning_confirmation() {
  if [ "${SKIP_CONFIRMATION:-}" = "true" ] || [ "${SKIP_CONFIRMATION:-}" = "1" ]; then
    return
  fi

  echo -e "\e[1;33m"
  echo -e "WARNING: You are about to deploy to the IC network, this will use your default identity."
  echo -e "\e[0m"

  identity=$(dfx identity whoami)

  if [ -z "$identity" ]; then
    echo "No identity found, please login to your dfx environment."
    exit 1
  fi

  echo "Current identity: $identity"
  echo

  read -p "Do you want to continue? [y/N]: " confirmation

  if [[ ! "$confirmation" =~ ^[yY] ]]; then
    echo
    echo "Deployment cancelled."
    exit 1
  fi
}

function usage() {
  cat <<EOF

Usage:
  $0


Options:
  --play | --playground Deploys Orbit to the playground network (WARNING: This will reset the control-panel, to avoid this use 'no-reset' as an argument)
  --testing Performs a testing deployment of Orbit to the IC
  --staging Performs a staging deployment of Orbit to the IC
  --prod | --production Performs a production deployment of Orbit to the IC
EOF
}

function help() {
  cat <<EOF

Helper script to setup Orbit deployment.

NOTE: This requires a working rust toolchain, dfx and nodejs to operate correctly.
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

function get_subnet_type() {
  local network=$(get_network)

  if [ "$network" == "prod" ]; then
    echo "fiduciary"
  else
    echo ""
  fi
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

function reset_playground_network() {
  local network="$(get_network)"

  if [ "$network" != "playground" ]; then
    echo "ERROR: This operation is only supported on the playground network"
    exit 1
  fi

  echo "Resetting the playground network..."
  echo "This will remove the code and data for the control_panel canister."

  set +e # Disable 'exit on error'
  canister_id_output=$(dfx canister id control_panel --network $network 2>&1)
  canister_id_exit_code=$?
  set -e # Re-enable 'exit on error'

  if [ $canister_id_exit_code -eq 0 ]; then
    echo "Canister 'control_panel' exists with ID: $canister_id_output"
    echo "Uninstalling code from the control_panel..."

    dfx canister --network $network uninstall-code control_panel
  else
    echo "Canister 'control_panel' does not exist."
  fi
}

function deploy_control_panel() {
  local network="$(get_network)"
  local subnet_type=$(get_subnet_type)

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

    dfx canister create control_panel --network $network --with-cycles 5000000000000 $([[ -n "$subnet_type" ]] && echo "--subnet-type $subnet_type")
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
  local subnet_type=$(get_subnet_type)

  echo "Deploying the UI canister to the '$network' network."

  BUILD_MODE=$network dfx deploy --network $network ui --with-cycles 2000000000000 $([[ -n "$subnet_type" ]] && echo "--subnet-type $subnet_type")
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
    identity_warning_confirmation
    set_network prod
    exec_function setup_enviroment
    exec_function deploy_control_panel
    exec_function deploy_ui
    echo
    ;;
  --staging)
    shift
    identity_warning_confirmation
    set_network staging
    exec_function setup_enviroment
    exec_function deploy_control_panel
    exec_function deploy_ui
    echo
    ;;
  --play | --playground)
    shift
    identity_warning_confirmation
    set_network playground
    exec_function setup_enviroment
    if [ "${1-}" == "no-reset" ]; then
      shift
    else
      exec_function reset_playground_network
    fi
    exec_function deploy_control_panel
    exec_function deploy_ui
    echo
    ;;
  --testing)
    shift
    identity_warning_confirmation
    set_network testing
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
