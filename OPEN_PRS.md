# Open demo PRs

Copy each Bug block into the Phoenix CI dashboard quick-pick.

## PR-101 - Discount stacking ignores the cap
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/7
- Branch: `pr/101-discount-cap`

```text
Stacked discounts ignore discount_cap_cents: subtotal 100.00 with 15% + 3.00 fixed grants 18.00 — the 5.00 cap must win.
```

## PR-102 - FX conversion snaps to whole units
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/8
- Branch: `pr/102-fx-rounding`

```text
convert() snaps results to whole units: convert(999, 12345) returns 1200 instead of 1233 — cent precision must be preserved.
```

## PR-103 - Batch reservation can oversell
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/9
- Branch: `pr/103-oversell-boundary`

```text
reserve() validates each request against TOTAL stock instead of remaining availability, so [3,3,5] on stock 10 / reserved 4 succeeds.
```

## PR-104 - Basis-point rounding uses half-up
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/10
- Branch: `pr/104-half-up-bps`

```text
mul_bps_round_even rounds exact halves UP instead of to the even neighbour (25c @50% gives 13, must be 12).
```

## PR-105 - Exact halves are truncated
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/11
- Branch: `pr/105-truncate-always`

```text
mul_bps_round_even drops every exact half (75c @50% gives 37, must be 38).
```

## PR-106 - Hidden flat FX fee
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/12
- Branch: `pr/106-convert-flat-fee`

```text
convert() silently adds a 50c conversion fee: convert(10_000, 11250) returns 11300 instead of 11250.
```

## PR-107 - Line total charges one unit too few
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/13
- Branch: `pr/107-qty-minus-one`

```text
line_total_cents charges qty-1 units: a line of 2 × 350c bills 350c instead of 700c.
```

## PR-108 - Tax computed on gross instead of net
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/14
- Branch: `pr/108-tax-on-gross`

```text
tax_cents compounds onto a naive tax (taxes the gross), inflating receipts: 105c @15% yields 18c of tax instead of 16c.
```

## PR-109 - Tax rounding uses half-up
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/15
- Branch: `pr/109-tax-half-up`

```text
tax_cents rounds halves up: 5c @50% yields 3c instead of banker's 2c.
```

## PR-110 - Free-shipping threshold inverted
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/16
- Branch: `pr/110-free-threshold-inverted`

```text
is_free() requires weight ABOVE the threshold instead of at-or-below, so eligible orders pay shipping.
```

## PR-111 - Per-kg surcharge floors instead of ceiling
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/17
- Branch: `pr/111-per-kg-floor`

```text
weight_based_cents(1001) returns 250 instead of 500 — started kilograms must be charged.
```

## PR-112 - Century years are always leap
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/18
- Branch: `pr/112-leap-century`

```text
is_leap_year(1900) returns true — the divisible-by-100 rule was dropped.
```

## PR-113 - Calendar quarter off by one at edges
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/19
- Branch: `pr/113-quarter-off-by-one`

```text
quarter_of_month(3) returns Q2 — bracket edges are wrong (months 3,6,9 shift into the next quarter).
```

## PR-114 - Fiscal-year offset ignored
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/20
- Branch: `pr/114-fiscal-offset-dropped`

```text
fiscal_quarter ignores offset_months: with an April FY start, April must be FQ1 but reports as calendar Q2.
```

## PR-115 - Availability arguments swapped
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/21
- Branch: `pr/115-available-swapped`

```text
available() returns reserved minus stock (saturated), so every reservation attempt is rejected.
```

## PR-116 - Availability panics on over-reservation
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/22
- Branch: `pr/116-drop-saturation`

```text
available() uses plain subtraction and panics when reserved exceeds stock instead of saturating at zero.
```

## PR-117 - Batch partially grants instead of failing
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/23
- Branch: `pr/117-partial-grant`

```text
reserve() grants whatever fits and continues; the contract requires failing the WHOLE batch on the first impossible request.
```

## PR-118 - Cap applied to percentage leg only
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/24
- Branch: `pr/118-cap-leg-only`

```text
The discount cap limits only the percentage leg before the fixed amount is added: 15%+3.00 with cap 5.00 grants 8.00 instead of 5.00.
```

## PR-119 - Negative fixed discount not clamped
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/25
- Branch: `pr/119-negative-grant`

```text
A negative discount_fixed_cents produces a NEGATIVE grant (order pays extra); grants must clamp to zero.
```

## PR-120 - Grant order not preserved
- Link: https://github.com/Ay-obami/swarm-demo-target/pull/26
- Branch: `pr/120-grant-order-violated`

```text
reserve() returns granted amounts sorted ascending instead of preserving request order ([5,2] comes back as [2,5]).
```