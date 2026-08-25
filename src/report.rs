//! Reporting helpers (calendar quarters + leap years).

pub fn quarter_of_month(month: u32) -> u8 {
    assert!((1..=12).contains(&month), "month out of range: {month}");
    (((month - 1) / 3) + 1) as u8
}

pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
