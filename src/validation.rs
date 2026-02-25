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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_date_strings() {
        assert!(validate_date_string("2024-01-15"));
        assert!(validate_date_string("2000-12-31"));
        assert!(validate_date_string("1999-06-01"));
    }

    #[test]
    fn invalid_date_empty() {
        assert!(!validate_date_string(""));
    }

    #[test]
    fn invalid_date_wrong_separator() {
        assert!(!validate_date_string("2024/01/15"));
    }

    #[test]
    fn invalid_date_too_short() {
        assert!(!validate_date_string("24-01-15"));
        assert!(!validate_date_string("2024-1-15"));
        assert!(!validate_date_string("2024-01-1"));
    }

    #[test]
    fn invalid_date_too_long() {
        assert!(!validate_date_string("2024-01-150"));
    }

    #[test]
    fn invalid_date_non_digits() {
        assert!(!validate_date_string("abcd-ef-gh"));
    }

    #[test]
    fn date_format_only_validation() {
        // This validates format only, not calendar correctness
        assert!(validate_date_string("2024-13-01"));
        assert!(validate_date_string("9999-99-99"));
    }

    #[test]
    fn valid_year_strings() {
        assert!(validate_year_string("2024"));
        assert!(validate_year_string("1999"));
        assert!(validate_year_string("2000"));
    }

    #[test]
    fn invalid_year_empty() {
        assert!(!validate_year_string(""));
    }

    #[test]
    fn invalid_year_too_short() {
        assert!(!validate_year_string("24"));
    }

    #[test]
    fn invalid_year_too_long() {
        assert!(!validate_year_string("20244"));
    }

    #[test]
    fn invalid_year_non_digits() {
        assert!(!validate_year_string("abcd"));
        assert!(!validate_year_string("20-4"));
    }
}
