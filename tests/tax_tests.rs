//! Tax acceptance tests. Contract: EXCLUSIVE tax on the NET subtotal,
//! banker's rounding, non-negative.

use warehouse::tax;

#[test]
fn exclusive_tax_rounds_half_to_even() {
    // 105c at 15% = 15.75 → 16 (upper side; not an exact half).
    assert_eq!(tax::tax_cents(105, 1_500), 16);
}

#[test]
fn exact_halves_round_to_even() {
    // 25c at 100% = 25c tax?? rate is bps: 10_000bps = 100% ⇒ 2.5? No:
    // 25 * 10_000 / 10_000 = 25 exactly. Use a true half instead:
    // 5c at 50% (5_000bps) = 2.5 → banker's → 2.
    assert_eq!(tax::tax_cents(5, 5_000), 2);
}

#[test]
fn gross_sums_net_and_tax() {
    assert_eq!(tax::gross_cents(105, 1_500), 121);
}

#[test]
fn negative_net_clamps_to_zero() {
    assert_eq!(tax::tax_cents(-5, 1_500), 0);
}
