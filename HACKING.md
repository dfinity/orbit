# Hacking

This document covers useful information for developers who want to contribute to the project.

## Table of content

- [Development](#development)
- [Deployment](#deployment)
- [Internationalization](#internationalization)

## Development

### Frontend

For a faster frontend development experience, you can run the following commands to enable hot-reloading and use it through locahost:

```bash
cd apps/wallet
nvm use
pnpm dev
```

### Hosts file

If you are facing issues with the wallet UI and Internet Identity, you can add the following entries to your hosts file:

```
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
