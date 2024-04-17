# Orbit glossary

This glossary is intended to provide a reference for users and developers to navigate within the Orbit ecosystem. It includes technical terms, Orbit-specific terms, and wallet terms.

# Table of Contents

- [Introduction](#introduction)
- [Internet Computer Terms](#technical-terms)
- [Orbit Components](#orbit-components)
- [Orbit Terms](#orbit-terms)
- [Orbit Wallet Terms](#orbit-wallet-terms)
- [Conclusion](#conclusion)

# Introduction

Orbit is composed of a set of canisters and tools that work together to provide a trustless multi-custody operations for managing digital assets. It is built on the Internet Computer and uses canister smart contracts to provide a secure and decentralized solution. It contains general purpose canisters, such as the Control Panel and Station, as well as, tailor made user interface for specific use cases, such as the Wallet UI for managing assets in a trustless multi-custody crypto wallet.

## Internet Computer Terms

### Canister

A canister is a type of smart contract that bundles code and state. A canister can be deployed as a smart contract on the Internet Computer and accessed over the Internet. See [Canister](https://internetcomputer.org/how-it-works/canister-lifecycle/#canisters) for more information.

### Internet Identity

Internet Identity is a decentralized identity system that allows users to authenticate themselves to the Internet Computer. It is a key component of the Orbit Wallet, as it allows users to sign in and access their wallets. See [Internet Identity](https://internetcomputer.org/docs/current/references/ii-spec) for more information.

### Principal

Principals are generic identifiers for canisters, users and possibly other concepts in the future. As far as most uses of the Internet Computer are concerned they are opaque binary blobs with a length between 0 and 29 bytes, and there is intentionally no mechanism to tell canister ids and user ids apart. See [Principal ID](https://internetcomputer.org/docs/current/references/ic-interface-spec/#principal) for more information.

Example textual representation of Principals: `un4fu-tqaaa-aaaab-qadjq-cai`

## Orbit Components

### Control panel

The control panel is a canister that facilitates common operations for accessing and managing Orbit Stations. It is responsible for storing the list of station canisters the user is associated and facilitating the creation of new station canisters.

### Station

A station is the core canister component of Orbit. It is a trustless multi-custody canister that allows users to manage their crypto assets and operations. Stations are created by the user and can be shared with other users to create a multi-signature system for managing assets.

### Upgrader

The upgrader is a canister that is responsible for upgrading the Orbit station canister. Each station canister has an associated upgrader canister that allows the station users to upgrade the station canister to a new version.

### Wallet UI

The Wallet UI is the user interface (UI) of the Orbit Wallet. It is a web application that allows users to interact with their stattions, view their balances, send and receive tokens, and manage their accounts.

## Orbit Terms

### Request

All operations in the Station canister are performed through requests. A request is a signed message that contains the operation to be performed and the necessary parameters. Requests are signed by the user's Internet Identity and are sent to the Station canister to perform the operation, such as sending tokens or adding a new account. Requests are only executed if they pass the relevant checks and validations, this includes the required signatures and permissions applied by the approval policies of the station.

#### Request Statuses

The status of a request can be one of the following:

- **AwaitingApproval:** A request is in the awaiting approval state when it has been submitted to the station canister for evaluation. These requests can be canceled by the user who submitted them.

- **Approved:** A request is in the approved state when it has passed all rules in the Approval Policy. Approved requests can be executed by the station canister.

- **Rejected:** A request is in the rejected state when it has failed to pass the rules in the Approval Policy. Rejected requests cannot be executed by the station canister.

- **Scheduled:** A request is in the scheduled state when it has been approved and is waiting to be executed. Scheduled requests are executed by the station canister in the scheduled order.

- **Cancelled:** A request is in the cancelled state when it has been canceled by the user who submitted it. Cancelled requests cannot be executed by the station canister.

- **Processing:** A request is in the processing state when it is being executed by the station canister. Processing requests are in the process of performing the operation specified in the request.

- **Completed:** A request is in the completed state when it has been successfully executed by the station canister. Completed requests have successfully performed the operation specified in the request.

- **Failed:** A request is in the failed state when it has failed to be executed by the station canister. Failed requests have not performed the operation specified in the request.

### Approval Policy

An Approval Policy is a set of rules that define the behavior of requests in the station canister. Approval policies are used to enforce security measures, such as requiring multiple signatures to execute a request, they are defined by authorized users and can be customized to fit the needs of different use cases.

### Approval Rule

An Approval Rule is a condition that must be met for a request to be approved by the station canister. Approval policy rules can be based on the request type, the request amount, the request sender, or any other parameter of the request. Approval policy rules are defined by authorized users and can be customized to fit the needs of different use cases.

### Approval Rule Types

Approval rules can be of the following types:

- **Auto-Approve:** An auto-approve rule automatically approves requests, regardless of the request parameters. Auto-approve rules can be used to expedite the execution of requests that do not require manual approval.

- **Quorum:** A threshold rule requires a minimum number of signatures to approve a request. Threshold rules can be used to enforce multi-signature requirements for executing requests.

- **Percentage:** A percentage rule requires a minimum percentage of users to approve a request. Percentage rules can be used to enforce multi-signature requirements based on the percentage of users in the station.

- **Allow listed:** An allow-list rule are only relevant to transfers and checks if the destination address is in the allow-list. Allow-list rules can be used to restrict transfers to specific addresses that are in the address book.

- **Allow listed by label:** An allow-list by label rule are only relevant to transfers and checks if the destination address is in the allow-list with the provided label. Allow-list by label rules can be used to restrict transfers to specific addresses by defined labels.

There are also the following composite rules that can be used to combine multiple rules:

- **All of:** A rule that requires all of the sub-rules to be approved. This rule can be used to enforce complex approval requirements that involve multiple conditions.

- **Any of:** A rule that requires any of the sub-rules to be approved. This rule can be used to enforce flexible approval requirements that allow for multiple conditions.

- **Not:** A rule that requires the sub-rule to be rejected. This rule can be used to enforce negative approval requirements that prevent specific conditions from being met. An example of this rule is to prevent a specific user from approving a request.

### Permission

Permissions are rules that define the actions that a user can perform in a station canister. Permissions are defined by the authorized station users and can be customized to fit the needs of different use cases. Permissions can be granted to individual users, groups of users or any (un)authenticated user and can be revoked at any time.

### Account

An account is a record in the station canister that represents a user's ownership of a specific asset. Accounts can hold different types of assets, such as tokens, NFTs, or other fungible or non-fungible assets. Accounts can be created, updated, and archived by the station users through requests.

#### Account Name

An account name is a human-readable name that represents the account in the station canister. Account names can be customized by privileged users and are unique within the station canister.

#### Account Address

An account address is a unique identifier that represents the account address in relation to the asset it holds.

#### Account Balance

An account balance is the amount of a specific asset that an account holds. Account balances are updated when assets are deposited or withdrawn from the account.

### Address Book

The address book is a feature of the station canister that allows users to store and manage a list of addresses. The address book can be used to store the addresses of other users, contracts, or services that the user interacts with frequently.

#### Address Book Entry

An address book entry is a record in the address book that represents an address and its associated metadata. Address book entries can be created, updated, and deleted by the user.

#### Address Book Entry Name

An address book entry name is a human-readable name that represents the address book entry in the address book. Address book entry names can be customized by the user and are unique within the address book for the same address type.

### Station Name

The station name is a human-readable name that represents the station canister. The station name can be customized by the user and is unique within the user's account of the control panel canister.

### Station Version

The station version is a number that represents the version of the station canister. The version is used to tailor the Wallet UI to the specific features and capabilities of the station canister.

### Station ID

The station ID is a unique identifier that represents the station canister in the Internet Computer. The station ID is used to uniquely identify the station canister. It uses the [Principal](#principal) format.

### Identity

The identity is a unique identifier that represents the user in the Internet Computer. The identity is used to uniquely identify the user and is associated with the user's Control Panel canister. The identity is also used to associate the user with the station canister. It uses the [Principal](#principal) format.

## Orbit Wallet Terms

### Wallet ID

The Wallet ID is a unique identifier that represents the user's wallet in the Internet Computer. The Wallet ID is used to uniquely identify the user's wallet and is associated with the user's Internet Identity. The Wallet ID is the same as the [Station ID](#station-id) used more broadly in the Orbit ecosystem.

### Wallet Name

The wallet name is a human-readable name that represents the user's wallet in the Internet Computer. The wallet name can be customized by the user and is unique within the user's account of the control panel canister. The wallet name is the same as the [Station Name](#station-name) used more broadly in the Orbit ecosystem.

# Conclusion

This glossary is a living document that will be updated as new terms and concepts are introduced in the Orbit ecosystem. If you have any questions or suggestions for terms to be added to the glossary, please feel free to reach out to the Orbit team.