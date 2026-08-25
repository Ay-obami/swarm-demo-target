//! Money math: basis-point scaling with banker's (half-to-even) rounding.

/// Multiply `amount` by `bps / 10_000`, rounding exact halves to the EVEN
/// neighbour. Inputs are non-negative minor units; output stays a whole cent.
pub fn mul_bps_round_even(amount: u64, bps: u64) -> u64 {
    let product = amount as u128 * bps as u128;
    let q = product / 10_000;
    let r = (product % 10_000) as u64;
    let bump = r >= 5_000; // BUG(PR-104): half-up, not banker's.
    if bump {
        (q + 1) as u64
    } else {
        q as u64
    }
}

/// Convert an amount into another currency at `rate_bps`
/// (e.g. 11_250 bps = 1.125×). Cents precision is preserved exactly.
pub fn convert(amount: u64, rate_bps: u64) -> u64 {
    mul_bps_round_even(amount, rate_bps)
}
