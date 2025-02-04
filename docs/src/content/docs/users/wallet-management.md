---
title: Wallet Management in Orbit Wallet
sidebar:
   label: Wallet Management
description: This page provide the necessary information to get started with Orbit.
---


In Orbit, a **Wallet** (also referred to as a **Station**) is the core unit for managing your assets. Each Wallet can have multiple **Accounts**, and each Account can hold different assets. Users can be granted specific privileges and policies per Account, enabling flexible and secure asset management.



## Wallet (Station) Overview

A **Wallet** (or **Station**) is a trust-less, multi-custody canister that allows you to manage your crypto assets and operations. Key features of a Wallet include:

- **Multi-Account Support**: Each Wallet can have multiple Accounts, each holding different assets.
- **Multi-User Access**: Wallets can be shared with other users, enabling collaborative asset management.
- **Custom Policies**: Define approval policies and permissions for each Account.



## Accounts and Wallets Relationship

### What is an Account?
An **Account** is a record within a Wallet that represents ownership of specific assets. Each Account can hold multiple assets, such as ICP, or other ICRC-1 based tokens.

### Key Features of Accounts:
- **Unique Names**: Each Account has a human-readable name for easy identification.
- **Asset Management**: Accounts can hold assets from different blockchains and token standards.
- **Privileges and Policies**: Users can define specific permissions and approval policies for each Account.



## Adding Users to a Wallet

To add users to your Wallet and grant them access to specific Accounts:

1. **Navigate to Settings**:
   - Go to the **Settings** tab in your Wallet.
   - Select **Users**.

2. **Add a New User**:
   - Click **Create User**.
   - Enter the user's **Identity** (their unique identifier on the Internet Identity).
   - Assign the user to one or more Groups.

3. **Set Permissions**:
   - Define the user's or group's permissions for each Account.

4. **Save Changes**:
   - Click **Save** to apply the changes.
   - The user will now have access to the specified Accounts based on their permissions.



## Managing Accounts

### Creating an Account
To create a new Account within your Wallet:

1. Go to the **Accounts** tab in your Wallet.
2. Click **Create Account**.
3. Enter a unique **Account Name**.
4. Select the **Assets** to be associated with the Account.
5. Click **Save**.

### Editing an Account
To edit an existing Account:

1. Go to the **Accounts** tab.
2. Click on the Account you want to edit.
3. Update the **Account Name** (via Settings button) or **Associated Assets** (via the Asset detail page).
4. Click **Save**.

### Deleting an Account
Accounts cannot be deleted at the moment. However, you can remove all assets from an Account.

:::note
Deleting an Asset does not delete the assets on the blockchain. You can always re-add the Asset to the Account later.
:::



## Configuring Approval Policies

Approval policies define the rules for executing transactions within an Account. You can configure policies at the Account level.

### Types of Approval Policies
1. **Auto-Approve**:
   - Transactions are automatically approved without additional signatures.
   - Suitable for single-user Accounts.

2. **Quorum**:
   - Requires a minimum number of signatures to approve a transaction.
   - Example: 2 out of 3 users must approve.

3. **Quorum-Percentage**:
   - Requires a minimum percentage of users to approve a transaction.
   - Example: 50% of users must approve.

4. **Allow-listed Addresses**:
   - Restricts transactions to specific addresses in the Address Book.

### Setting Up an Approval Policy
1. Go to the **Accounts** tab.
2. Click on the Account you want to configure.
3. Navigate to **Approval Policies**.
4. Select the type of policy you want to apply.
5. Configure the policy rules (e.g., number of signatures, percentage, or allow-listed addresses).
6. Click **Save**.
7. 


## Monitoring Wallet Activity

To monitor activity within your Wallet:

1. Go to the **Activity** tab.
2. View a list of all transactions, including:
   - **Transaction Type**: Send, receive, or approve.
   - **Status**: Created, approved, completed, or failed.
   - **Timestamp**: When the transaction occurred.



## Tips for Managing Wallets

- **Use Descriptive Names**: Assign meaningful names to Wallets and Accounts for easy identification.
- **Regularly Review Policies**: Ensure approval policies align with your security requirements.
- **Monitor User Access**: Periodically review and update user permissions.
- **Backup Important Data**: Keep a record of Principal IDs, Account names, and approval policies.



## Example Workflow

### Scenario: Managing a Multi-User Wallet
1. **Create a Wallet**:
   - Name: "Team Treasury"
   - Add Accounts: "Marketing Budget," "Development Fund"

2. **Add Users**:
   - User A: Full access to "Marketing Budget"
   - User B: Transaction approval for "Development Fund"

3. **Set Approval Policies**:
   - "Marketing Budget": Auto-Approve (single user).
   - "Development Fund": Quorum (2 out of 3 users must approve).

4. **Monitor Activity**:
   - Regularly check the **Transfer Requests** tab for transaction history.