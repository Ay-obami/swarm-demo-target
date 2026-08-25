//! Reporting helpers (calendar quarters + leap years + fiscal calendars).

pub fn quarter_of_month(month: u32) -> u8 {
    assert!((1..=12).contains(&month), "month out of range: {month}");
    (((month - 1) / 3) + 1) as u8
}

pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Fiscal quarter when the fiscal year starts `offset_months` after January
/// (0 = calendar year, 3 = FY starting April, …). Q1 covers the first three
/// months OF the fiscal year.
pub fn fiscal_quarter(month: u32, offset_months: u32) -> u8 {
    assert!((1..=12).contains(&month), "month out of range: {month}");
    let idx = (month - 1 + 12 - offset_months % 12) % 12;
    (idx / 3 + 1) as u8
}


