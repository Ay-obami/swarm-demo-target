//! Stock reservation with batch semantics: requests are granted in order and
//! the FIRST request that cannot be satisfied aborts the whole batch.

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum InventoryError {
    #[error("oversell: requested {requested} units but only {available} are available")]
    Oversell { requested: u32, available: u32 },
}

pub fn available(stock: u32, reserved: u32) -> u32 {
    stock.saturating_sub(reserved)
}

/// Reserve each request against REMAINING availability (stock minus already
/// reserved). The first request that cannot be satisfied fails the batch.
pub fn reserve(stock: u32, reserved: u32, requests: &[u32]) -> Result<Vec<u32>, InventoryError> {
    let mut left = available(stock, reserved);
    let mut granted = Vec::with_capacity(requests.len());
    for &request in requests {
        // BUG(PR-103): compares against TOTAL stock instead of what's left,
        // so cumulative grants within one batch can oversell.
        if request > stock {
            return Err(InventoryError::Oversell {
                requested: request,
                available: left,
            });
        }
        left -= request;
        granted.push(request);
    }
    Ok(granted)
}
