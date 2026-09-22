#!/usr/bin/env bash
#
# Build the Orbit CLI from source and run `orbit-release-cli audit`.
#
# Why build from source instead of the global `orbit-release-cli`: a global on PATH may
# be from an older checkout that predates the `audit` subcommand. Building here
# keeps us on the version in this repo, with no doubt about what's invoked.
#
# Usage:
#   run-audit.sh --station <ID> [--identity <NAME>] [--identity-source icp] \
#                [--network ic] [--output <PATH>] ...
#
# Everything you pass is forwarded verbatim to `orbit-release-cli audit`, so any flag
# the audit accepts works here. With no args it prints the audit's help.
set -euo pipefail

REPO="$(git -C "$(dirname "${BASH_SOURCE[0]}")" rev-parse --show-toplevel)"
CLI_JS="$REPO/cli/dist/cli.js"

# 1. Install workspace deps once. (If the build below fails on a checkout that
#    already has node_modules, the deps are likely stale — run `pnpm install`
#    in the repo by hand and re-run.)
if [ ! -d "$REPO/node_modules" ]; then
  (cd "$REPO" && pnpm install)
fi

# 2. Build the CLI (fast; just tsc + a copy step).
(cd "$REPO" && pnpm --filter orbit-release-cli build) >/dev/null

# 3. Run the audit, forwarding all arguments (default to --help if none given,
#    since `audit` requires --station and would otherwise error).
if [ "$#" -eq 0 ]; then set -- --help; fi
exec node "$CLI_JS" audit "$@"
