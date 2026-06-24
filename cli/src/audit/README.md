# `orbit-cli audit`

Read-only sanity checks against an Orbit station's configuration. The command pulls live state via the station's `list_*` query methods, runs a set of static checks against the configuration, and prints a severity-sorted report. Nothing is mutated — the audit is safe to run at any time, against any station the caller has read access to.

## Usage

```bash
orbit-cli audit --station <CANISTER_ID> [--network <ic|local>] [--identity <NAME>] [--output <PATH>]
```

### Options

| Flag                          | Default      | Purpose                                                                                                                                           |
| ----------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --station <CANISTER_ID>` | **required** | The station canister id to audit.                                                                                                                 |
| `-n, --network <ic\|local>`   | `ic`         | The network the station lives on.                                                                                                                 |
| `-i, --identity <NAME>`       | `default`    | The dfx identity used to call the station. Must have read access to the station's `list_*` query methods (admin-tier users have this by default). |
| `-o, --output <PATH>`         | _(stdout)_   | Write the report to a file instead of stdout. Exit code is still set based on findings.                                                           |
| `-h, --help`                  |              | Print help and exit.                                                                                                                              |

### Exit codes

| Code | Meaning                                |
| ---- | -------------------------------------- |
| `0`  | No findings.                           |
| `1`  | At least one `WARNING`, no `BLOCKER`s. |
| `2`  | At least one `BLOCKER`.                |

Useful for CI integration — `set -e` pipelines fail naturally when a blocker shows up.

### Examples

```bash
# Audit a mainnet station, print report to stdout.
orbit-cli audit --station rrkah-fqaaa-aaaaa-aaaaq-cai

# As an admin-tier identity, write the report to a file.
orbit-cli audit --station rrkah-fqaaa-aaaaa-aaaaq-cai \
                --identity admin-readonly \
                --output ./audit-report.txt

# Local development station.
orbit-cli audit --station <local-canister-id> --network local
```

## What it checks

The first release ships three checks. Each finding has a stable check id (so configs and CI rules can refer to them without breaking across versions) and a severity (`BLOCKER`, `WARNING`, or `INFO`).

### `quorum.empty-approver-set` — `BLOCKER`

Flags `Quorum` / `QuorumPercentage` rules whose `UserSpecifier` currently resolves to **zero active users**.

`RequestApprovalSummary::evaluate` clamps `min_approved` down to the number of possible approvers. When the approver set is empty the clamp drives the threshold to `0`, and `approved (0) >= min_approved (0)` evaluates the rule to `Approved` without any votes cast. The next matching request auto-approves silently.

The check walks every `RequestPolicy.rule`, including nested combinators (`AnyOf`, `AllOf`, `Not`) and `NamedRule` references. Cycles in `NamedRule` resolution are detected and skipped.

### `external-call.validation-equals-execution` — `BLOCKER`

Flags `CallExternalCanister` permissions and request policies whose `validation_method` and `execution_method` resolve to the same `(canister_id, method_name)` pair.

When a `CallExternalCanister` request is submitted, the station invokes the configured `validation_method` immediately to render and validate the argument blob — before the approval policy has completed. If that method is also the `execution_method`, its side effect runs at submission time, bypassing the approval gate. There is no benign reason for the pair to match.

The check sweeps both surfaces — request policies (`RequestSpecifier::CallExternalCanister`) and permissions (`Resource::ExternalCanister(Call(...))`).

### `asset.edit-policy-weaker-than-transfer` — `WARNING`

Warns when the **easiest** path to passing an `EditAsset` request requires fewer approvals than the **strictest** `Transfer` policy in the station.

An approved transfer's destination ledger is resolved from `asset.metadata["ledger_canister_id"]` at execute time. A successful `EditAsset` between approval and execution can therefore redirect routing. When `EditAsset` is gated more loosely than `Transfer`, an actor who can pass the lower bar can subvert the higher one.

The MVP compares the easiest `EditAsset` path against the strictest `Transfer` path station-wide. Per-asset / per-account scoping is a planned follow-up.

## Report format

The report is plain text, sorted by severity (`BLOCKER` → `WARNING` → `INFO`), with each finding showing the offending policy / asset / permission and a suggested fix. A `summary:` line ends the report with counts.

```
Orbit Station Audit Report
station: rrkah-fqaaa-aaaaa-aaaaq-cai
network: ic

==== BLOCKERS (1) ====

[quorum.empty-approver-set]
  policy abc123... (Transfer(Any)) — Quorum
  Quorum rule asks for 1 approval(s) but UserSpecifier::Id(1 user(s)) currently
  resolves to 0 active users. Next matching request will auto-approve.
  fix: Add eligible approvers to this specifier, or wrap in AnyOf with an
       admin-group fallback before the next matching request is submitted.

==== Positive confirmations ====
- 12 request policies loaded.
- 5 users loaded.
- ...

summary: 1 blocker, 0 warning, 0 info
```

## Implementation notes

- **No new runtime dependencies.** The audit uses the same `dfx canister call --output json` pattern as the existing `registry/` and `release/` subcommands. The result is parsed as Candid-in-JSON.
- **Hand-written types.** The audit imports a small subset of the station API as hand-written TypeScript in [types.ts](./types.ts) instead of generating bindings with `didc bind`. The surface is small enough that the extra build step isn't worth it for this MVP. If the audit grows, switching to generated bindings is a straightforward refactor.
- **Specifier resolution mirrors the station evaluator.** `resolveApprovers` and `minVotesForRule` in [resolver.ts](./resolver.ts) reimplement the relevant subset of `find_matching_users` and `RequestApprovalSummary::evaluate` from `core/station/impl/src/models/request_policy_rule.rs`. If the station evaluator changes, the resolver may need updating; the tests cover the static rule shapes used today.
- **Read-only by construction.** Only query methods are called. The audit cannot mutate state under any circumstance.

## Tests

```bash
pnpm --filter orbit-cli test
```

Unit tests cover each check across positive and negative cases, including combinator descent and cycle detection on `NamedRule` references. Fixtures are in [checks/fixtures.ts](./checks/fixtures.ts).

The audit's check logic is exercised by these tests; the dfx-call plumbing is exercised end-to-end against a real station (see the example invocations above).

## Extending

Each check is a function that takes the loaded station data and returns `Finding[]`:

```ts
export const myCheck = (
  policies: RequestPolicy[],
  users: User[],
  /* ... */
): Finding[] => {
  // ...
};
```

To add a new check:

1. Drop a new file in `checks/` returning `Finding[]`. Use a stable check id under a dot-namespace (`category.short-name`).
2. Spec it in `checks/<name>.spec.ts` with at least one positive and one negative case. Reuse the fixture builders in `checks/fixtures.ts`.
3. Wire it into `index.ts` alongside the existing checks.

A planned follow-up will move this to a registry pattern with a stable plugin interface so third-party checks can be loaded without modifying core.

## Roadmap

Possible follow-ups (not committed):

- `--format json` for CI / pipeline consumption.
- Config file with per-check enable/disable + severity overrides + threshold knobs.
- Plugin loader so out-of-tree checks can be added without forking.
- Additional checks: small-group Quorum specifiers (WARNING), missing admin-group fallback (INFO), dangling user/group references (WARNING), per-asset `EditAsset` vs `Transfer` scoping (refines the current station-wide check).
