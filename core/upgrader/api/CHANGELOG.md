## 0.2.0 (2025-06-02)


### 🚀 Features

- **station:** external canister snapshots ([#429](https://github.com/dfinity/orbit/pull/429))

- **station:** monitor external canisters ([#416](https://github.com/dfinity/orbit/pull/416))

- **station:** enable station top up from the cycles ledger balance ([#472](https://github.com/dfinity/orbit/pull/472))

- **docs:** docs portal ([#486](https://github.com/dfinity/orbit/pull/486))

- **apps:** init marketing project ([#498](https://github.com/dfinity/orbit/pull/498))

- **wallet:** updated branding ([#497](https://github.com/dfinity/orbit/pull/497))

- **marketing:** marketing site home page ([#500](https://github.com/dfinity/orbit/pull/500))

- **docs:** initial docs portal ([#493](https://github.com/dfinity/orbit/pull/493))

- **upgrader:** disaster recovery via canister snapshots ([#542](https://github.com/dfinity/orbit/pull/542))

- **station:** configurable station initialization ([#482](https://github.com/dfinity/orbit/pull/482))

- **upgrader:** new endpoint to list station snapshots ([#545](https://github.com/dfinity/orbit/pull/545))

- **station:** update Request::last_modification_timestamp on adding approval ([#576](https://github.com/dfinity/orbit/pull/576))

- **wallet:** DR UI ([#557](https://github.com/dfinity/orbit/pull/557))


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

- **CI:** use rustup 1.27.1 for docker build ([#532](https://github.com/dfinity/orbit/pull/532))

- **station:** bump ic-cdk to fix canister_status parsing ([#538](https://github.com/dfinity/orbit/pull/538))


### 🧱 Updated Dependencies

- Updated station-api to 0.5.0


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
- Updated station-api to 0.1.0


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.6 (2024-10-22)


### 🚀 Features

- **dfx-orbit:** dfx-orbit version 0.5.0 ([#370](https://github.com/dfinity/orbit/pull/370))

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **upgrader:** large WASM support in disaster recovery ([#382](https://github.com/dfinity/orbit/pull/382))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))


### 🩹 Fixes

- **release:** workaround nx bug in release script ([#375](https://github.com/dfinity/orbit/pull/375))


### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.0.2-alpha.6
- Updated station-api to 0.0.2-alpha.7


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- mraszyk @mraszyk

## 0.0.2-alpha.5 (2024-10-02)


### 🚀 Features

- **station,upgrader:** support large station and upgrader wasm


### ❤️  Thank You

- Jan Hrubes
- Kepler Vital
- Leon Tan
- mraszyk
- olaszakos

## 0.0.2-alpha.4 (2024-07-04)


### 🚀 Features

- **station:** disaster recovery MVP


### ❤️  Thank You

- Kepler Vital
- Max
- olaszakos

## 0.0.2-alpha.3 (2024-06-24)


### 🚀 Features

- **api:** Add the serde::Serialize trait to API types


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

This was a version bump only for upgrader-api to align it with other projects, there were no code changes.

## 0.0.2-alpha.0 (2024-05-12)

### 🚀 Features

- add trigger_upgrade function to enable the station canister to ask the upgrader to upgrade itself

- add upgrade function to upgrade the station canister

### ❤️ Thank You

- rikonor
- Kepler Vital
