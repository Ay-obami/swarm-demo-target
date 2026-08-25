//! Tiered shipping costs. Brackets are gram-based and INCLUSIVE of their
//! upper bound; orders ≥ 10_000 cents ship free when the parcel weighs at
//! most FREE_THRESHOLD_GRAMS.

pub const FREE_THRESHOLD_GRAMS: u32 = 5_000;

pub fn shipping_cents(weight_g: u32) -> i64 {
    match weight_g {
        0..=999 => 300,
        1_000..=4_999 => 600,
        5_000..=19_999 => 1_200,
        _ => 2_000,
    }
}

pub fn is_free(order_cents: i64, weight_g: u32) -> bool {
    order_cents >= 10_000 && weight_g <= FREE_THRESHOLD_GRAMS
}

/// Per-kilo surcharge: 250c per started kilogram (ceiling).
pub fn weight_based_cents(weight_g: u32) -> i64 {
    let kilos = weight_g.div_ceil(1_000);
    kilos as i64 * 250
}
