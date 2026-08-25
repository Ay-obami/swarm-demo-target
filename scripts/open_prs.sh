#!/usr/bin/env bash
set -euo pipefail
cd /home/ayobami/swarm-demo-target

open_pr() { # open_pr <n> <branch> <title> <body>
  gh pr create --repo Ay-obami/swarm-demo-target \
    --base main --head "$2" --title "$3" --body "$4" 2>&1 | tail -1
}
echo "PR-101: $(open_pr 101 pr/101-discount-stack "Discount stacking ignores the cap" "Stacked pct+fixed discounts blow past discount_cap_cents. Acceptance tests pin the policy.")"
echo "PR-102: $(open_pr 102 pr/102-fx-rounding "FX conversion snaps to whole units" "convert() truncates cents after scaling; exact cent preservation is required.")"
echo "PR-103: $(open_pr 103 pr/103-oversell-boundary "Batch reservation can oversell" "reserve() checks total stock instead of remaining availability per grant.")"
