#!/usr/bin/env bash
set -eEuo pipefail

maybe_load_nvm() {
  if [ -s "${NVM_DIR:-$HOME/.nvm}/nvm.sh" ]; then
    echo "Loading nvm from ${NVM_DIR:-$HOME/.nvm}"
    \. "${NVM_DIR:-$HOME/.nvm}/nvm.sh"
  fi

  command -v nvm >/dev/null 2>&1
}

is_n_installed() {
  command -v n >/dev/null 2>&1
}

maybe_load_n() {
  if ! is_n_installed; then
    echo "n is not installed, installing..."
    npm install -g n
  fi

  is_n_installed
}

# Attempt to read the required Node.js version from .nvmrc
if [ -f ".nvmrc" ]; then
  required_node_version="$(cat .nvmrc)"
else
  echo ".nvmrc file not found. Please ensure it exists and specifies the required Node.js version."
  exit 1
fi

# Check if nvm is installed and use it if available
if maybe_load_nvm; then
  echo "Using nvm to manage Node.js versions."
  echo "Installing and using Node.js version $required_node_version..."
  nvm install "$required_node_version"
  nvm use "$required_node_version"
  echo "Node.js is set to the required version using nvm."
# Check if n is installed and use it if available
elif maybe_load_n; then
  echo "Using n to manage Node.js versions."
  n "$required_node_version"
  echo "Node.js is set to the required version using n."
else
  echo "Neither nvm nor n is installed. Please install one of them to manage Node.js versions."
  exit 1
fi
