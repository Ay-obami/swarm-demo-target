//! Shared domain types. Money is always integer **minor units** (cents);
//! ratios are integer **basis points** (10_000 = 100%).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub sku: String,
    pub qty: u32,
    pub unit_price_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: String,
    pub items: Vec<Item>,
    /// Percentage discount in basis points (1_500 = 15%).
    pub discount_pct_bps: i64,
    /// Fixed discount applied after the percentage one.
    pub discount_fixed_cents: i64,
    /// Hard ceiling for the TOTAL discount granted.
    pub discount_cap_cents: i64,
}
