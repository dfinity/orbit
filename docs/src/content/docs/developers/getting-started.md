---
title: Getting Started
description: This page provide the necessary information to get started with developing Orbit.
---

Orbit is an innovative platform layer built for the Internet Computer Protocol (ICP) and designed to extend to other blockchains. Follow this guide to set up and start developing with Orbit.

---

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust Toolchain**: Install from [rust-lang.org](https://www.rust-lang.org/).
- **DFX**: Install the DFINITY Canister SDK by following the [Internet Computer documentation](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove/).
- **Node.js**: Download and install from [nodejs.org](https://nodejs.org/).
- **pnpm**: Install globally using npm:
  ```bash
  npm install -g pnpm
  ```

---

## Step 1: Clone the Orbit Repository

Start by cloning the Orbit repository from GitHub:

```bash
git clone https://github.com/dfinity/orbit.git
cd orbit
```

---

## Step 2: Install Dependencies

Use `pnpm` to install the project dependencies:

```bash
pnpm install
```

---

## Step 3: Set Up the Development Environment

Run the helper script provided by Orbit to configure the development environment:

```bash
./orbit
```

This script initializes the necessary settings for your environment.

---

## Step 4: Start the Local Development Server

1. Start the Internet Computer replica in the background:

   ```bash
   dfx start --clean --pocketic --host 127.0.0.1:4943
   ```

2. Deploy the canisters to the local replica:

   ```bash
   dfx deploy
   ```

3. Navigate to the wallet application directory:

   ```bash
   cd apps/wallet
   ```

4. Start the development server:

   ```bash
   pnpm dev
   ```

5. Open your browser and go to `http://localhost:3000` to access the Orbit wallet interface.

---

## Step 5: Explore the Codebase

Here’s a quick overview of the Orbit project structure:

- **Core Logic**: Located in the `core` directory.
- **Wallet Frontend**: Found in the `apps/wallet` directory.
- **Shared Libraries**: Available in the `libs` directory.

---

## Step 6: Contribute to Orbit

If you'd like to contribute:

1. Read the [HACKING.md](https://github.com/dfinity/orbit/blob/main/HACKING.md) file for contribution guidelines.
2. Ensure your code follows the project's coding standards and includes tests.

---

## Resources

- [Orbit GitHub Repository](https://github.com/dfinity/orbit)
- [DFX Documentation](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove/)

---

You're now ready to develop and contribute to Orbit! 🚀
