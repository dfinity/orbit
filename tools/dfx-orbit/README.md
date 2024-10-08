# Command Line Interface

The `dfx-orbit` command line is a tool for interacting with Orbit.
It is designed to work alongside `dfx` to allow a `dfx`-like workflow to manage canisters through Orbit.

## Getting started

### Prequisites

This guide assumes, that the user has setup and is acquainted with the following tools:

- A fairly recent rust toolchain. This tool is known to work on linux using rust `1.79.0`.
- A working `dfx` development setup.
- An internet identity and an Orbit account with the correct permissions.

### Installation

Currently, there are two ways of installing `dfx-orbit`:

#### Standalone install

To get the most recent version of `dfx-orbit` without manually cloning the entire repository, run

```
cargo install -f --git https://github.com/dfinity/orbit.git --bin dfx-orbit
```

#### Clone and install from the repository

This version is potentially more useful, if you want to make patches or use a specific branch.

```
git clone https://github.com/dfinity/orbit.git
cargo install -f --path tools/dfx-orbit/
```

Verify that the tool works:

```
$ dfx-orbit --version
dfx-orbit 0.1.0

$ dfx-orbit --help
Command line tool for interacting with the Orbit digital asset manager on the ICP blockchain.

Usage: dfx-orbit <COMMAND>

Commands:
  station        Manages Orbit stations
...
```

### Connect to Orbit

Connect your local dfx identity to your Orbit identity:

- Log in to Orbit.
- Navigate to Settings -> Users -> Edit a user -> Identities
- Add the principal provided by `dfx identity get-principal`

Tell the command line tool where to find the orbit station:

- Log in to Orbit.
- Navigate to station settings.
- Copy the wallet ID
- Store the station details locally.

  ```
  dfx-orbit station add [STATION_NAME] --station-id [STATION_ID] --network ic
  ```

- Verify that the station is in your list of stations:
  ```
  dfx-orbit station list
  ```
- If you have multiple stations, set this as your default:
  ```
  dfx-orbit station use [STATION_NAME]
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

#### Request a canister upgrade

Suppose that you have built a new Wasm. To upgrade your canister to the new Wasm:

```
dfx-orbit request canister install --mode upgrade [CANISTER_NAME] --wasm [WASM_PATH]
```

Then a verifier can verify this request, using:

```
dfx-orbit verify [REQUEST_ID] canister install --mode upgrade [CANISTER_NAME] --wasm [WASM_PATH]
```

### Upload assets to a canister

We will assume that Orbit is a controller of the asset canister.
If not, please transfer the control of the canister to the orbit station.

#### Authorize the developer to upload assets

Note: Uploaded assets are not published. They are only prepared for release.

```
dfx-orbit request asset permission [CANISTER_NAME] prepare
```

Similarly, you can validate a request using

```
dfx-orbit verify [REQUEST_ID] asset permission [CANISTER_NAME] prepare
```

In case you want to verify, whether you have the `Prepare` permission on the asset canister,
run:

```
dfx canister call frontend list_permitted '(record { permission = variant { Prepare } })'
```

and check whether your principal is among the ones listed.
You can optain your own principal via:

```
dfx identity get-principal
```

#### Request an asset update

A developer may upload one or more directories of HTTP assets with:

```
dfx-orbit request asset upload [CANISTER_NAME] --files [SOME_DIR]
```

This will upload the assets to the asset canister and then request the orbit station to publish
the assets.

#### Verifying an asset update

After the request has been made, the reviewers can locally verify the request:

```
dfx-orbit verify [REQUEST_ID ] asset upload CANISTER --batch-id [BATCH_ID] --files [SOME_DIR]
```

> The verifiers needs to have the same set of data as was used in the request.
> How the verifier accomplishes this is outside the scope of this document.
>
> - The verifier might either download a tarball from the requester and manually verify the content
> - The verifier might check out a git revision and check that the content matches
> - If there are build scripts used while generating the assets, care must be taken to make
>   the build step deterministic, such that verifiers can recreate the exact assets

Once the request has been approved, the changes will take effect.
