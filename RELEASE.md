# Orbit release process

A release has three phases: cut, publish, deploy. Cutting and publishing are one flow that ends in a GitHub release with the built artifact attached. Deploying is what puts that artifact on the live canisters. All of it runs from the Actions tab. This is the guide to what to click.

Most of this used to be manual, run from a laptop with the production key. It is now three workflows: **Cut release**, **Deploy frontend**, **Deploy backend**. Publishing was already automated and has not changed.

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

* **projects**: comma-separated project names, for example `wallet-dapp,station`. Empty releases everything that changed since the last release. This is where you pick the whole batch or a subset.
* **version_specifier**: `auto` lets the conventional commits decide the bump. Override with `patch` / `minor` / `major` if you need to.
* **pre_release**: `none`, or `alpha` / `beta` / `rc` for a pre-release.
* **dry_run**: computes the versions and changelogs and opens nothing, so you can preview.

It runs `orbit-cli release prepare`, which bumps the versions, writes the changelogs, and updates `.release.json` in one commit, then opens a PR whose body lists exactly what is being released. Review the versions and merge it. If nothing changed, it says so and opens no PR.

## Phase 2: publish (automatic)

Merging the release PR touches `.release.json`, which fires the existing **Release** workflow (`release.yaml`). It builds each artifact, creates the `@orbit/<project>-v<version>` tag, and creates the GitHub release with the artifact attached. You do not click anything.

This stops at the GitHub release. Nothing is on a live canister yet. That is phase 3.

## Phase 3: deploy

Two workflows, same shape. Pick a subset or the whole set, deploy to playground with no gate, then promote to production behind a single approval. One approval covers the whole selection.

### Deploy frontend

Actions tab, run **Deploy frontend**. Tick the apps you want (wallet on by default, marketing and docs off), and leave **promote_to_production** on unless you want a playground-only run.

* The **playground** job builds each app for playground and uploads it. Go test at the playground URL.
* The **production** job pauses for approval. Once approved, it uploads the released tarball to production.

Frontend config is baked in at build time, so playground gets its own build while production deploys the released production tarball.

### Deploy backend

Actions tab, run **Deploy backend**. Tick station, upgrader, or control-panel, and leave promote on unless it is a playground-only run. The two backend types deploy differently:

* **station / upgrader**: `orbit-cli registry publish` loads the wasm into the control-panel registry. This only makes the version available. Each production station still upgrades itself, or its upgrader, only when that station's own admins submit and approve the request (finance being 2-of-N). Publishing station also publishes upgrader, its dependency.
* **control-panel**: a direct `dfx canister install --mode upgrade` of the one canister. There is no registry and no per-wallet buffer, so the approval gate is the only thing between this and every wallet. Handle with care.

### The approval gate

The gate is native **GitHub Environments**. A `playground` environment that runs freely, and a `production` environment that requires a named approver. When a run reaches the production job, GitHub pauses it and shows a "review pending deployments" banner. A reviewer clicks approve and the job runs. If you are not on the reviewer list, the run stops at playground. Reviewers are set in Settings, Environments, production, Required reviewers.

## One-time setup

The deploy workflows need this in place before they can run:

* Create the `playground` and `production` GitHub Environments.
* Add the signing key as a secret on each: `DEPLOY_IDENTITY_PEM` for the frontend workflow, `BACKEND_IDENTITY_PEM` for the backend workflow, each scoped to the environment so a job only ever sees its own key. The backend identity must be both a registry admin on the control-panel and a controller of the control-panel canister.
* Add required reviewers to `production`. That is the approval gate.
* Fix the playground `derivationOrigin` so Internet Identity login works there, ideally by making it come from an env var.
* Stand up a persistent test station so backend wasms published to the playground registry can be exercised by a real self-upgrade before production.

One decision affects the frontend production job: whether the production key lives in the `production` environment as a secret, or stays off CI. In CI, production is a one-approval click. Off CI, production stays a local `scripts/deploy-app` run reading the key from 1Password, and the master key never touches GitHub. A reasonable start is production local, playground automated, and move production into the protected environment once the flow is trusted.

## Release cadence

We don't have a set release cadence today. I'd propose a bi-weekly one, every second Thursday. It would be best-effort, but it gives us a bit of urgency and a rhythm to work to. A recurring Google Calendar event can act as the soft reminder.
