#!/usr/bin/env bash
set -eEuo pipefail

# Whether or not to reuse the artifacts that are already built
REUSE_ARTIFACTS=${REUSE_ARTIFACTS:-"false"}

# Default identity store path
DFX_DEFAULT_IDENTITY_STORE_PATH=${DFX_DEFAULT_IDENTITY_STORE_PATH:-"$HOME/.config/dfx/identity"}

# Identity PEM path to use for the deployment of the asset canister files
IDENTITY_PEM_PATH=${IDENTITY_PEM_PATH:-""}

# Path to the dfx.json file
DFX_JSON_PATH=${DFX_JSON_PATH:-"dfx.json"}

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

  local network=$(get_network)

  echo -e "\e[1;33m"
  echo "WARNING: You are about to deploy to the IC with the \"$network\" network, this will use your active identity."
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

  set_identity_pem_path "$identity"
}

function usage() {
  cat <<EOF

Usage:
  $0


Options:
  --local Deploys Orbit to the local network (If 'reset' is specified, the control-panel will be reset)
  --playground Deploys Orbit to the playground network (If 'reset' is specified, the control-panel will be reset)
  --staging Performs a staging deployment of Orbit to the IC
  --production Performs a production deployment of Orbit to the IC
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

  if ! command -v icx-asset >/dev/null 2>&1; then
    echo "icx-asset not found, installing..."

    cargo install icx-asset --version 0.21.0

    echo "icx-asset installed successfully."

  fi
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

function get_replica_url() {
  local network=$(get_network)
  local result

  # Extract the first provider or bind using jq
  result=$(jq -r --arg network "$network" \
    '.networks[$network] | 
    if .providers then .providers[0] 
    elif .bind then "http://" + .bind 
    else null 
    end' "$DFX_JSON_PATH")

  if [ -z "$result" ] || [ "$result" == "null" ]; then
    echo -e "\e[1;31m"
    echo "ERROR: Replica URL not found for the network: $network"
    echo "Please make sure the dfx.json file is correctly configured."
    echo -e "\e[0m"

    exit 1
  else
    echo "$result"
  fi
}

function set_identity_pem_path() {
  local identity_name=$1
  local identity_store_path=${DFX_DEFAULT_IDENTITY_STORE_PATH}

  if [ -z "$IDENTITY_PEM_PATH" ]; then
    local identity_pem_path="${identity_store_path}/${identity_name}/${identity_name}.pem"

    # Check if the identity pem file exists, else fallback to the id.pem filename
    if [ ! -f "$identity_pem_path" ]; then
      identity_pem_path="${identity_store_path}/${identity_name}/identity.pem"
    fi

    if [ ! -f "$identity_pem_path" ]; then
      identity_pem_path="${identity_store_path}/${identity_name}/id.pem"
    fi

    export IDENTITY_PEM_PATH=$identity_pem_path
  fi

  if [ ! -f "$IDENTITY_PEM_PATH" ]; then
    echo -e "\e[1;31m"
    echo "ERROR: Identity PEM file not found for the identity: $identity_name"
    echo "Please make sure the identity is available in the default identity store path: $identity_store_path"
    echo -e "\e[0m"

    exit 1
  fi
}

#############################################
# UTILS                                     #
#############################################

function should_build_artifacts() {
  if [ "${REUSE_ARTIFACTS,,}" == "true" ] || [ "${REUSE_ARTIFACTS}" == "1" ]; then
    return 1 # 1 means "false" in shell scripting
  fi

  return 0 # 0 means "true" in shell scripting
}

function build_wasms() {
  echo "Preparing the WASMs for the station and upgrader canisters."

  local network=$(get_network)

  if should_build_artifacts || [ ! -f ./artifacts/station/station.wasm.gz ]; then
    BUILD_MODE=$network ./scripts/docker-build.sh --station
  fi

  if should_build_artifacts || [ ! -f ./artifacts/upgrader/upgrader.wasm.gz ]; then
    BUILD_MODE=$network ./scripts/docker-build.sh --upgrader
  fi

  echo "Station and upgrader WASMs are ready."
}

function setup_cycles_wallet() {
  local network="$(get_network)"

  cycles_wallet_id_output=$(dfx identity get-wallet --network $network 2>/dev/null || echo "")

  if [ -z "$cycles_wallet_id_output" ]; then
    echo "Cycles wallet does not exist, using the default mainnet wallet for the deployment."
    cycles_wallet_id=$(dfx identity get-wallet --network ic)
    dfx identity set-wallet "$cycles_wallet_id" --network $network
  fi
}

function reset_control_panel() {
  local network="$(get_network)"

  if [ "$network" != "playground" ] && [ "$network" != "local" ]; then
    echo "ERROR: This operation is only supported on the playground or local network"
    exit 1
  fi

  echo "Resetting the \"$network\" network..."
  echo "This will remove the code and data for the control_panel canister."

  canister_id_output=$(dfx canister id control_panel --network $network 2>/dev/null || echo "")

  if [ -n "$canister_id_output" ]; then
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

  echo "Preparing the control_panel wasm..."

  if should_build_artifacts || [ ! -f ./artifacts/control-panel/control_panel.wasm.gz ]; then
    BUILD_MODE=$network ./scripts/docker-build.sh --control-panel
  fi

  # Read the WASM files and convert them to hex format
  upgrader_wasm_module_bytes=$(hexdump -ve '1/1 "%.2x"' ./artifacts/upgrader/upgrader.wasm.gz | sed 's/../\\&/g')
  station_wasm_module_bytes=$(hexdump -ve '1/1 "%.2x"' ./artifacts/station/station.wasm.gz | sed 's/../\\&/g')

  canister_id_output=$(dfx canister id control_panel --network $network 2>/dev/null || echo "")

  if [ -z "$canister_id_output" ]; then
    echo "Canister 'control_panel' does not exist, creating and installing..."

    dfx canister create control_panel --network $network --with-cycles 5000000000000 $([[ -n "$subnet_type" ]] && echo "--subnet-type $subnet_type")
    dfx canister install control_panel --network $network --wasm ./artifacts/control-panel/control_panel.wasm.gz
  else
    echo "Canister 'control_panel' already exists with ID: $canister_id_output"

    module_hash=$(dfx canister info control_panel --network $network | grep "Module hash" | awk '{print $3}')

    if [ "$module_hash" == "None" ]; then
      echo "Installing the wasm module to the control_panel canister..."
      dfx canister install control_panel --network $network --wasm ./artifacts/control-panel/control_panel.wasm.gz --mode install
    else
      echo "Upgrading the wasm module to the control_panel canister..."
      dfx canister install control_panel --network $network --wasm ./artifacts/control-panel/control_panel.wasm.gz --mode upgrade --yes
    fi
  fi

  echo "Updating the control_panel canister with the new station and upgrader WASM modules..."
  dfx canister call control_panel --network $network upload_canister_modules --argument-file <(echo "(record { upgrader_wasm_module = opt blob \"$upgrader_wasm_module_bytes\"; station_wasm_module = null; })")
  dfx canister call control_panel --network $network upload_canister_modules --argument-file <(echo "(record { upgrader_wasm_module = null; station_wasm_module = opt blob \"$station_wasm_module_bytes\"; })")
}

function deploy_app_wallet() {
  local network=$(get_network)
  local subnet_type=$(get_subnet_type)

  echo "Deploying the Orbit Wallet to the '$network' network."

  if should_build_artifacts || [ ! -f ./artifacts/wallet-dapp/wallet-dapp.tar.gz ]; then
    BUILD_MODE=$network ./scripts/docker-build.sh --wallet-dapp
  fi

  if [ -d ./artifacts/wallet-dapp/dist ]; then
    rm -rf ./artifacts/wallet-dapp/dist
  fi

  mkdir -p ./artifacts/wallet-dapp/dist
  tar -xvf ./artifacts/wallet-dapp/wallet-dapp.tar.gz -C ./artifacts/wallet-dapp/dist

  canister_id_output=$(dfx canister id app_wallet --network $network 2>/dev/null || echo "")

  if [ -z "$canister_id_output" ]; then
    echo "Canister 'app_wallet' does not exist, creating and installing..."

    BUILD_MODE=$network dfx deploy --network $network app_wallet --with-cycles 2000000000000 $([[ -n "$subnet_type" ]] && echo "--subnet-type $subnet_type")
  else
    echo "Canister 'app_wallet' already exists with ID: $canister_id_output"
    echo
    echo "Uploading assets to the app_wallet canister..."

    icx-asset --pem $IDENTITY_PEM_PATH --replica $(get_replica_url) sync --no-delete $canister_id_output artifacts/wallet-dapp/dist
  fi
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
  --local)
    shift
    set_network "local"
    ;;
  --playground)
    shift
    set_network playground
    ;;
  --staging)
    shift
    set_network staging
    ;;
  --production)
    shift
    set_network production
    ;;
  *)
    echo "ERROR: unknown argument $1"
    usage
    echo
    echo "Use '$0 --help' for more information"
    exit 1
    ;;
  esac
  exec_function setup_enviroment
  identity_warning_confirmation
  if [ "${1-}" == "reset" ]; then
    shift
    exec_function reset_control_panel
  fi
  exec_function deploy_control_panel
  exec_function deploy_app_wallet
  echo
  echo -e "\e[1;32mDeployment completed successfully to the '$(get_network)' network.\e[0m"
  echo
done
