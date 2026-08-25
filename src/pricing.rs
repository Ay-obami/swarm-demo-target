//! Line and order-level price computation.

use crate::models::Item;

pub fn line_total_cents(item: &Item) -> i64 {
    item.unit_price_cents * item.qty as i64
}

pub fn subtotal_cents(items: &[Item]) -> i64 {
    items.iter().map(line_total_cents).sum()
}
