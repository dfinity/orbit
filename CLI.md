# Command Line Interface

The `dfx-orbit` command line is a tool for interacting with Orbit.  Currently it focuses on deploying canisters controlled by Orbit but the scope is expected to grow.

## Getting started

### Installation
Build the tool:
```
$ cargo build -p dfx-orbit
```

Verify that the tool works:
```
$ ./target/debug/dfx-orbit --version
dfx-orbit 0.1.0

$ ./target/debug/dfx-orbit --help
Command line tool for interacting with the Orbit digital asset manager on the ICP blockchain.

Usage: dfx-orbit <COMMAND>

Commands:
  station        Manages Orbit stations
...
```

Add `dfx-orbit` to your `PATH`.

### Connect to Orbit

Connect your local dfx identity to your Orbit identity:

* Log in to Orbit.
* Navigate to Settings -> Users -> Edit a user -> Identities
* Add the principal provided by `dfx identity get-principal`

Tell the command line tool where to find the orbit station:

* Log in to Orbit.
* Navigate to station settings.
* Copy the wallet ID
* Store the station details locally.  If your wallet is called `shiny` and is running locally, the command is:
  ```
  dfx-orbit station add --name shiny --canister-id "$WALLET_ID" --network local
  ```
* Verify that the station is in your list of stations:
  ```
  dfx-orbit station list
  ```
* If you have multiple stations, set this as your default:
  ```
  dfx-orbit station use --name shiny
  ```
* Verify that you can get your profile on the Orbit station:
  ```
  dfx-orbit me
  ```

## Control a canister with Orbit

### Grant Orbit control of the canister
Assume that you have a canister called `MY_CANISTER` in `dfx`.  You may also refer to your canister by canister ID.

Add Orbit as a controller while keeping existing controllers:
```
(
  orbit_station="$(dfx-orbit station show | jq -r .canister_id)"
  dfx canister update-settings --add-controller "$orbit_station" MY_CANISTER
)
```

### Register yourself as a developer for your canister
This will allow you to propose upgrades to `MY_CANISTER`:

```
dfx-orbit request permission canister change wasm --canister MY_CANISTER
```
This will create an Orbit request.  Once approved you will be able to propose canister upgrades.

> :warning: **The Orbit GUI does not currently show this proposal unless you enter the proposal URL directly, under /en/settings/requests?reqid=THE_ID**

### Upgrade a canister
Suppose that you have built a new Wasm and put a copy at `./MY-CANISTER.wasm.gz`.  To upgrade your canister to the new Wasm:
```
dfx-orbit request canister change wasm --canister MY_CANISTER --mode upgrade --wasm ./MY-CANISTER.wasm.gz
```