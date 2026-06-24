# `orbit-cli audit`

Read-only sanity checks against an Orbit station's configuration. The command pulls live state via the station's `list_*` query methods using `agent-js`, runs a set of static checks against the configuration, and prints a severity-sorted report. Nothing is mutated — the audit is safe to run at any time, against any station the caller has read access to.

> **Identity prerequisite.** The audit signs requests with an Ed25519 dfx identity loaded from `~/.config/dfx/identity/<name>/identity.pem`. The `dfx` CLI itself does not need to be on PATH at runtime, but the identity store layout follows dfx's convention. Passphrase-protected identities aren't supported; create a plaintext one for the audit if needed (`dfx identity new orbit-audit --storage-mode plaintext`).

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

Findings include policy, user, group, canister, and method identifiers. When using `--output`, write to a location that's appropriate for that level of detail — the report is internal station metadata, not something you'd want to commit to a public repo or sync to a shared drive without thinking about it first.

## Tests

```bash
pnpm --filter orbit-cli test
```

Unit tests cover each check across positive and negative cases, including combinator descent and cycle detection on `NamedRule` references. Fixtures are in [checks/fixtures.ts](./checks/fixtures.ts).

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
