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

- Updated orbit-essentials-macros to 0.1.0


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- olaszakos

## 0.0.2-alpha.6 (2024-10-22)


### 🚀 Features

- **control-panel:** support deploying large station WASM ([#364](https://github.com/dfinity/orbit/pull/364))

- **control-panel:** support large WASM in registry ([#365](https://github.com/dfinity/orbit/pull/365))

- **dfx-orbit:** dfx-orbit version 0.5.0 ([#370](https://github.com/dfinity/orbit/pull/370))

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **station:** allow external canister creation on subnet of choice ([#383](https://github.com/dfinity/orbit/pull/383))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))


### 🩹 Fixes

- **release:** workaround nx bug in release script ([#375](https://github.com/dfinity/orbit/pull/375))

- **orbit-essentials:** prefix wasm chunk hashes with slash in asset canister ([#391](https://github.com/dfinity/orbit/pull/391))


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- mraszyk @mraszyk

## 0.0.2-alpha.5 (2024-10-02)


### 🚀 Features

- **station,upgrader:** support large station and upgrader wasm

- **station:** external canister input to accept opt policies and permissions by type


### 🩹 Fixes

- **http:** improve route matching to fix mac subdomain resolution issue


### ❤️  Thank You

- Jan Hrubes
- Kepler Vital
- Leon Tan
- mraszyk

## 0.0.2-alpha.4 (2024-08-26)


### 🚀 Features

- **station:** optimized repository lookups


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- Max
- olaszakos

## 0.0.2-alpha.3 (2024-06-24)


### 🚀 Features

- **control-panel:** add registry schema to the control-panel

- **control-panel:** add registry api interface and repository

- **control-panel:** add registry api implementation


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

This was a version bump only for orbit-essentials to align it with other projects, there were no code changes.

## 0.0.2-alpha.0 (2024-05-12)

### 🚀 Features

- helpers for orbit canisters

- add metrics registry to canisters

- add data access traits

- add fn to optionally deserialize blobs

- mock ic_cdk for testing

- cumtom impl random for wasm rng

- format and parse timestamps to rfc3339

- add next_time to emulate time advancement in the same round

### ❤️ Thank You

- Kepler Vital
- mraszyk
- rikonor
