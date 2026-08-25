//! Sales-tax computation. Tax is EXCLUSIVE: computed on the post-discount
//! NET subtotal, rounded half-to-even to whole cents.

use crate::currency::mul_bps_round_even;

pub fn tax_cents(net_subtotal_cents: i64, rate_bps: i64) -> i64 {
    let net = net_subtotal_cents.max(0) as u64;
    mul_bps_round_even(net, rate_bps.max(0) as u64) as i64
}

/// Net + tax convenience for receipts.
pub fn gross_cents(net_subtotal_cents: i64, rate_bps: i64) -> i64 {
    net_subtotal_cents.max(0) + tax_cents(net_subtotal_cents, rate_bps)
}
