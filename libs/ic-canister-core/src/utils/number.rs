/// Formats a balance to a floating point number with the given number of decimals.
pub fn format_amount(balance: i128, decimals: u32) -> f64 {
    balance as f64 / 10u128.pow(decimals) as f64
}

#[cfg(test)]
mod tests {
    use super::format_amount;

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount(1_000_000_000_000_000_000, 18), 1.0);
        assert_eq!(
            format_amount(1_000_000_000_000_000_000, 0),
            1_000_000_000_000_000_000.0
        );
        assert_eq!(
            format_amount(1_000_000_000_000_000_000, 6),
            1_000_000_000_000.0
        );
    }

    #[test]
    fn test_negative_format_amount() {
        assert_eq!(format_amount(-1_000_000_000_000_000_000, 18), -1.0);
        assert_eq!(
            format_amount(-1_000_000_000_000_000_000, 0),
            -1_000_000_000_000_000_000.0
        );
        assert_eq!(
            format_amount(-1_000_000_000_000_000_000, 6),
            -1_000_000_000_000.0
        );
    }

    #[test]
    fn test_format_amount_zero() {
        assert_eq!(format_amount(0, 18), 0.0);
        assert_eq!(format_amount(0, 0), 0.0);
        assert_eq!(format_amount(0, 6), 0.0);
    }

    #[test]
    fn test_format_amount_large() {
        assert_eq!(
            format_amount(1_000_000_000_000_000_000_000_000_000_000_000, 18),
            1_000_000_000_000_000.0
        );
        assert_eq!(
            format_amount(1_000_000_000_000_000_000_000_000_000_000_000, 0),
            1_000_000_000_000_000_000_000_000_000_000_000.0
        );
        assert_eq!(
            format_amount(1_000_000_000_000_000_000_000_000_000_000_000, 6),
            1_000_000_000_000_000_000_000_000_000.0
        );
    }

    #[test]
    fn test_format_amount_with_decimal_points() {
        assert_eq!(format_amount(1_000_123_456, 12), 0.001_000_123_456);
        assert_eq!(format_amount(1_000_123_456, 9), 1.000_123_456);
        assert_eq!(format_amount(1_000_123_456, 6), 1_000.123_456);
        assert_eq!(format_amount(1_000_123_456, 3), 1_000_123.456);
        assert_eq!(format_amount(1_000_123_456, 0), 1_000_123_456.0);
    }

    #[test]
    fn test_format_amount_with_negative_decimal_points() {
        assert_eq!(format_amount(-1_000_123_456, 12), -0.001_000_123_456);
        assert_eq!(format_amount(-1_000_123_456, 9), -1.000_123_456);
        assert_eq!(format_amount(-1_000_123_456, 6), -1_000.123_456);
        assert_eq!(format_amount(-1_000_123_456, 3), -1_000_123.456);
        assert_eq!(format_amount(-1_000_123_456, 0), -1_000_123_456.0);
    }
}
