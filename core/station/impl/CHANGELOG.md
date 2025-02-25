## 0.4.0 (2025-02-25)


### 🚀 Features

- **wallet:** searchable address book; pick from address book at transfer ([#470](https://github.com/dfinity/orbit/pull/470))

- **station:** optimize stable memory serialization ([#477](https://github.com/dfinity/orbit/pull/477))

- **station:** enable station top up from the cycles ledger balance ([#472](https://github.com/dfinity/orbit/pull/472))

- **station:** support named rules ([#483](https://github.com/dfinity/orbit/pull/483))

- **control-panel:** reduce initial station and upgrader cycles to 1T ([#485](https://github.com/dfinity/orbit/pull/485))

- **docs:** docs portal ([#486](https://github.com/dfinity/orbit/pull/486))

- **control-panel:** rate limiting to at most 100 stations per day ([#489](https://github.com/dfinity/orbit/pull/489))

- **wallet:** Approval Policy aka Named Rules UI ([#488](https://github.com/dfinity/orbit/pull/488))

- **apps:** init marketing project ([#498](https://github.com/dfinity/orbit/pull/498))

- **station:** update initial policies/permissions ([#499](https://github.com/dfinity/orbit/pull/499))

- **wallet:** updated branding ([#497](https://github.com/dfinity/orbit/pull/497))


### 🩹 Fixes

- **ci:** install CMC in Orbit local deployment ([#465](https://github.com/dfinity/orbit/pull/465))

- **ci:** build and CI issues ([#467](https://github.com/dfinity/orbit/pull/467))

- **station:** docker build ([#492](https://github.com/dfinity/orbit/pull/492))

- **station:** security issue 42 ([#505](https://github.com/dfinity/orbit/pull/505))

- **wallet:** security issue 36, 37, 38, 39, 40, 41 ([#503](https://github.com/dfinity/orbit/pull/503))

- **station:** security issue 19, 54 ([#509](https://github.com/dfinity/orbit/pull/509))

- **station:** security issue 45 ([#510](https://github.com/dfinity/orbit/pull/510))


### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.2.0
- Updated station-api to 0.4.0


### ❤️  Thank You

- Jan Hrubes @jedna
- Kepler Vital
- mraszyk @mraszyk
- olaszakos

## 0.3.0 (2024-12-17)


### 🚀 Features

- **station:** display station and upgrader cycle balance on Administration page ([#457](https://github.com/dfinity/orbit/pull/457))


### 🩹 Fixes

- **wallet:** update lockfile ([#459](https://github.com/dfinity/orbit/pull/459))


### 🧱 Updated Dependencies

- Updated station-api to 0.3.0


### ❤️  Thank You

- Jan Hrubes @jedna
- olaszakos

## 0.2.0 (2024-12-03)


### 🚀 Features

- **station:** external canister snapshots ([#429](https://github.com/dfinity/orbit/pull/429))

- **station:** monitor external canisters ([#416](https://github.com/dfinity/orbit/pull/416))


### 🩹 Fixes

- **wallet:** fix request export order ([#449](https://github.com/dfinity/orbit/pull/449))


### 🧱 Updated Dependencies

- Updated station-api to 0.2.0


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
- Updated upgrader-api to 0.1.0
- Updated station-api to 0.1.0


### ❤️  Thank You

- Kepler Vital
- olaszakos

## 0.0.2-alpha.10 (2024-11-22)


### 🩹 Fixes

- **wallet:** transfer requests page should only show transfers ([#438](https://github.com/dfinity/orbit/pull/438))


### ❤️  Thank You

- Kepler Vital

## 0.0.2-alpha.9 (2024-11-22)


### 🚀 Features

- **dfx-orbit:** Review list pagination ([#403](https://github.com/dfinity/orbit/pull/403))

- **wallet:** use didc for candid parsing ([#402](https://github.com/dfinity/orbit/pull/402))

- **station:** add metadata to external canisters ([#418](https://github.com/dfinity/orbit/pull/418))

- **station:** add expiration dt during request creation ([#424](https://github.com/dfinity/orbit/pull/424))

- **station:** requestor can cancel pending requests ([#426](https://github.com/dfinity/orbit/pull/426))


### 🧱 Updated Dependencies

- Updated station-api to 0.0.2-alpha.8


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.8 (2024-10-22)


### 🚀 Features

- **dfx-orbit:** dfx-orbit version 0.5.0 ([#370](https://github.com/dfinity/orbit/pull/370))

- **control-panel:** allow deploying station to subnet of choice ([#372](https://github.com/dfinity/orbit/pull/372))

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **station:** add canister execution method and validation pair edit variant ([#381](https://github.com/dfinity/orbit/pull/381))

- **station:** allow external canister creation on subnet of choice ([#383](https://github.com/dfinity/orbit/pull/383))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))

- **station:** init with default external canisters policies ([#393](https://github.com/dfinity/orbit/pull/393))

- **station:** enable native settings change of a non managed canister ([#396](https://github.com/dfinity/orbit/pull/396))


### 🩹 Fixes

- **release:** workaround nx bug in release script ([#375](https://github.com/dfinity/orbit/pull/375))


### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.0.2-alpha.6
- Updated upgrader-api to 0.0.2-alpha.6
- Updated station-api to 0.0.2-alpha.7


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- mraszyk @mraszyk

## 0.0.2-alpha.7 (2024-10-02)


### 🚀 Features

- **station:** notify failed station upgrade

- **station:** rate limiter for request creation

- **station:** add ability to cancel pending requests to EditUserOperation

- **station,upgrader:** support large station and upgrader wasm

- **station:** external canister input to accept opt policies and permissions by type

- **station:** large WASM support for external canisters


### 🩹 Fixes

- **station:** bound the maximum number of processing requests

- **http:** improve route matching to fix mac subdomain resolution issue


### ❤️  Thank You

- Jan Hrubes
- Kepler Vital
- Leon Tan
- mraszyk

## 0.0.2-alpha.6 (2024-08-26)


### 🚀 Features

- **wallet:** add disaster recovery config UI

- **station:** address book of external canisters

- **station:** add external canister management

- **station:** external canister method calls permissions

- **station:** integrate request policies and external canisters

- **station:** add external canister endpoints

- **station:** add cache layer to permission repository

- **station:** mint cycles to top up station

- **station:** use different types for input and get of external canister policies

- **station:** use own policies & permissions for canisters cycles fund

- **station:** add frontend to manage cycle obtain startegies

- **station:** add fee, comment, from address to csv export

- **station:** optimized repository lookups


### 🩹 Fixes

- **station:** requests are visible to users with approval rights


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.5 (2024-07-04)


### 🚀 Features

- **station:** disaster recovery MVP

- **upgrader:** disaster recovery flow reuses the same upgrader canister

- **upgrader:** disaster recovery recreates accounts


### ❤️  Thank You

- Kepler Vital
- Max
- olaszakos

## 0.0.2-alpha.4 (2024-06-24)


### 🚀 Features

- **station:** introduce ChangeManagedCanister request type

- **station:** add CreateManagedCanister request type

- **station:** add canister_status endpoint

- **station:** add CallExternalCanister request type

- **station:** optional fallback controller


### ❤️  Thank You

- Kepler Vital
- Max
- mraszyk

## 0.0.2-alpha.3 (2024-05-29)


### 🩹 Fixes

- canbench station build


### ❤️  Thank You

- Jan Wendling
- Kepler Vital
- mraszyk

## 0.0.2-alpha.2 (2024-05-17)


### 🚀 Features

- add user station labels


### 🩹 Fixes

- resource access control


### 🔥 Performance

- reduce canister running costs


### ❤️  Thank You

- Kepler Vital
- mraszyk
- olaszakos

## 0.0.2-alpha.1 (2024-05-13)


### 🚀 Features

- optimize station default memory allocation

- enable users to update the station name


### ❤️  Thank You

- Kepler Vital

## 0.0.2-alpha.0 (2024-05-12)


### 🚀 Features

- model props use next_time instead of time

- let user add station name

- show acceptance rules for proposal

- add notifications for the requester for failed/rejected request

- added asset management core

- self-upgrading mechanism

- add option to finalize proposal execution async

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