---
title: Upgrading Your Orbit Wallet
sidebar:
  label: System Upgrades
description: This page provide the necessary information to get started with Orbit.
---

Orbit Wallet account consists of two main canisters: the **Station** (the wallet canister) and the **Upgrader** (a supporting canister used for upgrades and disaster recovery). When a new version of either the Station or the Upgrader is available, you will be notified in the dashboard UI. This page explains how to perform upgrades and ensure your Orbit Wallet is always up-to-date.

## Overview of Upgrades

- **Station Canister**: The core wallet canister that manages your assets and operations.
- **Upgrader Canister**: A supporting canister responsible for upgrading the Station and enabling disaster recovery.

When a new version is released, you must upgrade both canisters separately. The dashboard UI will notify you of available updates and guide you through the process.

## Performing an Upgrade

### Step 1: Open the Upgrade Dialog

1. Click on the **upgrade notification** in the top-right corner of the dashboard.
2. This will open the **Upgrade Dialog**, which shows:
   - The canister to be upgraded (Station or Upgrader).
   - Details about the new version (e.g., version number, release notes).

### Step 2: Review the Upgrade Details

- Carefully read the information provided in the Upgrade Dialog.
- Ensure you understand the changes and how they might affect your Wallet.

### Step 3: Proceed with the Upgrade

1. Click **Upgrade** to start the process.
2. The system will begin upgrading the selected canister (Station or Upgrader).
3. Wait for the upgrade to complete. This may take a few moments.

### Step 4: Repeat for Additional Upgrades

- If multiple updates are available (e.g., both Station and Upgrader), you must upgrade them **one at a time**.
- After completing one upgrade, return to the dashboard and check for additional notifications.
- Repeat the process for each available update.

:::caution
The Upgrader canister plays a critical role in disaster recovery. Ensure it is always up-to-date to maintain the ability to recover your Wallet in case of issues.
:::

## Troubleshooting Upgrades

If you encounter issues during an upgrade:

1. **Check Your Internet Connection**:

   - Ensure you have a stable internet connection during the upgrade process.

2. **Refresh the Dashboard**:

   - If the upgrade seems stuck, refresh the dashboard UI and check for updates again.

3. **Contact Support**:
   - If the issue persists, reach out to Orbit Wallet support for assistance. Provide details about the upgrade and any error messages you encountered.

## Frequently Asked Questions (FAQs)

#### Q: What happens if I don’t upgrade?

A: While your Wallet will continue to function, you may miss out on new features, security patches, and performance improvements. It’s recommended to upgrade as soon as updates are available.

#### Q: What happens if an upgrade fails?

Upgrades are atomic, meaning that if they fail, the system will automatically revert to the previous version. You can then try the upgrade again.

#### Q: Can I skip an upgrade?

A: No, upgrades must be performed sequentially. Skipping an upgrade may result in compatibility issues.

#### Q: Will my data be affected by an upgrade?

A: Upgrades are designed to preserve your data.

#### Q: How often are updates released?

A: Updates are released periodically to introduce new features, fix bugs, and improve security. The frequency depends on the development cycle.
