# Orbit release process

A release has three phases: cut, publish, deploy. Cutting and publishing are one flow that ends in a GitHub release with the built artifact attached. Deploying is what puts that artifact on the live canisters. Everything up to and including a playground deploy runs from the Actions tab. Production is a command an operator runs.

Most of this used to be manual. It is now three workflows: **Cut release**, **Deploy frontend**, **Deploy backend**. Publishing was already automated and has not changed.

## Components

There is no single "Orbit release". There are independently versioned projects, and nx bumps each one on its own from the conventional commits that touched it. Six produce a deployable artifact. The rest are internal: shared crates that exist only to cascade a version bump, and the release tooling itself.

The current name is fixed. It is the git tag (`@orbit/{name}-v{version}`), the artifact name, and the build target, so renaming it is a real change, not a label. The proposed name is what we would rename it to in a separate PR. That rename has not been decided yet.

Prose here says station. The identifiers still say wallet for historical reasons: the `wallet-dapp` project, the `wallet` checkbox on the deploy form, `--app wallet`, the `app_wallet` key in `canister_ids.json`. Renaming those is a lot of work for a wording change. It splits the release tag series in two, so the scripts that resolve `latest` have to know both names, and every published release keeps the old one. It needs its own PR.

So backticked names below are the current ones, matching what you see in the repo and on the workflow forms. The table above maps each to its proposed name.

| Current name | Proposed name | Type | Role and where it deploys | Ships alone |
| --- | --- | --- | --- | --- |
| `wallet-dapp` | `station-frontend` | Frontend | The station UI users log into. Asset tarball to its asset canister (`5fu67`, app.orbit.global). | Yes |
| `marketing-dapp` | `landing-page` | Frontend | The public marketing site. Asset tarball to the marketing canister (orbit.global). | Yes |
| `docs-portal` | `docs-portal` | Frontend | The documentation site. Asset tarball to the docs canister (docs.orbit.global). | Yes |
| `station` | `station` | Backend | The backend, one instance deployed per org. Wasm to the control-panel registry, stations self-upgrade. | Yes |
| `upgrader` | `upgrader` | Backend | Per-station helper that performs safe upgrades, paired one-to-one with a station. Wasm to the registry. | Yes |
| `control-panel` | `control-panel` | Backend | The single global registry and directory. Deploys stations. Wasm, deployed as the control-panel canister. | Yes |
| `dfx-orbit` | `orbit-cli` | CLI | The CLI we ship to users. Git tag and GitHub release, users install it themselves. | Yes |
| `orbit-cli` (internal) | `orbit-release-cli` | Tooling | Drives the release: `release prepare`, `release publish`, `registry publish`. | Not released |
| `*-api`, `orbit-essentials` | (unchanged) | Crates | Shared Candid and types. No artifact, deploys nowhere. | Bumped automatically |

The usual points of confusion:

* The three frontends are separate projects with separate versions and separate asset canisters. They are not a single bundle.
* Apart from the three frontends, only the Control Panel is a singleton. There is one global instance. Station and Upgrader are multi-instance: the Control Panel deploys a fresh Station per org, and each Station comes paired with its own Upgrader. Station and Upgrader deploy and control each other, which is what makes a station upgrade safe.
* The `station-api` / `upgrader-api` / `control-panel-api` crates are the contract, just the Candid interface and shared types. The bare name (`station`) is the canister that runs; the `-api` crate compiles to no canister. Bumping an api crate cascades a bump into whatever depends on it, which is why one small change can move several version numbers at once.
* `dfx-orbit` and `orbit-cli` are not the same tool today. `dfx-orbit` is the CLI we ship to users. `orbit-cli` is our internal tool that drives the release (`release prepare`, `release publish`, `registry publish`). Only `dfx-orbit` is a release target. The rename settles this by moving the internal tool out of the way: it becomes `orbit-release-cli`, which frees `orbit-cli` for the CLI users install.

## Phase 1: cut a release

Actions tab, run the **Cut release** workflow. The form:

* **one checkbox per project**, named as in the table above. Tick nothing and it releases everything that changed since the last release, which nx works out from the commits. This is where you pick the whole batch or a subset.
* **version_specifier**: `auto` lets the conventional commits decide the bump. Override with `patch` / `minor` / `major`, or with `prepatch` / `preminor` / `premajor` / `prerelease` to move onto a pre-release version.
* **pre_release**: `none`, or `alpha` / `beta` / `rc` to cut something like `0.8.0-rc.0` instead of `0.8.0`. Only valid with `auto` or `prerelease`.
* **dry_run**: computes the versions and changelogs and opens nothing, so you can preview.

It runs `orbit-cli release prepare`, which bumps the versions, writes the changelogs, and updates `.release.json` in one commit, then opens a PR whose body lists exactly what is being released. Review the versions and merge it. If nothing changed, it says so and opens no PR.

Expect one snag on that PR. It is opened by `github-actions` using `GITHUB_TOKEN`, and GitHub fires no workflow events for anything that token does, so the required **External PR Ruleset** check never starts and sits on "Expected". Close the PR and reopen it: the reopen comes from you rather than from the bot, the check runs, and your approval survives. The real fix is to open the PR with a GitHub App installation token instead, which does fire events.

## Phase 2: publish (automatic)

Merging the release PR touches `.release.json`, which fires the existing **Release** workflow (`release.yaml`). It builds each artifact, creates the `@orbit/<project>-v<version>` tag, and creates the GitHub release with the artifact attached. You do not click anything.

This stops at the GitHub release. Nothing is on a live canister yet. That is phase 3.

## Phase 3: deploy

Playground and production are deployed differently on purpose.

**Playground is a workflow.** **Deploy frontend** and **Deploy backend** both work the same way: tick what you want and run it. Neither asks for approval, because no user sees playground.

**Production is a command you run.** The key that signs a production deploy is not a repo secret, so there is no button for it. You run one script from a machine that can read the key. A production frontend sync reaches every user the moment it finishes, and a control-panel upgrade reaches every station, so the key that does either should not sit where a merged pull request can read it.

Both halves run the same two scripts, `scripts/deploy-app` and `scripts/deploy-backend`. The workflows are wrappers around them, so CI and a laptop do the same thing.

### What to deploy first

Deploy the station frontend before publishing a station release to the registry.

The frontend serves a UI version matching the API version of the station it is talking to, from the `compat.json` it ships with. A station on an API version the deployed `compat.json` has never heard of counts as incompatible, and the user is dropped on the unversioned path, which is the older UI. Publishing the station wasm first opens that window: an admin upgrades, then loads a UI that cannot pair with them. Deploying the frontend first closes it. Stations still on the previous version keep being redirected to their matching bundle, which is still there because `deploy-app` syncs with `--no-delete`.

### Frontends to playground

Actions tab, run **Deploy frontend**. Tick the station frontend, which the form still calls **wallet**, and run it. It builds for playground and uploads to `bxkhk-6yaaa-aaaal-ai6va-cai`. Go test at https://playground.orbitwallet.io.

Only the station frontend has a playground canister. `marketing-dapp` and `docs-portal` exist on production only, so ticking them here fails on purpose, with a message telling you to deploy them to production instead. All three backend targets do work on playground. Publishing to the registry needs only the control-panel and the wasm chunk store, and both exist there.

Leave **promote_to_production** unchecked. It is off by default, and the `production` environment holds no key, so ticking it just fails the second job.

### Backends to playground

Actions tab, run **Deploy backend**. Tick station, upgrader, or control-panel and run it. Same as above: leave promote unchecked. The two backend types deploy differently:

* **station / upgrader**: `orbit-cli registry publish` loads the wasm into the control-panel registry. This only makes the version available. Each production station still upgrades itself, or its upgrader, only when that station's own admins submit and approve the request (finance being 2-of-N). Publishing station also publishes upgrader, its dependency.
* **control-panel**: a direct `dfx canister install --mode upgrade` of the one canister. There is no registry and no per-station buffer, so nothing stands between this and every station. Handle with care.

### Production

Run it from a machine that can read the signing key. `--op` takes a 1Password secret reference and reads the key into a temp file that is deleted when the script exits, so nothing is written down by hand. `--pem` takes a path if you already have the key on disk.

The scripts want Node at the version in `.nvmrc` and pnpm 9; a newer Node fails the engine check before anything runs. `deploy-app` needs docker and icx-asset, `deploy-backend` needs dfx and `orbit-cli` on your PATH. `pnpm install` builds it, but the global link only lands if `pnpm setup` has been run on that machine, so without that you get "orbit-cli not found" from an install that reported success.

Frontends. This deploys the released tarball, not a local build:

```
./scripts/deploy-app --app wallet --env production --tag latest --op "op://<vault>/<item>/identity.pem"
```

Backends. One target per run, and `station` publishes `upgrader` with it:

```
./scripts/deploy-backend --target station --network production --op "op://<vault>/<item>/identity.pem"
```

Both resolve to the newest stable release and skip pre-releases, so an rc sitting at the top of the list will not be picked up by accident. To deploy one on purpose, pass the tag: `deploy-app --tag @orbit/wallet-dapp-v0.8.0-rc.0`.

Both print what they are about to do and wait for you to confirm, and both check the artifact against the checksum published beside it.

`deploy-backend` also stops if your checkout is not the release you are deploying. The registry takes the version label from your working tree instead of from the artifact, so a stale checkout would publish the right wasm under the wrong version. Check out the tag it names and run it again.

Ask the release administrators for the vault reference. Whoever holds it can deploy production, and nobody else can.

### After a station release

Publishing puts the version in the registry and stops there. Each station upgrades when its own admins raise and approve the request.

An upgrade started from the station UI always takes a backup snapshot first, so there is a rollback point whether or not anyone thought about it. Using it means a `SystemRestore` request, which the UI can display but not create, so a rollback is an API call today.

Reinstalling an older station wasm works only while the stable memory version is unchanged. Across a change the station refuses the downgrade and traps in `post_upgrade`, which aborts the install and leaves the old code running. That is a safe refusal, not a broken canister, but it means the snapshot is the only way back.

## Remaining setup

Playground is wired up. There is a playground-only deploy identity, the `playground` environment holds its credentials, and that environment only accepts deployments from `main`. The identity has no production access at all, which is the property that stops a merged pull request from reaching production. Do not replace it with the identity that deploys production today: that one controls the production canisters too, and using it here would cancel out the reason production is deployed by hand.

What is left:

* Create the `production` environment and leave it without a key. An empty environment is what keeps the production job inert. Moving production deploys into CI would mean adding its credentials, required reviewers and the same branch restriction first, and accepting that CI can then reach production.
* Stand up a test station that stays up, so someone can run a real self-upgrade against a wasm in the playground registry before the same wasm goes to production.
