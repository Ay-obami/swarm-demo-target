//! Report-helper acceptance tests incl. fiscal-year offsets.

use warehouse::report;

#[test]
fn calendar_quarters() {
    for (m, q) in [(1u32, 1), (3, 1), (4, 2), (6, 2), (7, 3), (9, 3), (10, 4), (12, 4)] {
        assert_eq!(report::quarter_of_month(m), q, "month {m}");
    }
}

#[test]
fn leap_years_respect_century_rule() {
    assert!(report::is_leap_year(2024));
    assert!(report::is_leap_year(2000));
    assert!(!report::is_leap_year(1900));
    assert!(!report::is_leap_year(2023));
}

#[test]
fn fiscal_offset_shifts_quarters() {
    // FY starting April (offset 3): April/May/June are FQ1.
    assert_eq!(report::fiscal_quarter(4, 3), 1);
    assert_eq!(report::fiscal_quarter(6, 3), 1);
    assert_eq!(report::fiscal_quarter(8, 3), 2);
    // Zero offset must equal the calendar quarter.
    assert_eq!(report::fiscal_quarter(2, 0), 1);
}
