# Orbit release process

A release has three phases: cut, publish, deploy. Cutting and publishing are one flow that ends in a GitHub release with the built artifact attached. Deploying is what puts that artifact on the live canisters. Everything up to and including a playground deploy runs from the Actions tab; production is a command an operator runs. This is the guide to both.

Most of this used to be manual. It is now three workflows: **Cut release**, **Deploy frontend**, **Deploy backend**. Publishing was already automated and has not changed.

## Components

There is no single "Orbit release". There are independently versioned projects, and nx bumps each one on its own from the conventional commits that touched it. Six produce a deployable artifact. The rest are internal crates that only exist to cascade a version bump.

The current name is fixed. It is the git tag (`@orbit/{name}-v{version}`), the artifact name, and the build target, so renaming it is a real change, not a label. The proposed name is the clearer one we would move to in a separate PR if we decide to.

| Current name | Proposed name | Type | Role and where it deploys | Ships alone |
| --- | --- | --- | --- | --- |
| `wallet-dapp` | `wallet-app` | Frontend | The wallet UI users log into. Asset tarball to the wallet canister (`5fu67`, app.orbit.global). | Yes |
| `marketing-dapp` | `landing-page` | Frontend | The public marketing site. Asset tarball to the marketing canister (orbit.global). | Yes |
| `docs-portal` | `docs-site` | Frontend | The documentation site. Asset tarball to the docs canister (docs.orbit.global). | Yes |
| `station` | `station` | Backend | Per-wallet backend, one deployed per org. Wasm to the control-panel registry, stations self-upgrade. | Yes |
| `upgrader` | `upgrader` | Backend | Per-station helper that performs safe upgrades, paired one-to-one with a station. Wasm to the registry. | Yes |
| `control-panel` | `control-panel` | Backend | The single global registry and directory. Deploys stations. Wasm, deployed as the control-panel canister. | Yes |
| `dfx-orbit` | `orbit-cli` | CLI | The CLI we ship to users. Git tag and GitHub release, users install it themselves. | Yes |
| `*-api`, `orbit-essentials` | (unchanged) | Crates | Shared Candid and types. No artifact, deploys nowhere. | Bumped automatically |

Things worth stating plainly, because they are the usual source of confusion:

* The three frontends are separate projects with separate versions and separate asset canisters. They are not one "wallet dapp" bundle.
* Only the Control Panel is a singleton. There is one global instance. Station and Upgrader are multi-instance: the Control Panel deploys a fresh Station per org, and each Station comes paired with its own Upgrader. Station and Upgrader deploy and control each other, which is what makes a station upgrade safe.
* The `station-api` / `upgrader-api` / `control-panel-api` crates are the contract, just the Candid interface and shared types. The bare name (`station`) is the canister that runs; the `-api` crate compiles to no canister. Bumping an api crate cascades a bump into whatever depends on it, which is why one small change can move several version numbers at once.
* `dfx-orbit` and `orbit-cli` are not the same tool. `dfx-orbit` is the CLI we ship to users. `orbit-cli` is our internal tool that drives the release (`release prepare`, `release publish`, `registry publish`). Only `dfx-orbit` is a release target.

## Phase 1: cut a release

Actions tab, run the **Cut release** workflow. The form:

* **one checkbox per project**: tick `wallet-dapp`, `station`, and so on. Tick nothing and it releases everything that changed since the last release, which nx works out from the commits. This is where you pick the whole batch or a subset.
* **version_specifier**: `auto` lets the conventional commits decide the bump. Override with `patch` / `minor` / `major` if you need to.
* **pre_release**: `none`, or `alpha` / `beta` / `rc` to cut something like `0.8.0-rc.0` instead of `0.8.0`. Only valid with `auto` or `prerelease`.
* **dry_run**: computes the versions and changelogs and opens nothing, so you can preview.

It runs `orbit-cli release prepare`, which bumps the versions, writes the changelogs, and updates `.release.json` in one commit, then opens a PR whose body lists exactly what is being released. Review the versions and merge it. If nothing changed, it says so and opens no PR.

## Phase 2: publish (automatic)

Merging the release PR touches `.release.json`, which fires the existing **Release** workflow (`release.yaml`). It builds each artifact, creates the `@orbit/<project>-v<version>` tag, and creates the GitHub release with the artifact attached. You do not click anything.

This stops at the GitHub release. Nothing is on a live canister yet. That is phase 3.

## Phase 3: deploy

Playground and production are deployed differently on purpose.

**Playground is a workflow.** Two of them, same shape: tick what you want, run it, done. No gate, because nothing user-facing is behind it.

**Production is a command you run.** The key that signs a production deploy is not a repo secret, so there is no button for it. You run one script from a machine that can read the key. The reason is blast radius: a production frontend sync is live to every user the moment it finishes, and a control-panel upgrade has no buffer either, so we would rather the credential not sit somewhere a merged pull request can reach it.

Both halves run the same two scripts, `scripts/deploy-app` and `scripts/deploy-backend`. The workflows are wrappers around them, so CI and a laptop do the same thing.

### Frontends to playground

Actions tab, run **Deploy frontend**. Tick the apps you want (wallet on by default, marketing and docs off) and run it. It builds each app for playground and uploads it. Go test at the playground URL.

Leave **promote_to_production** unchecked. It is off by default and only does anything if a production key has been put on the environment, which is not how this is set up.

### Backends to playground

Actions tab, run **Deploy backend**. Tick station, upgrader, or control-panel and run it. Same as above: leave promote unchecked. The two backend types deploy differently:

* **station / upgrader**: `orbit-cli registry publish` loads the wasm into the control-panel registry. This only makes the version available. Each production station still upgrades itself, or its upgrader, only when that station's own admins submit and approve the request (finance being 2-of-N). Publishing station also publishes upgrader, its dependency.
* **control-panel**: a direct `dfx canister install --mode upgrade` of the one canister. There is no registry and no per-wallet buffer, so nothing stands between this and every wallet. Handle with care.

### Production

Run it from a machine that can read the signing key. `--op` takes a 1Password secret reference and reads the key into a temp file that is deleted when the script exits, so nothing is written down by hand. `--pem` takes a path if you already have the key on disk.

Frontends. This deploys the released tarball, not a local build:

```
./scripts/deploy-app --app wallet --env production --tag latest --op "op://<vault>/<item>/identity.pem"
```

Backends. One target per run, and `station` publishes `upgrader` with it:

```
./scripts/deploy-backend --target station --network production --op "op://<vault>/<item>/identity.pem"
```

Both print what they are about to do and ask for confirmation. Both verify the artifact's published checksum before shipping it. `deploy-backend` also refuses to run if your checkout is not the release you are deploying, because the registry labels the entry with the version from your working tree rather than from the artifact, so a stale checkout would publish the right wasm under the wrong version. If it stops for that reason, check out the tag it names and run it again.

Ask the release administrators for the vault reference. Whoever holds it is who can deploy production, which is the point.

## One-time setup

The deploy workflows need this in place before they can run:

* Create the `playground` GitHub Environment and give it the playground deployment credential. The name the jobs look for is declared at the top of each deploy workflow.
* Restrict that environment's deployment branches to `main`, so a job on some other branch cannot claim the credential.
* Create the `production` environment but leave it without a key, which is what keeps the production job inert. If production deploys are ever moved into CI, that environment needs required reviewers and the same branch restriction first.
* Fix the playground `derivationOrigin` so Internet Identity login works there, ideally by making it come from an env var.
* Stand up a persistent test station so backend wasms published to the playground registry can be exercised by a real self-upgrade before production.
