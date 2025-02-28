---
title: Managing Assets in Orbit Wallet
sidebar:
  label: Managing Assets
description: This page provide the necessary information to get started with Orbit.
---

In Orbit Wallet, you can manage a wide range of digital assets securely and efficiently. This section guides you through the steps to add, send, and receive assets using your Orbit Wallet account.

## **Supported Assets**

Orbit Wallet supports the management of following digital assets:

- **ICP (Internet Computer Protocol token)**
- **ICRC-1 tokens**

:::tip
Additional token standards may be supported in future updates.
:::

## **Adding Assets to Your Wallet**

A station is usually created with a default ICP account. If a desired asset is not automatically visible in your wallet:

1. Navigate to **"Settings > Assets"** in your dashboard.
2. Click **"New Asset"**.
3. Enter the asset's contract details (token name, type, and identifier).
4. Click **"Create"** to save the new asset.

## **Sending Assets**

1. Select the asset you wish to send from your wallet dashboard.
2. Click **"New Transfer"**.
3. Enter the **recipient address** and **amount** to transfer.
4. Review the transaction details and click **"Create"**.
5. For multi-signature wallets, additional user approvals may be required.

:::note
Transaction fees and confirmation times depend on the type of asset.
:::

## **Receiving Assets**

1. Select the asset you want to receive in your dashboard.
2. Copy the asset address on top.
3. Share the address with the sender.

:::danger
Always verify your address before sharing to avoid errors.
:::

## **Bulk Transactions**

Bulk transactions enable efficient handling of multiple transfers simultaneously, particularly useful for enterprise operations.

### **Creating Bulk Transactions:**

1. Navigate to the asset page.
2. Click **Upload CSV** and upload a **CSV file** with the following columns:
   - "to" with recipient addresses
   - "amount" with transaction amounts
   - (optional) "comment" for additional notes
3. Review and click **Transfer** to submit the batch for processing.
4. For multi-approval wallets, additional user approvals may be required.

:::caution
Ensure the CSV file follows the required format to avoid errors. **Header row is mandatory**.
:::

## **Additional Tips**

- **Stay Secure:** Double-check recipient addresses before confirming transactions.
- **Monitor Activity:** Use the transaction history feature to review past transfers.
- **Permission Management:** Ensure you have the correct role for sending assets if multi-signature approval is required.
