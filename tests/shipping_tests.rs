//! Shipping acceptance tests: inclusive brackets, free-shipping threshold,
//! per-kg ceiling surcharge.

use warehouse::shipping;

#[test]
fn tier_brackets_are_inclusive() {
    assert_eq!(shipping::shipping_cents(0), 300);
    assert_eq!(shipping::shipping_cents(999), 300);
    assert_eq!(shipping::shipping_cents(1_000), 600);
    assert_eq!(shipping::shipping_cents(4_999), 600);
    assert_eq!(shipping::shipping_cents(5_000), 1_200);
}

#[test]
fn free_shipping_is_inclusive_at_threshold() {
    assert!(shipping::is_free(10_000, 5_000));
    assert!(!shipping::is_free(10_000, 5_001));
    assert!(!shipping::is_free(9_999, 100));
}

#[test]
fn weight_surcharge_charges_every_started_kilo() {
    assert_eq!(shipping::weight_based_cents(1), 250);
    assert_eq!(shipping::weight_based_cents(1_000), 250);
    assert_eq!(shipping::weight_based_cents(1_001), 500);
}
