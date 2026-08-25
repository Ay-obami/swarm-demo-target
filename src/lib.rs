//! Warehouse domain logic used as the Swarm CI demo target.
//!
//! Modules: money math ([`currency`]), pricing ([`pricing`]), discount
//! stacking ([`discounts`]), stock reservation ([`inventory`]),
//! reporting helpers ([`report`]) over shared [`models`].

pub mod currency;
pub mod discounts;
pub mod inventory;
pub mod models;
pub mod pricing;
pub mod report;
pub mod shipping;
pub mod tax;

