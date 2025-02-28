# Function to print messages with a dynamic border, accounting for multi-line messages
print_message() {
  local msg="$1"
  local min_length="${2:-50}"
  local max_length=0

  # Split the message into lines and find the longest
  while IFS= read -r line; do
    ((${#line} > max_length)) && max_length=${#line}
  done <<<"$msg"

  # Ensure the message is at least `min_length` characters long
  if ((max_length < min_length)); then
    max_length=$min_length
  fi

  # Generate a border based on the length of the longest line
  local border=$(printf '%*s' "$max_length" '' | tr ' ' '-')

  echo "$border-"
  # Print each line of the message
  while IFS= read -r line; do
    printf "%s\n" "$line"
  done <<<"$msg"
  echo "$border-"
}

function get_network() {
  local network=${IC_NETWORK:-local}
  echo "$network"
}

function set_network() {
  local network=$1
  export IC_NETWORK=$network
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
  local identity=$(dfx identity whoami)

  if [ -z "$identity" ]; then
    echo "No identity found, please login to your dfx environment."
    exit 1
  fi

  echo "Current identity: $identity"
  echo

  local identity_store_path=${DFX_DEFAULT_IDENTITY_STORE_PATH}

  if [ -z "$IDENTITY_PEM_PATH" ]; then
    local identity_pem_path="${identity_store_path}/${identity}/${identity}.pem"

    # Check if the identity pem file exists, else fallback to the id.pem filename
    if [ ! -f "$identity_pem_path" ]; then
      identity_pem_path="${identity_store_path}/${identity}/identity.pem"
    fi

    if [ ! -f "$identity_pem_path" ]; then
      identity_pem_path="${identity_store_path}/${identity}/id.pem"
    fi

    export IDENTITY_PEM_PATH=$identity_pem_path
  fi

  if [ ! -f "$IDENTITY_PEM_PATH" ]; then
    echo -e "\e[1;31m"
    echo "ERROR: Identity PEM file not found for the identity: $identity"
    echo "Please make sure the identity is available in the default identity store path: $identity_store_path"
    echo -e "\e[0m"

    exit 1
  fi
}

function install_icx_asset() {
  if ! command -v icx-asset >/dev/null 2>&1; then
    echo "icx-asset not found, installing..."

    cargo install --locked icx-asset --version 0.21.0

    echo "icx-asset installed successfully."

  fi
}
