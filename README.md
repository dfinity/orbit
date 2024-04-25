> This software is still a work in progress and should only be used for development purposes.

# Orbit Wallet

[![Internet Computer](./docs/internet-computer-logo.jpg)](https://internetcomputer.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg?style=for-the-badge)](./LICENSE)

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Requirements](#requirements)
  - [Building the code](#building-the-code)

## Overview

**Orbit** is a crypto wallet built with trustless multi custody backed by smart contracts from its very foundation. Our mission is to provide the most convenient and secure solution to manage crypto assets as an individual or as a group, directly hosted on the Internet Computer as an innovative approach to a Networked Wallet. It’s suited for large crypto holders including OGs (individual persons), foundations, DAOs and other crypto-native companies.

## Getting Started

### Requirements

Please make sure you have the following installed:

- [Rust](https://www.rust-lang.org/learn/get-started)
- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [nvm](https://github.com/nvm-sh/nvm)

### Building the Code

Start a local replica listening on port 4943:

```
dfx start --clean --host 127.0.0.1:4943
```

Then the following steps can be used to setup the Orbit canister ecosystem for local development.

```bash
./orbit --init
```

This will build the canisters, install the required node modules and deploy the canisters to your local replica. All the canisters will be deployed to the `local` network with their fixed canister ids.

To access wallet interface and avoid issues, please add to your `/etc/hosts` file the following line:

```
# Orbit Wallet UI and Internet Identity
127.0.0.1 rdmx6-jaaaa-aaaaa-aaadq-cai.localhost werw6-ayaaa-aaaaa-774aa-cai.localhost
```

After adding this line, you can access the wallet interface at [http://werw6-ayaaa-aaaaa-774aa-cai.localhost:4943](http://werw6-ayaaa-aaaaa-774aa-cai.localhost:4943).

For a faster frontend development experience, you can run the following commands to enable hot-reloading and use it through locahost:

```bash
cd apps/wallet
nvm use
pnpm dev
```
