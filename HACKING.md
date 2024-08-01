# Hacking

This document covers useful information for developers who want to contribute to the project.

## Table of content

- [Development](#development)
- [Deployment](#deployment)
- [Internationalization](#internationalization)

## Development

### Frontend

For a faster frontend development experience, you can run the following commands to enable hot-reloading and use it through locahost:

```sh
cd apps/wallet
nvm use
pnpm dev
```

### Hosts file

If you are facing issues with the wallet UI and Internet Identity, you can add the following entries to your hosts file:

```sh
# Orbit Wallet UI and Internet Identity
127.0.0.1 rdmx6-jaaaa-aaaaa-aaadq-cai.localhost werw6-ayaaa-aaaaa-774aa-cai.localhost
```

## Deployment

To deploy the canisters to the Internet Computer, you can run the following commands from the root of the project:

```bash
./scripts/deploy.sh $NETWORK
```

Where `$NETWORK` is the target network you want to deploy to. The available options are `production`, `local`, `staging` and `playground`.

This script will use your current DFX identity to deploy the canisters.

## Internationalization

### Orbit Wallet

The Orbit Wallet UI is localized using `vue-i18n`. The translations are stored in the [apps/wallet/src/locales](./apps/wallet/src/locales) directory.

## Playing

To play, make sure you

- run pnpm dev AFTER all the rest
- you approve your identity, i.e. `./orbit --approve-waiting-list [user-id]`
- you get some tokens, `dfx ledger transfer --memo 1 [account-id] --icp 1250`

## To "almost hot reload"

## 1. setting up the environment

```sh
dfx start --clean --host 127.0.0.1:4943 --background
./orbit --init
```

### 1b. Running the frontend

```sh
cd apps/wallet
nvm use
pnpm dev
```

## 2. stopping the environment

```sh
dfx stop
```

## 3. resume and make some backend changes

```sh
dfx start --host 127.0.0.1:4943 --background
./scripts/generate-wasm.sh station
upload the wasm from the ./wasms folder in the /settings/system page of the orbit wallet
```

## 4. run tests

```sh
cargo test --locked --workspace --exclude integration-tests
```

## 5. run integration tests

```sh
./scripts/run-integration-tests.sh
```
