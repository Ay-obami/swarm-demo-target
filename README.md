# swarm-demo-target

Demo **target repository** for [Phoenix CI](https://github.com/Ay-obami/Swarm_CI):
a warehouse-domain Rust crate that intentionally carries open "PRs"
(seeded bug branches). Phoenix CI's agents read the failing tests, plan a fix,
apply it in a sandbox, and must pass a **real `cargo test`** before the merge
gate opens.

## Layout

| Module | Responsibility |
|---|---|
| `src/currency.rs`  | basis-point money math, banker's rounding, FX conversion |
| `src/pricing.rs`   | line totals + order subtotal |
| `src/discounts.rs` | discount stacking policy (pct → fixed → cap) |
| `src/inventory.rs` | batch stock reservation with oversell protection |
| `src/report.rs`    | calendar quarters, leap years, fiscal-year offsets |
| `src/tax.rs`       | exclusive sales tax on the net subtotal |
| `src/shipping.rs`  | tiered shipping, free-shipping threshold, per-kg surcharge |

## Open PRs (20 seeded bugs)

Every branch carries its own red acceptance tests — the contract the agents
must turn green. **Copyable title + bug-description pairs for all 20 live in
[OPEN_PRS.md](OPEN_PRS.md)**; the machine-readable catalog is `prs.json`.

Branch naming: `pr/<label>-<slug>`; each is also published as the
pull-request style ref `refs/pull/<label>/head`, which is exactly what
Phoenix CI fetches when you paste a PR link.

Examples:

- `pr/102-fx-rounding` — FX conversion snaps cents to whole units
  (`convert(999, 12_345)` → 1200 instead of 1233)
- `pr/110-free-threshold-inverted` — free shipping requires weight ABOVE the
  threshold instead of at-or-below
- `pr/117-partial-grant` — batch reservation partially grants instead of
  failing on the first impossible request

## Pointing Phoenix CI at a PR

```jsonc
// POST /tasks   (or paste the link into the dashboard's "PR link" field)
{
  "pr_id": "1",
  "title": "FX conversion loses cents",
  "bug_description": "convert() must apply rate_bps once and preserve cents; convert(999, 12345) must be 1233 but returns 1200.",
  "pr_url": "https://github.com/Ay-obami/swarm-demo-target/pull/1"
}
```

Equivalent branch form (works for any pushed branch, no PR object needed):

```jsonc
{ "repo_url": "https://github.com/Ay-obami/swarm-demo-target.git",
  "git_ref": "pr/110-free-threshold-inverted", "bug_description": "…" }
```

A local filesystem path also works as `repo_url` for fully offline demos.

## Rebuilding / re-seeding the catalog

```bash
python3 scripts/seed_prs.py --remote origin --repo Ay-obami/swarm-demo-target
# add --no-open-prs for branches only; REMOTE="." style local refs via git update-ref
```

The seeder force-updates the seeded branches (they're re-derived from green
`main` + the patch table inside `prs.json`), pushes them, opens fresh GitHub
PRs, closes stale ones, and regenerates `OPEN_PRS.md`.

