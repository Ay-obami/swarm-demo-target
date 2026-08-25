//! Acceptance tests for FX conversion precision (PR-102 target).
//!
//! Contract: conversion applies the rate ONCE via basis-point scaling and
//! preserves cents exactly (banker's rounding on true halves only).

use warehouse::currency;

#[test]
fn exact_rate_conversion_is_exact() {
    // 100.00 at 1.125× → 112.50
    assert_eq!(currency::convert(10_000, 11_250), 11_250);
}

#[test]
fn halves_round_to_even() {
    assert_eq!(currency::mul_bps_round_even(25, 5_000), 12); // 12.5 → 12
    assert_eq!(currency::mul_bps_round_even(75, 5_000), 38); // 37.5 → 38
}

#[test]
fn cents_precision_is_preserved() {
    // 999c at 1.2345× = 1233.26… → 1233 cents (must NOT snap to whole units).
    assert_eq!(currency::convert(999, 12_345), 1_233);
}
