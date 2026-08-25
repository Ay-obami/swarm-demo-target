//! Acceptance tests for the discount stacking policy (PR-101 target).
//!
//! Policy: percentage FIRST, then fixed, then cap; never below zero, never
//! above the subtotal.

use warehouse::discounts;
use warehouse::models::{Item, Order};
use warehouse::pricing;

fn order(pct_bps: i64, fixed_cents: i64, cap_cents: i64) -> Order {
    Order {
        id: "t".into(),
        items: vec![],
        discount_pct_bps: pct_bps,
        discount_fixed_cents: fixed_cents,
        discount_cap_cents: cap_cents,
    }
}

#[test]
fn line_and_subtotal_math() {
    let items = vec![
        Item { sku: "a".into(), qty: 2, unit_price_cents: 350 },
        Item { sku: "b".into(), qty: 1, unit_price_cents: 1_000 },
    ];
    assert_eq!(pricing::line_total_cents(&items[0]), 700);
    assert_eq!(pricing::subtotal_cents(&items), 1_700);
}

#[test]
fn cap_limits_total_discount() {
    // subtotal 100.00 → 15% = 15.00, + 3.00 fixed = 18.00 → cap 5.00 wins.
    let o = order(1_500, 300, 500);
    assert_eq!(discounts::total_discount_cents(&o, 10_000), 500);
}

#[test]
fn grant_never_exceeds_subtotal_even_without_cap() {
    let o = order(9_000, 50_000, i64::MAX);
    assert_eq!(discounts::total_discount_cents(&o, 10_000), 10_000);
}

#[test]
fn percentage_rounds_half_to_even() {
    // 99c at 5% = 4.95c → banker's rounding gives 5c (q=49 odd, exact half).
    let o = order(5_000, 0, 10_000);
    assert_eq!(discounts::total_discount_cents(&o, 99), 50);
}
