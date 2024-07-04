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
  dfx-orbit station add shiny --station-id "$WALLET_ID" --network local --url https://orbitwallet.io
  ```
* Verify that the station is in your list of stations:
  ```
  dfx-orbit station list
  ```
* If you have multiple stations, set this as your default:
  ```
  dfx-orbit station use shiny
  ```
* Show the station details
  ```
  dfx-orbit station show
  ```
* In the orbit web UI, create a user with your local dfx principal:
  ```
  dfx identity get-principal
  ```
* Verify that you can get your profile on the Orbit station:
  ```
  dfx-orbit me
  ```

TODO: The Oisy canister ID is also called the wallet ID and the station ID.  Consistent nomenclature that doesn't conflict with established terminology would be nice.

## Control a canister with Orbit

### Grant Orbit control of the canister
Assume that you have a canister called `MY_CANISTER` in `dfx`.  You may also refer to your canister by canister ID.

Check the current controllers of the canister:
```
dfx canister info MY_CANISTER --network MY_CANISTER_NETWORK
```

Add Orbit as a controller of the canister:
```
dfx-orbit canister claim MY_CANISTER
```
Verify that Orbit has been added as a controller:
```
dfx canister info MY_CANISTER --network MY_CANISTER_NETWORK
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

## Upload assets to a canister
We will assume that Orbit is a controller of the asset canister.  If not, please adapt the following commands by using `dfx canister call` instead of `dfx-orbit request canister call`.

### Authorize the developer to upload assets
Note: Uploaded assets are not published.  They are only prepared for release.
```
developer_principal="$(dfx identity get-principal)"
dfx-orbit request canister call frontend grant_permission "
(
  record {
    permission = variant { Prepare };
    to_principal = principal \"$developer_principal\";
  },
)
"
```
When the request has been approved, check the list of principals permitted to prepare assets:
```
dfx canister call frontend list_permitted '(record { permission = variant { Prepare } })'
```

### Authorize the orbit station to commit assets
Note: Committing uploaded assets causes them to be published on the asset canister web site.
```
station_principal="$(dfx-orbit station show | jq -r .station_id)"
dfx-orbit request canister call frontend grant_permission "
(
  record {
    permission = variant { Commit };
    to_principal = principal \"$station_principal\";
  },
)
"
```
When the request has been approved, check the list of principals permitted to commit assets:
```
dfx canister call frontend list_permitted '(record { permission = variant { Commit } })'
```

### Upload assets
A developer may upload one or more directories of HTTP assets with:
```
dfx-orbit canister upload-http-assets --canister CANISTER_NAME --source SOME_DIR/ --source OTHER_DIR/
```
The developer may now request that the assets be published.  The command for this is printed at the end of the upload command.  Example:
```
...
Jul 03 09:36:42.148 INFO Computing evidence.
Proposed batch_id: 5
Assets have been uploaded.  For the changes to take effect, run:
dfx-orbit request canister call frontend commit_proposed_batch '(record { batch_id = 5 : nat; evidence = blob "\e3\b0\c4\42\98\fc\1c\14\9a\fb\f4\c8\99\6f\b9\24\27\ae\41\e4\64\9b\93\4c\a4\95\99\1b\78\52\b8\55" })'
```
Once the request has been approved, the changes will take effect.