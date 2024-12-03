## 0.2.0 (2024-12-03)


### 🚀 Features

- **station:** external canister snapshots ([#429](https://github.com/dfinity/orbit/pull/429))

- **station:** monitor external canisters ([#416](https://github.com/dfinity/orbit/pull/416))


### 🩹 Fixes

- **wallet:** fix request export order ([#449](https://github.com/dfinity/orbit/pull/449))


### ❤️  Thank You

- Jan Hrubes @jedna
- mraszyk @mraszyk
- olaszakos

## 0.1.0 (2024-11-27)


### 🚀 Features

- ⚠️  **station:** multi chain support ([#374](https://github.com/dfinity/orbit/pull/374))


### 🩹 Fixes

- **control-panel:** fix initial station deploy ([#441](https://github.com/dfinity/orbit/pull/441))


### ⚠️  Breaking Changes

- ⚠️  **station:** multi chain support ([#374](https://github.com/dfinity/orbit/pull/374))

### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.1.0


### ❤️  Thank You

- Kepler Vital
- olaszakos

## 0.0.2-alpha.8 (2024-11-22)


### 🚀 Features

- **dfx-orbit:** Review list pagination ([#403](https://github.com/dfinity/orbit/pull/403))

- **wallet:** use didc for candid parsing ([#402](https://github.com/dfinity/orbit/pull/402))

- **station:** add metadata to external canisters ([#418](https://github.com/dfinity/orbit/pull/418))

- **station:** add expiration dt during request creation ([#424](https://github.com/dfinity/orbit/pull/424))

- **station:** requestor can cancel pending requests ([#426](https://github.com/dfinity/orbit/pull/426))


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.7 (2024-10-22)


### 🚀 Features

- **dfx-orbit:** dfx-orbit version 0.5.0 ([#370](https://github.com/dfinity/orbit/pull/370))

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **station:** add canister execution method and validation pair edit variant ([#381](https://github.com/dfinity/orbit/pull/381))

- **station:** allow external canister creation on subnet of choice ([#383](https://github.com/dfinity/orbit/pull/383))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))


### 🩹 Fixes

- **release:** workaround nx bug in release script ([#375](https://github.com/dfinity/orbit/pull/375))


### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.0.2-alpha.6


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- mraszyk @mraszyk

## 0.0.2-alpha.6 (2024-10-02)


### 🚀 Features

- **station:** notify failed station upgrade

- **station:** add ability to cancel pending requests to EditUserOperation

- **station,upgrader:** support large station and upgrader wasm

- **station:** external canister input to accept opt policies and permissions by type

- **station:** large WASM support for external canisters


### ❤️  Thank You

- Jan Hrubes
- Kepler Vital
- Leon Tan
- mraszyk

## 0.0.2-alpha.5 (2024-08-26)


### 🚀 Features

- **wallet:** add disaster recovery config UI

- **station:** address book of external canisters

- **station:** add external canister management

- **station:** external canister method calls permissions

- **station:** integrate request policies and external canisters

- **station:** add external canister endpoints

- **station:** mint cycles to top up station

- **station:** use different types for input and get of external canister policies

- **station:** use own policies & permissions for canisters cycles fund

- **station:** add frontend to manage cycle obtain startegies

- **station:** add fee, comment, from address to csv export

- **station:** optimized repository lookups


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.4 (2024-07-04)


### 🚀 Features

- **station:** disaster recovery MVP

- **upgrader:** disaster recovery flow reuses the same upgrader canister

- **upgrader:** disaster recovery recreates accounts


### ❤️  Thank You

- Kepler Vital
- Max
- olaszakos

## 0.0.2-alpha.3 (2024-06-24)


### 🚀 Features

- **station:** introduce ChangeManagedCanister request type

- **station:** add CreateManagedCanister request type

- **station:** add canister_status endpoint

- **station:** add CallExternalCanister request type

- **api:** Add the serde::Serialize trait to API types

- **station:** optional fallback controller


### ❤️  Thank You

- Jan Wendling
- Kepler Vital
- Max
- mraszyk

## 0.0.2-alpha.2 (2024-05-17)


### 🚀 Features

- add user station labels


### 🔥 Performance

- reduce canister running costs


### ❤️  Thank You

- Kepler Vital
- mraszyk
- olaszakos

## 0.0.2-alpha.1 (2024-05-13)


### 🚀 Features

- enable users to update the station name


### ❤️  Thank You

- Kepler Vital

## 0.0.2-alpha.0 (2024-05-12)


### 🚀 Features

- let user add station name

- show acceptance rules for proposal

- add notifications for the requester for failed/rejected request

- added asset management core

- add account proposal operations

- add user group proposals

- policy specifiers

- add updated user model

- add access control with action based resource policies

- implement user group list and get

- add list and get for access control policies

- add list and get of proposal policies

- add operations to mutate access policies

- add operations to mutate proposal policies

- add me endpoint to fetch user privileges


### ❤️  Thank You

- Kepler Vital
- olaszakos