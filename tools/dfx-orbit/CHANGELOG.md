## 0.11.0 (2025-09-02)


### 🚀 Features

- **marketing:** marketing site home page ([#500](https://github.com/dfinity/orbit/pull/500))

- **docs:** initial docs portal ([#493](https://github.com/dfinity/orbit/pull/493))

- **station:** configurable station initialization ([#482](https://github.com/dfinity/orbit/pull/482))

- **upgrader:** new endpoint to list station snapshots ([#545](https://github.com/dfinity/orbit/pull/545))

- **station:** update Request::last_modification_timestamp on adding approval ([#576](https://github.com/dfinity/orbit/pull/576))

- **wallet:** DR UI ([#557](https://github.com/dfinity/orbit/pull/557))

- **station:** request deduplication ([#589](https://github.com/dfinity/orbit/pull/589))

- **station:** Add tags to the request ([#590](https://github.com/dfinity/orbit/pull/590))

- **station,upgrader,control-panel:** enable overflow-checks in production canister builds ([#571](https://github.com/dfinity/orbit/pull/571))


### 🩹 Fixes

- **CI:** use rustup 1.27.1 for docker build ([#532](https://github.com/dfinity/orbit/pull/532))

- **station:** bump ic-cdk to fix canister_status parsing ([#538](https://github.com/dfinity/orbit/pull/538))


### 🧱 Updated Dependencies

- Updated station-api to 0.6.0


### ❤️  Thank You

- Jan Hrubes @jedna
- Kepler Vital
- mraszyk @mraszyk
- olaszakos
- tomer-dfinity

## 0.10.0 (2025-02-26)


### 🚀 Features

- **station:** enable station top up from the cycles ledger balance ([#472](https://github.com/dfinity/orbit/pull/472))

- **station:** support named rules ([#483](https://github.com/dfinity/orbit/pull/483))

- **docs:** docs portal ([#486](https://github.com/dfinity/orbit/pull/486))

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

## 0.9.0 (2024-12-03)


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

## 0.8.0 (2024-11-27)


### 🚀 Features

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
- olaszakos

## 0.7.0 (2024-11-22)


### 🚀 Features

- **dfx-orbit:** Enable controller management through Orbit ([#395](https://github.com/dfinity/orbit/pull/395))

- **dfx-orbit:** Review list pagination ([#403](https://github.com/dfinity/orbit/pull/403))

- **wallet:** use didc for candid parsing ([#402](https://github.com/dfinity/orbit/pull/402))

- **dfx-orbit:** Implement station file option ([#406](https://github.com/dfinity/orbit/pull/406))

- **station:** add expiration dt during request creation ([#424](https://github.com/dfinity/orbit/pull/424))


### 🩹 Fixes

- **dfx-orbit:** Fix argument parsing and evaluation around dfx-orbit ([#397](https://github.com/dfinity/orbit/pull/397))

- **dfx-orbit:** Check that there is a matching asset upload proposal ([#401](https://github.com/dfinity/orbit/pull/401))


### 🧱 Updated Dependencies

- Updated station-api to 0.0.2-alpha.8


### ❤️  Thank You

- Kepler Vital
- Leon Tan

## 0.6.0 (2024-10-22)


### 🚀 Features

- **dfx-orbit:** support installing canisters with large WASM ([#380](https://github.com/dfinity/orbit/pull/380))

- **station:** add canister execution method and validation pair edit variant ([#381](https://github.com/dfinity/orbit/pull/381))

- **wallet:** add external canister method call ui ([#385](https://github.com/dfinity/orbit/pull/385))


### 🩹 Fixes

- **orbit-essentials:** prefix wasm chunk hashes with slash in asset canister ([#391](https://github.com/dfinity/orbit/pull/391))


### 🧱 Updated Dependencies

- Updated orbit-essentials to 0.0.2-alpha.6
- Updated station-api to 0.0.2-alpha.7


### ❤️  Thank You

- Kepler Vital
- mraszyk @mraszyk

## 0.5.0 (2024-10-05)


### 🚀 Features

- **dfx-orbit:** Improved UX to request asset canister permissions
- **dfx-orbit:** Revoke asset canister permissions
- **dfx-orbit:** Commit / Cancel commit
- **dfx-orbit:** Verify commands to match each request


### 🩹 Fixes

- **release:** workaround nx bug in release script ([#375](https://github.com/dfinity/orbit/pull/375))
- **dfx-orbit:** Fix asset verification reproducibility issue when using machines with different CPU architecture

### ❤️  Thank You

- Kepler Vital
- Leon Tan

## 0.4.0 (2024-10-02)


### 🚀 Features

- **dfx-orbit:** dfx-orbitv0.3

- **dfx-orbit:** dfx-orbit version 0.4

- **station:** large WASM support for external canisters


### ❤️  Thank You

- Jan Hrubes
- Kepler Vital
- Leon Tan
- mraszyk

## 0.3.0 (2024-08-26)


### 🚀 Features

- **cli:** Init Orbit dfx extension placeholder

- **dfx-orbit:** Initial dfx-orbit tool

- **dfx-orbit:** Version 0.2

- **station:** use own policies & permissions for canisters cycles fund


### ❤️  Thank You

- Kepler Vital
- Leon Tan
- Max
