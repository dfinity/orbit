# Command Line Interface

The `dfx-orbit` command line is a tool for interacting with Orbit.
It is designed to work alongside `dfx` to allow a `dfx`-like workflow to manage canisters through Orbit.

## Getting started

### Installation

Build the tool:

Currently, there are two ways of installing `dfx-orbit`:

#### Standalone install

To get the most recent version of `dfx-orbit` without manually cloning the entire repository, run

```
cargo install -f --git https://github.com/dfinity/orbit.git --bin dfx-orbit
```

#### Install from the repository

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

> **NOTE**: This assumes that you already have a `dfx` setup working.
> If you need to set up a new identity, have a look at `dfx identity new`.

Connect your local dfx identity to your Orbit identity:

- Log in to Orbit.
- Navigate to Settings -> Users -> Edit a user -> Identities
- Add the principal provided by `dfx identity get-principal`

Tell the command line tool where to find the orbit station:

- Log in to Orbit.
- Navigate to station settings.
- Copy the wallet ID
- Store the station details locally. If your wallet is called `shiny` and is running locally, the command is:
  ```
  dfx-orbit station add shiny --station-id "$WALLET_ID" --network local --url https://orbitwallet.io
  ```
- Verify that the station is in your list of stations:
  ```
  dfx-orbit station list
  ```
- If you have multiple stations, set this as your default:
  ```
  dfx-orbit station use shiny
  ```
- Show the station details
  ```
  dfx-orbit station show
  ```
- In the orbit web UI, create a user with your local dfx principal:
  ```
  dfx identity get-principal
  ```
- Verify that you can get your profile on the Orbit station:
  ```
  dfx-orbit me
  ```

TODO: The Oisy canister ID is also called the wallet ID and the station ID. Consistent nomenclature that doesn't conflict with established terminology would be nice.

### Grant permission to make requests

You can check which permissions you have with:

```
dfx-orbit me | jq .Ok.privileges
```

Initially you are likely to have only permission to see your own profile:

```
[
  "Capabilities"
]
```

Without permission to make and view requests, you will not be able to do much. It is recommended to make a `Developer` group with the following permissions:

| Name in UI   | Privilege in `dfx-orbit me` | Name in error messages | Used for                         |
| ------------ | --------------------------- | ---------------------- | -------------------------------- |
| Request/List | `ListRequests`              | `Request(List)`        | `dfx-orbit review list`          |
| Request/Read | Not Shown                   | `Request(Read(Id))`    | `dfx-orbit review id REQUEST_ID` |

TODO: It would be nice to be able to link directly to a permission. E.g. this could open the permissions page and focus on one specific permission: https://orbitwallet.io/en/settings/user-groups/permissions#Request/List

## Make canister calls with Orbit

Instead of using `dfx canister call CANISTER METHOD ARGUMENTS` use `dfx-orbit request canister call CANISTER METHOD ARGUMENTS`.

For example, asset canisters have the methods `list_authorized` and `list_permitted`. You should be able to make these canister calls directly or via Orbit:

```
dfx canister call frontend list_authorized
dfx-orbit request canister call frontend list_authorized
```

## Control a canister with Orbit

### Grant Orbit control of the canister

Assume that you have a canister called `MY_CANISTER` in `dfx`. You may also refer to your canister by canister ID.

Check the current controllers of the canister:

```
dfx canister info MY_CANISTER --network MY_CANISTER_NETWORK
```

Add Orbit as a controller of the canister:

```
dfx canister update-settings --network NETWORK_NAME --set-controller ORBIT_PRINCIPAL MY_CANISTER
```

Verify that Orbit has been added as a controller:

```
dfx canister info MY_CANISTER --network MY_CANISTER_NETWORK
```

### Upgrade canisters

#### Request permission to make upgrade requests

This will allow you to propose upgrades to `MY_CANISTER`:

```
dfx-orbit request permission upgrade-canister MY_CANISTER
```

This will create an Orbit request. Once approved you will be able to propose canister upgrades.

> :warning: **The Orbit GUI does not currently show this proposal unless you enter the proposal URL directly, under /en/settings/requests?reqid=THE_ID**

#### Request a canister upgrade

Suppose that you have built a new Wasm and put a copy at `./MY-CANISTER.wasm.gz`. To upgrade your canister to the new Wasm:

```
dfx-orbit request canister install --mode upgrade --wasm ./MY-CANISTER.wasm.gz MY_CANISTER
```

### Upload assets to a canister

We will assume that Orbit is a controller of the asset canister. If not, please adapt the following commands by using `dfx canister call` instead of `dfx-orbit request canister call`.

#### Authorize the developer to upload assets

Note: Uploaded assets are not published. They are only prepared for release.

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

#### Authorize the orbit station to commit assets

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

#### Request an asset update

A developer may upload one or more directories of HTTP assets with:

```
dfx-orbit asset upload CANISTER_NAME SOME_DIR/ OTHER_DIR/
```

This will upload the assets to the asset canister and then request the orbit station to publish
the assets.

#### Verifying an asset update

After the request has been made, the reviewers can locally verify the request:

```
dfx-orbit asset check --then-approve CANISTER REQUEST_ID BATCH_ID SOME_DIR/ OTHER_DIR/
```

The exact command is printed in the output of `dfx-orbit asset upload` and must be distributed
from the proposer to the verifiers.

> The verifiers needs to have the same set of data as was used in the request.
> How the verifier accomplishes this is outside the scope of this document.
>
> - The verifier might either download a tarball from the requester and manually verify the content
> - The verifier might check out a git revision and check that the content matches
> - If there are build scripts used while generating the assets, care must be taken to make
>   the build step deterministic, such that verifiers can recreate the exact assets

Once the request has been approved, the changes will take effect.
