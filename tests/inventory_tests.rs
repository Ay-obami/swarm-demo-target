//! Acceptance tests for batch reservation semantics (PR-103 target).

use warehouse::inventory::{self, InventoryError};

#[test]
fn grants_against_remaining_availability() {
    // stock 10, already reserved 4 ⇒ 6 free; requests 3 then 3 both fit.
    let granted = inventory::reserve(10, 4, &[3, 3]).unwrap();
    assert_eq!(granted, vec![3, 3]);
}

#[test]
fn first_impossible_request_fails_the_batch() {
    // stock 10, reserved 4 ⇒ 6 free; after granting 3+3 nothing is left for 5.
    let err = inventory::reserve(10, 4, &[3, 3, 5]).unwrap_err();
    assert_eq!(
        err,
        InventoryError::Oversell { requested: 5, available: 0 }
    );
}

#[test]
fn grant_order_is_preserved() {
    let granted = inventory::reserve(10, 0, &[5, 2]).unwrap();
    assert_eq!(granted, vec![5, 2]);
}

#[test]
fn exact_boundary_request_succeeds() {
    assert!(inventory::reserve(10, 4, &[6]).is_ok());
}
