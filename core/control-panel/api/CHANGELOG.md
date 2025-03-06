## 0.2.0 (2025-03-03)


### 🚀 Features

- **station:** external canister snapshots ([#429](https://github.com/dfinity/orbit/pull/429))

- **station:** monitor external canisters ([#416](https://github.com/dfinity/orbit/pull/416))

- **station:** enable station top up from the cycles ledger balance ([#472](https://github.com/dfinity/orbit/pull/472))

- **docs:** docs portal ([#486](https://github.com/dfinity/orbit/pull/486))

- **apps:** init marketing project ([#498](https://github.com/dfinity/orbit/pull/498))

- **wallet:** updated branding ([#497](https://github.com/dfinity/orbit/pull/497))

- **control-panel:** public beta ([#519](https://github.com/dfinity/orbit/pull/519))

- **marketing:** marketing site home page ([#500](https://github.com/dfinity/orbit/pull/500))

- **docs:** initial docs portal ([#493](https://github.com/dfinity/orbit/pull/493))


### 🩹 Fixes

- **wallet:** update lockfile ([#459](https://github.com/dfinity/orbit/pull/459))

- **ci:** install CMC in Orbit local deployment ([#465](https://github.com/dfinity/orbit/pull/465))

- **ci:** build and CI issues ([#467](https://github.com/dfinity/orbit/pull/467))

- **station:** docker build ([#492](https://github.com/dfinity/orbit/pull/492))

- **station:** security issue 42 ([#505](https://github.com/dfinity/orbit/pull/505))

- **wallet:** security issue 36, 37, 38, 39, 40, 41 ([#503](https://github.com/dfinity/orbit/pull/503))

- **station:** security issue 19, 54 ([#509](https://github.com/dfinity/orbit/pull/509))

- **station:** security issue 45 ([#510](https://github.com/dfinity/orbit/pull/510))

- **ci:** update docker setup in release script ([#517](https://github.com/dfinity/orbit/pull/517))


### ❤️  Thank You

- Jan Hrubes @jedna
- Kepler Vital
- mraszyk @mraszyk
- olaszakos

## 0.1.0 (2024-11-27)


### 🚀 Features

- **dfx-orbit:** Review list pagination ([#403](https://github.com/dfinity/orbit/pull/403))

- **wallet:** use didc for candid parsing ([#402](https://github.com/dfinity/orbit/pull/402))

- **station:** add expiration dt during request creation ([#424](https://github.com/dfinity/orbit/pull/424))

- ⚠️  **station:** multi chain support ([#374](https://github.com/dfinity/orbit/pull/374))


### 🩹 Fixes

- **control-panel:** fix initial station deploy ([#441](https://github.com/dfinity/orbit/pull/441))


### ⚠️  Breaking Changes

- ⚠️  **station:** multi chain support ([#374](https://github.com/dfinity/orbit/pull/374))

### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.1.0


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.4 (2024-10-22)


### 🚀 Features

- **station:** disaster recovery MVP ([#271](https://github.com/dfinity/orbit/pull/271))

- **cli:** helper cli commands to interact with the control-panel registry ([#289](https://github.com/dfinity/orbit/pull/289))

- **wallet:** Always lowercase ICP Ledger addresses ([#292](https://github.com/dfinity/orbit/pull/292))

- **ci:** Require scope in PR title ([#288](https://github.com/dfinity/orbit/pull/288))

- **wallet:** remove unsafe-eval csp by bumping the agent ([#299](https://github.com/dfinity/orbit/pull/299))

- **dfx-orbit:** Initial dfx-orbit tool ([#303](https://github.com/dfinity/orbit/pull/303))

- **dfx-orbit:** Version 0.2 ([#308](https://github.com/dfinity/orbit/pull/308))

- **station:** mint cycles to top up station ([#307](https://github.com/dfinity/orbit/pull/307))

- **station:** optimized repository lookups ([#319](https://github.com/dfinity/orbit/pull/319))

- **station:** notify failed station upgrade ([#331](https://github.com/dfinity/orbit/pull/331))

- **dfx-orbit:** dfx-orbitv0.3 ([#318](https://github.com/dfinity/orbit/pull/318))

- **dfx-orbit:** dfx-orbit version 0.4 ([#337](https://github.com/dfinity/orbit/pull/337))

- **station,upgrader:** support large station and upgrader wasm ([#353](https://github.com/dfinity/orbit/pull/353))

- **control-panel:** support deploying large station WASM ([#364](https://github.com/dfinity/orbit/pull/364))

- **control-panel:** support large WASM in registry ([#365](https://github.com/dfinity/orbit/pull/365))

- **dfx-orbit:** dfx-orbit version 0.5.0 ([#370](https://github.com/dfinity/orbit/pull/370))

- **control-panel:** allow deploying station to subnet of choice ([#372](https://github.com/dfinity/orbit/pull/372))

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **station:** allow external canister creation on subnet of choice ([#383](https://github.com/dfinity/orbit/pull/383))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))


### 🩹 Fixes

- **station:** requests are visible to users with approval rights ([#322](https://github.com/dfinity/orbit/pull/322))

- **release:** workaround nx bug in release script ([#375](https://github.com/dfinity/orbit/pull/375))


### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.0.2-alpha.6


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- Max @bitdivine
- mraszyk @mraszyk
- olaszakos

## 0.0.2-alpha.3 (2024-06-24)


### 🚀 Features

- **control-panel:** add artifact api

- **control-panel:** add registry api interface and repository

- **api:** Add the serde::Serialize trait to API types

- **control-panel:** add registry api implementation

- **control-panel:** add find next module version


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

This was a version bump only for control-panel-api to align it with other projects, there were no code changes.

## 0.0.2-alpha.0 (2024-05-12)


### 🚀 Features

- let user add station name

- add control panel option to deploy wallet wasms


### ❤️  Thank You

- Kepler Vital