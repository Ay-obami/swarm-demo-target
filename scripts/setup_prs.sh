#!/usr/bin/env bash
# Seeds the demo PR branches: checks out main, applies the seeded bug for each
# PR, commits it on its branch, and publishes the pull-request style head ref
# (refs/pull/N/head) that Swarm CI fetches — same convention as GitHub.
#
# Usage: scripts/setup_prs.sh [remote]   # default remote: origin
#        REMOTE="." = local-only (no push), refs created via update-ref
set -euo pipefail
cd "$(dirname "$0")/.."

REMOTE="${1:-origin}"

seed() { # seed <n> <branch> <src-module-relpath> <title>
  local n="$1" branch="$2" module="$3" title="$4"
  git checkout -q -B "$branch" main
  cp "scripts/patches/$n/$module" "$module"
  git add "$module"
  git commit -q -m "PR-$n: $title

Seeded bug for the Swarm CI demo. Acceptance tests in tests/ pin the correct
behavior and currently FAIL; fixing this module must turn them green."
  if [[ "$REMOTE" == "." ]]; then
    git update-ref "refs/pull/$n/head" "refs/heads/$branch"
    echo "seeded $branch → refs/pull/$n/head ($(git rev-parse --short "refs/pull/$n/head")) [local]"
  else
    git push -q "$REMOTE" "refs/heads/$branch"
    git push -q "$REMOTE" "refs/heads/$branch:refs/pull/$n/head"
    echo "seeded + pushed $branch → $REMOTE as branch and refs/pull/$n/head"
  fi
}

seed 101 pr/101-discount-stack    src/discounts.rs "stacked discounts ignore the cap"
seed 102 pr/102-fx-rounding       src/currency.rs  "FX conversion snaps to whole units"
seed 103 pr/103-oversell-boundary src/inventory.rs "batch reservation can oversell"

git checkout -q main
echo "done. main is clean; branches are RED by design."
