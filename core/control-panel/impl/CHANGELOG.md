## 0.3.0 (2025-03-03)


### 🚀 Features

- **control-panel:** public beta ([#519](https://github.com/dfinity/orbit/pull/519))

- **marketing:** marketing site home page ([#500](https://github.com/dfinity/orbit/pull/500))

- **docs:** initial docs portal ([#493](https://github.com/dfinity/orbit/pull/493))


### 🧱 Updated Dependencies

- Updated control-panel-api to 0.2.0


### ❤️  Thank You

- Jan Hrubes @jedna
- Kepler Vital
- olaszakos

## 0.2.0 (2025-02-26)


### 🚀 Features

- **station:** external canister snapshots ([#429](https://github.com/dfinity/orbit/pull/429))

- **station:** monitor external canisters ([#416](https://github.com/dfinity/orbit/pull/416))

- **station:** enable station top up from the cycles ledger balance ([#472](https://github.com/dfinity/orbit/pull/472))

- **control-panel:** reduce initial station and upgrader cycles to 1T ([#485](https://github.com/dfinity/orbit/pull/485))

- **control-panel:** user can deploy 2 stations per day ([#487](https://github.com/dfinity/orbit/pull/487))

- **docs:** docs portal ([#486](https://github.com/dfinity/orbit/pull/486))

- **control-panel:** rate limiting to at most 100 stations per day ([#489](https://github.com/dfinity/orbit/pull/489))

- **station:** updated permission management ux ([#491](https://github.com/dfinity/orbit/pull/491))

- **apps:** init marketing project ([#498](https://github.com/dfinity/orbit/pull/498))

- **wallet:** updated branding ([#497](https://github.com/dfinity/orbit/pull/497))


### 🩹 Fixes

- **wallet:** update lockfile ([#459](https://github.com/dfinity/orbit/pull/459))

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

## 0.1.0 (2024-11-27)


### 🚀 Features

- ⚠️  **station:** multi chain support ([#374](https://github.com/dfinity/orbit/pull/374))


### 🩹 Fixes

- **control-panel:** fix initial station deploy ([#441](https://github.com/dfinity/orbit/pull/441))


### ⚠️  Breaking Changes

- ⚠️  **station:** multi chain support ([#374](https://github.com/dfinity/orbit/pull/374))

### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.1.0
- Updated control-panel-api to 0.1.0
- Updated station-api to 0.1.0


### ❤️  Thank You

- Kepler Vital
- olaszakos

## 0.0.2-alpha.8 (2024-11-22)


### 🚀 Features

- **dfx-orbit:** Review list pagination ([#403](https://github.com/dfinity/orbit/pull/403))

- **wallet:** use didc for candid parsing ([#402](https://github.com/dfinity/orbit/pull/402))

- **station:** add expiration dt during request creation ([#424](https://github.com/dfinity/orbit/pull/424))

- **enable:** wallet ui accepts wasm chunk store from registry ([#433](https://github.com/dfinity/orbit/pull/433))


### 🧱 Updated Dependencies

- Updated station-api to 0.0.2-alpha.8


### ❤️  Thank You

- Kepler Vital
- Leon Tan

## 0.0.2-alpha.7 (2024-10-22)


### 🚀 Features

- **control-panel:** support deploying large station WASM ([#364](https://github.com/dfinity/orbit/pull/364))

- **control-panel:** support large WASM in registry ([#365](https://github.com/dfinity/orbit/pull/365))

- **dfx-orbit:** dfx-orbit version 0.5.0 ([#370](https://github.com/dfinity/orbit/pull/370))

- **control-panel:** allow deploying station to subnet of choice ([#372](https://github.com/dfinity/orbit/pull/372))

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **station:** allow external canister creation on subnet of choice ([#383](https://github.com/dfinity/orbit/pull/383))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))


### 🩹 Fixes

- **release:** workaround nx bug in release script ([#375](https://github.com/dfinity/orbit/pull/375))


### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.0.2-alpha.6
- Updated control-panel-api to 0.0.2-alpha.4
- Updated station-api to 0.0.2-alpha.7


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- mraszyk @mraszyk

## 0.0.2-alpha.6 (2024-10-02)


### 🩹 Fixes

- **http:** improve route matching to fix mac subdomain resolution issue


### ❤️  Thank You

- Jan Hrubes
- Kepler Vital
- Leon Tan
- mraszyk

## 0.0.2-alpha.5 (2024-08-26)


### 🚀 Features

- **station:** mint cycles to top up station

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

- **control-panel:** add registry schema to the control-panel

- **control-panel:** add artifact api

- **control-panel:** add registry api interface and repository

- **station:** add CallExternalCanister request type

- **control-panel:** add registry api implementation

- **control-panel:** add find next module version

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

This was a version bump only for control-panel to align it with other projects, there were no code changes.

## 0.0.2-alpha.0 (2024-05-12)


### 🚀 Features

- let user add station name

- add metrics dapp label to stations

- add control panel option to deploy wallet wasms

- initial metrics endpoint for control-panel

- add more metrics to control-panel

- rate limit user registration


### ❤️  Thank You

- Kepler Vital