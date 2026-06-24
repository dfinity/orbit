#!/usr/bin/env bash
#
# Build the Orbit CLI from source and run `orbit-cli audit`.
#
# Why build from source instead of the global `orbit-cli`: the `audit`
# subcommand is recent, and a globally-installed orbit-cli is almost certainly
# older and doesn't have it. Building here keeps us on the checked-out version.
#
# Usage:
#   run-audit.sh --station <ID> [--identity <NAME>] [--identity-source icp] \
#                [--network ic] [--output <PATH>] ...
#
# Everything you pass is forwarded verbatim to `orbit-cli audit`, so any flag
# the audit accepts works here. With no args it prints the audit's help.
set -euo pipefail

REPO="$(git -C "$(dirname "${BASH_SOURCE[0]}")" rev-parse --show-toplevel)"
CLI_JS="$REPO/cli/dist/cli.js"
GEN="$REPO/cli/dist/audit/generated/station.did.js"

# 1. Install workspace deps once. (If the build below fails on a checkout that
#    already has node_modules, the deps are likely stale — run `pnpm install`
#    in the repo by hand and re-run.)
if [ ! -d "$REPO/node_modules" ]; then
  (cd "$REPO" && pnpm install)
fi

# 2. Build the CLI (fast; just tsc + a copy step).
(cd "$REPO" && pnpm --filter orbit-cli build) >/dev/null

# 3. Repair the ESM/CJS mismatch in the generated IDL.
#    The build does `cp src/audit/generated/station.did.js dist/...`, but that
#    file is an ES module (`export const idlFactory`) while the compiled CLI is
#    CommonJS — so `require()` throws `Unexpected token 'export'`. We rewrite the
#    dist copy to CommonJS. This runs after every build because the copy step
#    re-introduces the ESM file each time.
#
#    The proper fix is to emit/copy this file as CommonJS in the CLI's build
#    script (cli/package.json) so this guard becomes unnecessary.
#
#    Done in Node (already a hard requirement here) so the script needs no extra
#    runtime like perl/sed. Idempotent: a no-op once the file is already CJS.
node -e '
  const fs = require("fs");
  const p = process.argv[1];
  let s = fs.readFileSync(p, "utf8");
  if (s.includes("export const ")) {
    s = s.replace(/^export const /gm, "const ") + "\nmodule.exports = { idlFactory, init };\n";
    fs.writeFileSync(p, s);
  }
' "$GEN"

# 4. Run the audit, forwarding all arguments (default to --help if none given,
#    since `audit` requires --station and would otherwise error).
if [ "$#" -eq 0 ]; then set -- --help; fi
exec node "$CLI_JS" audit "$@"
