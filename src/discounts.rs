//! Discount stacking policy: PERCENTAGE first, then the FIXED amount, then
//! the CAP — and the grant can never exceed the subtotal or go negative.

use crate::currency::mul_bps_round_even;
use crate::models::Order;

pub fn total_discount_cents(order: &Order, subtotal_cents: i64) -> i64 {
    let subtotal = subtotal_cents.max(0) as u64;
    let pct =
        mul_bps_round_even(subtotal, order.discount_pct_bps.max(0) as u64) as i64;
    let fixed = order.discount_fixed_cents.max(0);
    let raw = pct + fixed;
    let capped = raw.min(order.discount_cap_cents.max(0));
    capped.min(subtotal_cents.max(0))
}
