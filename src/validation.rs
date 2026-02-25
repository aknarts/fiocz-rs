use log::{error, warn};

pub(crate) fn validate_date_string(date: &str) -> bool {
    if date.len() != 10 {
        error!("Incorrect length");
        return false;
    }
    for (index, c) in date.chars().enumerate() {
        if [
            0usize, 1usize, 2usize, 3usize, 5usize, 6usize, 8usize, 9usize,
        ]
        .contains(&index)
        {
            if !c.is_ascii_digit() {
                warn!("{c} is not a digit on position {index}");
                return false;
            }
        } else if c != '-' {
            warn!("{c} is not a dash on position {index}");
            return false;
        }
    }
    true
}

pub(crate) fn validate_year_string(year: &str) -> bool {
    if year.len() != 4 {
        error!("Incorrect length");
        return false;
    }
    for (index, c) in year.chars().enumerate() {
        if !c.is_ascii_digit() {
            warn!("{c} is not a digit on position {index}");
            return false;
        }
    }
    true
}
