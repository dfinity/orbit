---
title: Managing External Canisters in Orbit Wallet
sidebar:
  label: External Canisters
description: This page provide the necessary information to get started with Orbit.
---

Orbit Wallet provides advanced features to manage external canisters on the Internet Computer. This includes monitoring, managing permissions, and performing upgrades for seamless integration with decentralized applications.

## **What are External Canisters?**

Canisters are smart contracts on the Internet Computer that store and execute code. External canisters are those that operate outside the immediate Orbit Wallet system but are accessible and manageable through Orbit Wallet's features.

## **Why Manage External Canisters?**

Managing external canisters through Orbit Wallet allows you to:

- **Automate Canister Monitoring:** Keep track of their status and cycles.
- **Perform Upgrades:** Install and upgrade code securely.
- **Control Access Permissions:** Define who can interact with the canister.
- **Ensure Stability:** Avoid downtime due to cycle depletion or permission mismanagement.

## **Monitoring External Canisters**

### **Steps to Enable Monitoring:**

1. Navigate to **Canisters** from your Orbit Wallet dashboard.
2. Click **"Add Canister"** and fill in the _Configuration_, _Permissions_, and _Policies_.
3. On the canister detail page, enable monitoring with **Monitor** button in right column:
   - Set strategy to monitor cycles
   - Configure strategy to obtain cycles if balance is low on the Station
4. Save the configuration.

## **Managing Canister Permissions**

### **Steps to Manage Permissions:**

1. Select the canister from the **Canisters** dashboard.
2. Navigate to **Method call configurations** and click **Add new method**.
3. Configure name, permissions, and policies for the given method.
4. Save the changes.

:::caution[Security Tip]
Regularly review canister permissions to prevent unauthorized access.
:::

## **Upgrading External Canisters**

### **Steps for Canister Upgrades:**

1. Select the canister you want to upgrade.
2. Click **"Install"** and choose installation method:
   - **Install:** Install a new WASM file into the canister
   - **Upgrade:** Replace the existing canister code with a new version.
   - **Reinstall:** Reinstall the canister with the same code.
3. Upload the WASM file and submit the request.
4. Wait for the approval process (if multi-signature policies are enabled).
5. Monitor the canister status to ensure the upgrade completes successfully.

:::caution[Important]
Always test new versions in a staging environment before upgrading production canisters.
:::

## **Best Practices for Managing External Canisters**

- **Automate Monitoring:** Use alerts to stay informed of cycle balance changes.
- **Secure Permissions:** Restrict access to canister management functions.
- **Plan Upgrades:** Schedule upgrades during low-traffic periods to minimize disruptions.

## **Known Limitations**

Orbit Wallet currently does not support asset canister management flows. Teams managing assets must rely on the dfx-orbit CLI, which provides the necessary functionality for secure and policy-driven asset updates.
