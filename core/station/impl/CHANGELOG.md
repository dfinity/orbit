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