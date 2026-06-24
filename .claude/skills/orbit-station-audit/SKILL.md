---
name: orbit-station-audit
description: >-
  Build and run the Orbit station configuration audit (`orbit-cli audit`)
  end-to-end against a live station, including the icp-cli Internet Identity
  setup that reliably trips people up. Use this whenever the user wants to
  audit, sanity-check, or security-review an Orbit station or wallet — e.g.
  "audit my Orbit wallet", "run orbit-cli audit on station <canister-id>",
  "check my Orbit station for misconfigured approval policies / empty quorums",
  or any time they pair an Orbit station canister id with auditing, approval
  quorum, or permission checks. Reach for this even if they don't say
  "orbit-cli" by name. It covers building the CLI from source (the globally
  installed orbit-cli is usually too old to have the `audit` subcommand),
  obtaining an identity the station recognizes as a member, and reading the
  report.
---

# Orbit station audit

`orbit-cli audit` runs read-only sanity checks against a live station and
prints a severity-sorted report — safe to run any time, since it only issues
`list_*` queries and mutates nothing. Flags, exit codes, and report format are
documented in [`cli/src/audit/README.md`](../../../cli/src/audit/README.md);
read that for reference. This skill is the *operational* guide — it gets you
from a fresh checkout to a successful run and front-loads the four things that
actually derail a first one.

## The four gotchas (read these first)

1. **Don't rely on the global `orbit-cli`.** Depending on when it was last
   built it either predates the `audit` subcommand (unknown-command error) or —
   after a `pnpm install`, which rebuilds and re-links it via the `prepare-cli`
   postinstall — carries the gotcha-2 bug and crashes. Build and run this repo's
   freshly-repaired `cli/dist/cli.js` instead; the helper script does exactly that.
2. **The built CLI may crash with `SyntaxError: Unexpected token 'export'`.**
   The build copies a generated IDL (`station.did.js`) that is an ES module
   into the CommonJS bundle, and `require()` can't parse it. The helper script
   below repairs this automatically; the manual fix is in *Troubleshooting*.
3. **Internet Identity principals are per-origin.** The same II anchor yields a
   *different* principal for every app origin. The audit must be called by a
   principal the station knows as a member, so the identity has to be derived
   from the origin the wallet itself uses. In production that origin is
   **`orbitwallet.io`** (pinned as `derivationOrigin` in
   [`apps/wallet/src/configs/init.config.ts`](../../../apps/wallet/src/configs/init.config.ts)),
   *not* the URL bar. Pass the bare host `orbitwallet.io` to `--app` (no
   `https://`, even though the config shows a scheme). Deriving from anything
   else (`app.orbit.global`, `oisy.com`, the default `cli.id.ai`) gives a
   stranger principal that the station will reject.
4. **`icp identity link web` waits for an Enter keypress** before it opens the
   browser. Running it non-interactively, pipe a newline in so it proceeds:
   `printf '\n' | icp identity link web ...`. The human still completes the
   actual sign-in in the browser.

## Prerequisites

- Node 20 and pnpm 9 (the repo pins `pnpm@9.12.2` via `packageManager`;
  Corepack normally handles this).
- `icp-cli` on `PATH` (`brew install icp-cli`, or
  <https://internetcomputer.org/install>) — needed for `--identity-source icp`.
- A working copy of the Orbit repo. If you aren't in one:
  `git clone https://github.com/dfinity/orbit && cd orbit`.

## Step 1 — Build the CLI from source

The helper script does this for you (install if needed → build → repair the
ESM/CJS quirk from gotcha 2), so prefer it:

```bash
.claude/skills/orbit-station-audit/scripts/run-audit.sh --help
```

Equivalent by hand, if you'd rather see each step:

```bash
pnpm install                       # once, if node_modules is missing
pnpm --filter orbit-cli build      # emits cli/dist/cli.js
node cli/dist/cli.js audit --help  # may hit gotcha 2 — see Troubleshooting
```

Run the freshly built `cli/dist/cli.js` directly — that's the copy the helper
script repairs. The global on `PATH` isn't repaired, even right after
`pnpm install`.

## Step 2 — Get an identity the station recognizes

The audit signs its calls with an identity that must be a station **member**
(admin-tier users have the required read access by default). For an
Internet-Identity-based wallet, that means an `icp-cli` identity derived from
`orbitwallet.io` (gotcha 3).

First, reuse an existing one if you already have it:

```bash
icp identity list
icp identity principal --identity <name>   # for each plausible candidate
```

If a candidate's principal already matches the principal shown in your wallet
UI under **Settings → Identity**, use that name and skip to Step 3. (Identities
bound to other origins — `oisy.com`, the default `cli.id.ai` — will *not* match.)

Otherwise create one bound to the wallet's origin:

```bash
printf '\n' | icp identity link web orbit-audit --app orbitwallet.io
icp identity principal --identity orbit-audit
```

A browser opens to `id.ai`. **Sign in with the same Internet Identity anchor
you use to log into Orbit.** The printed principal should match your wallet's
Settings → Identity. The *definitive* test, though, is Step 3: if the audit's
read calls are authorized, you are a member.

If the principal doesn't match (or Step 3 returns an authorization error),
delete it and re-link trying a different origin or anchor:

```bash
icp identity delete orbit-audit          # note: `delete`, not `remove`
printf '\n' | icp identity link web orbit-audit --app app.orbit.global
```

`orbitwallet.io` is correct for the current production wallet; `app.orbit.global`
is the fallback to try if the station predates that derivation origin.

## Step 3 — Run the audit

```bash
.claude/skills/orbit-station-audit/scripts/run-audit.sh \
  --station <STATION-CANISTER-ID> \
  --identity orbit-audit \
  --identity-source icp \
  --output ~/Downloads/orbit-audit-$(date +%F).txt
```

Every flag after the script name is forwarded verbatim to `orbit-cli audit`, so
swap in whichever identity matched in Step 2. `--network` defaults to `ic` (pass
`--network local` for a local replica), and dropping `--output` prints to
stdout. `--output` writes internal station metadata (principals, policy ids) —
keep it private. Exit code `2` means at least one blocker (full table in the
README).

## Troubleshooting

- **`SyntaxError: Unexpected token 'export'` running the CLI** — the generated
  IDL got copied into the CommonJS build as an ES module. The helper script
  repairs this after every build; if you're building by hand, copy the
  perl/printf one-liner from its step 3. (It's gitignored build output, so the
  patch is safe and disposable — a rebuild reintroduces the problem, which is
  why the script re-applies the fix each run.) The real fix belongs in
  [`cli/package.json`](../../../cli/package.json)'s build script.
- **`Unauthorized` / read calls rejected** — the calling principal isn't a
  station member. Re-check Step 2: right anchor, right `--app` origin, and the
  principal actually matches your wallet UI.
- **`icp identity store not found`** — `icp-cli` isn't installed, or you used
  `--identity-source icp` without it. Install icp-cli, or fall back to a dfx PEM
  with `--identity-source dfx` (use a *plaintext* identity — passphrase-protected
  PEMs aren't supported; see the README).
