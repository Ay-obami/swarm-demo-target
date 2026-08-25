# swarm-demo-target

Demo **target repository** for [Swarm CI](https://github.com/Ay-obami/Swarm_CI):
a small warehouse-domain Rust crate that intentionally carries open "PRs"
(seeded bug branches). Swarm CI's agents read the failing tests, plan a fix,
apply it in a sandbox, and must pass a **real `cargo test`** before the merge
gate opens.

## Layout

| Module | Responsibility |
|---|---|
| `src/currency.rs`  | basis-point money math, banker's rounding, FX conversion |
| `src/pricing.rs`   | line totals + order subtotal |
| `src/discounts.rs` | discount stacking policy (pct → fixed → cap) |
| `src/inventory.rs` | batch stock reservation with oversell protection |
| `src/report.rs`    | calendar helpers |

## Open PRs (seeded bug branches)

Each branch keeps its acceptance tests but breaks one function. The tests are
the contract — fixing the module turns the branch green.

| PR ref (`refs/pull/N/head`) | Branch | Bug |
|---|---|---|
| `refs/pull/101/head` | `pr/101-discount-stack` | discount cap ignored when stacking pct + fixed |
| `refs/pull/102/head` | `pr/102-fx-rounding` | FX conversion truncates to whole units after scaling |
| `refs/pull/103/head` | `pr/103-oversell-boundary` | batch reservation checks stock instead of remaining availability |

## Pointing Swarm CI at a PR

```jsonc
// POST /tasks
{
  "pr_id": "102",
  "title": "FX conversion loses cents",
  "bug_description": "convert() must apply rate_bps once and preserve cents; tests show 999c @12345bps should be 1233 but we get whole-unit snapping.",
  "pr_url": "https://github.com/<owner>/swarm-demo-target/pull/102"
}
```

Swarm CI derives the clone URL and fetches `refs/pull/102/head` — exactly how
real GitHub PR heads are addressed. A local path also works for offline demos:
`"repo_url": "/path/to/repo", "git_ref": "refs/pull/103/head"`.

Sample `bug_description` lines (paste into the dashboard):

- **101** — `stacked discounts blow past the cap: subtotal 100.00 with 15% + 3.00 fixed and cap 5.00 grants 18.00 instead of 5.00`
- **102** — `FX conversion snaps cents to whole units: convert(999, 12_345) returns 1200 instead of 1233`
- **103** — `batch reservation can oversell: it validates each request against total stock instead of remaining availability, so [3,3,5] on 10 units with 4 reserved succeeds`

## Rebuilding the PR refs locally

```bash
scripts/setup_prs.sh        # seeds branches + refs/pull/{101,102,103}/head
```
