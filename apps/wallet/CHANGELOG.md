## 0.2.0 (2024-12-03)


### 🚀 Features

- **wallet:** show upgrader ID on station info card ([#446](https://github.com/dfinity/orbit/pull/446))

- **station:** external canister snapshots ([#429](https://github.com/dfinity/orbit/pull/429))

- **station:** monitor external canisters ([#416](https://github.com/dfinity/orbit/pull/416))

- **wallet:** search for address on address book page ([#450](https://github.com/dfinity/orbit/pull/450))


### 🩹 Fixes

- **wallet:** fix request export order ([#449](https://github.com/dfinity/orbit/pull/449))


### ❤️  Thank You

- Jan Hrubes @jedna
- mraszyk @mraszyk
- olaszakos

## 0.1.1 (2024-11-28)


### 🩹 Fixes

- **wallet:** add new opt candid metadata field ([#447](https://github.com/dfinity/orbit/pull/447))


### ❤️  Thank You

- Kepler Vital

## 0.1.0 (2024-11-27)


### 🚀 Features

- ⚠️  **station:** multi chain support ([#374](https://github.com/dfinity/orbit/pull/374))


### 🩹 Fixes

- **control-panel:** fix initial station deploy ([#441](https://github.com/dfinity/orbit/pull/441))

- **wallet:** fix semver compatibility check ([#442](https://github.com/dfinity/orbit/pull/442))


### ⚠️  Breaking Changes

- ⚠️  **station:** multi chain support ([#374](https://github.com/dfinity/orbit/pull/374))

### ❤️  Thank You

- Kepler Vital
- olaszakos

## 0.0.2-alpha.15 (2024-11-22)


### 🩹 Fixes

- **wallet:** transfer requests page should only show transfers ([#438](https://github.com/dfinity/orbit/pull/438))


### ❤️  Thank You

- Kepler Vital

## 0.0.2-alpha.14 (2024-11-22)


### 🚀 Features

- **station:** add metadata to external canisters ([#418](https://github.com/dfinity/orbit/pull/418))

- **station:** add expiration dt during request creation ([#424](https://github.com/dfinity/orbit/pull/424))

- **station:** requestor can cancel pending requests ([#426](https://github.com/dfinity/orbit/pull/426))

- **enable:** wallet ui accepts wasm chunk store from registry ([#433](https://github.com/dfinity/orbit/pull/433))

- **wallet:** add close btn to error dialog ([#434](https://github.com/dfinity/orbit/pull/434))


### 🩹 Fixes

- **station:** add missing metadata field from UI ([#427](https://github.com/dfinity/orbit/pull/427))


### ❤️  Thank You

- Jan Hrubes @jedna
- Kepler Vital
- olaszakos

## 0.0.2-alpha.13 (2024-10-28)


### 🚀 Features

- **wallet:** enable webassembly loading ([#409](https://github.com/dfinity/orbit/pull/409))


### ❤️  Thank You

- Kepler Vital

## 0.0.2-alpha.12 (2024-10-25)


### 🚀 Features

- **dfx-orbit:** Review list pagination ([#403](https://github.com/dfinity/orbit/pull/403))

- **wallet:** use didc for candid parsing ([#402](https://github.com/dfinity/orbit/pull/402))


### ❤️  Thank You

- Kepler Vital
- Leon Tan

## 0.0.2-alpha.11 (2024-10-22)


### 🚀 Features

- **control-panel:** support deploying large station WASM ([#364](https://github.com/dfinity/orbit/pull/364))

- **control-panel:** support large WASM in registry ([#365](https://github.com/dfinity/orbit/pull/365))

- **dfx-orbit:** dfx-orbit version 0.5.0 ([#370](https://github.com/dfinity/orbit/pull/370))

- **control-panel:** allow deploying station to subnet of choice ([#372](https://github.com/dfinity/orbit/pull/372))

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **station:** add canister execution method and validation pair edit variant ([#381](https://github.com/dfinity/orbit/pull/381))

- **station:** allow external canister creation on subnet of choice ([#383](https://github.com/dfinity/orbit/pull/383))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))

- **station:** init with default external canisters policies ([#393](https://github.com/dfinity/orbit/pull/393))


### 🩹 Fixes

- **release:** workaround nx bug in release script ([#375](https://github.com/dfinity/orbit/pull/375))


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- mraszyk @mraszyk

## 0.0.2-alpha.10 (2024-10-02)


### 🚀 Features

- **station:** notify failed station upgrade

- **wallet:** external canisters list page

- **wallet:** add external canister setup wizard

- **wallet:** bootstrap external canister detail page

- **wallet:** add canister top up dialog

- **station:** add ability to cancel pending requests to EditUserOperation

- **wallet:** add canister unlink dialog

- **station,upgrader:** support large station and upgrader wasm

- **wallet:** add canister ic settings dialog

- **wallet:** add canister install dialog

- **wallet:** add default json visualizer to unknown operations

- **station:** external canister input to accept opt policies and permissions by type

- **station:** large WASM support for external canisters


### ❤️  Thank You

- Jan Hrubes
- Kepler Vital
- Leon Tan
- mraszyk

## 0.0.2-alpha.9 (2024-08-27)


### 🚀 Features

- **wallet:** show global external canister permissions


### 🩹 Fixes

- **release:** compat file includes correct api version


### ❤️  Thank You

- Kepler Vital

## 0.0.2-alpha.8 (2024-08-26)


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

- **station:** add comments to batch transfers via CSV

- **station:** add frontend to manage cycle obtain startegies

- **station:** add fee, comment, from address to csv export

- **station:** optimized repository lookups


### 🩹 Fixes

- **station:** requests are visible to users with approval rights


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.7 (2024-07-29)


### 🚀 Features

- **wallet:** remove unsafe-eval csp by bumping the agent

- **wallet:** add advanced and automated mode to upgrades

- **wallet:** prompt the user to update if new version available


### ❤️  Thank You

- Kepler Vital

## 0.0.2-alpha.6 (2024-07-04)


### 🚀 Features

- **station:** disaster recovery MVP

- **wallet:** Always lowercase ICP Ledger addresses

- **upgrader:** disaster recovery flow reuses the same upgrader canister

- **upgrader:** disaster recovery recreates accounts


### ❤️  Thank You

- Kepler Vital
- Max
- olaszakos

## 0.0.2-alpha.5 (2024-06-24)


### 🚀 Features

- **station:** introduce ChangeManagedCanister request type

- **station:** add CreateManagedCanister request type

- **station:** add canister_status endpoint

- **control-panel:** add artifact api

- **control-panel:** add registry api interface and repository

- **station:** add CallExternalCanister request type

- **control-panel:** add registry api implementation

- **control-panel:** add find next module version

- **station:** optional fallback controller


### 🩹 Fixes

- **wallet:** permission page shows individual resource section


### ❤️  Thank You

- Kepler Vital
- Max
- mraszyk

## 0.0.2-alpha.4 (2024-05-29)


### 🚀 Features

- **wallet:** add versioned path to wallet ui

- **wallet:** assure api compatibility check is resilient to missing file


### ❤️  Thank You

- Jan Wendling
- Kepler Vital
- mraszyk

## 0.0.2-alpha.3 (2024-05-17)

### 🚀 Features

- add user station labels

### 🔥 Performance

- reduce canister running costs

### ❤️ Thank You

- Kepler Vital
- mraszyk
- olaszakos

## 0.0.2-alpha.2 (2024-05-13)

### 🚀 Features

- show manage system info permission

### ❤️ Thank You

- Kepler Vital

## 0.0.2-alpha.1 (2024-05-13)

### 🚀 Features

- enable users to update the station name

### ❤️ Thank You

- Kepler Vital

## 0.0.2-alpha.0 (2024-05-12)

### 🚀 Features

- show deploy wallet quota exceeded error screen

- let user add station name

- show acceptance rules for proposal

- add notifications for the requester for failed/rejected request

- add core logger using pino

- support login with internet identity

- add control-panel canister integrated with the wallet ui

- added wallet management user interface

### 🩹 Fixes

- safari can open ii window

### ❤️ Thank You

- Kepler Vital
- olaszakos
