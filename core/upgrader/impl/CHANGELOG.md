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

## 0.0.2-alpha.6 (2024-11-22)


### 🚀 Features

- **dfx-orbit:** Review list pagination ([#403](https://github.com/dfinity/orbit/pull/403))

- **wallet:** use didc for candid parsing ([#402](https://github.com/dfinity/orbit/pull/402))

- **station:** add expiration dt during request creation ([#424](https://github.com/dfinity/orbit/pull/424))


### 🩹 Fixes

- **upgrader:** relax disaster recovery query restrictions ([#428](https://github.com/dfinity/orbit/pull/428))


### 🧱 Updated Dependencies

- Updated station-api to 0.0.2-alpha.8


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.5 (2024-10-22)


### 🚀 Features

- **upgrader:** set target canister's compute allocation if failed to stop ([#369](https://github.com/dfinity/orbit/pull/369))

- **dfx-orbit:** dfx-orbit version 0.5.0 ([#370](https://github.com/dfinity/orbit/pull/370))

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **upgrader:** large WASM support in disaster recovery ([#382](https://github.com/dfinity/orbit/pull/382))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))


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

## 0.0.2-alpha.4 (2024-10-02)


### 🚀 Features

- **station:** notify failed station upgrade

- **station,upgrader:** support large station and upgrader wasm


### ❤️  Thank You

- Jan Hrubes
- Kepler Vital
- Leon Tan
- mraszyk
- olaszakos

## 0.0.2-alpha.3 (2024-07-04)


### 🚀 Features

- **station:** disaster recovery MVP

- **upgrader:** disaster recovery flow reuses the same upgrader canister


### ❤️  Thank You

- Jan Wendling
- Kepler Vital
- Max
- mraszyk
- olaszakos

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

This was a version bump only for upgrader to align it with other projects, there were no code changes.

## 0.0.2-alpha.0 (2024-05-12)

### 🚀 Features

- add trigger_upgrade function to enable the station canister to ask the upgrader to upgrade itself

- add upgrade function to upgrade the station canister

### ❤️ Thank You

- rikonor
- Kepler Vital
