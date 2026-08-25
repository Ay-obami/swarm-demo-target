//! Sales-tax computation. Tax is EXCLUSIVE: computed on the post-discount
//! NET subtotal, rounded half-to-even to whole cents.

use crate::currency::mul_bps_round_even;

pub fn tax_cents(net_subtotal_cents: i64, rate_bps: i64) -> i64 {
    // BUG(PR-109): rounds halves UP, breaking banker's rounding.
    let net = net_subtotal_cents.max(0) as u64;
    let product = net * rate_bps.max(0) as u64;
    let q = product / 10_000;
    let r = (product % 10_000) as u64;
    (q + u64::from(r >= 5_000)) as i64
}

/// Net + tax convenience for receipts.
pub fn gross_cents(net_subtotal_cents: i64, rate_bps: i64) -> i64 {
    net_subtotal_cents.max(0) + tax_cents(net_subtotal_cents, rate_bps)
}
