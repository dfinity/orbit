/// Formats a balance to a floating point number with the given number of decimals.
///
/// The floating conversion will only be precise up to 15 decimal places.
pub fn amount_to_f64(balance: i128, decimals: u32) -> f64 {
    balance as f64 / 10u128.pow(decimals) as f64
}

#[cfg(test)]
mod tests {
    use super::amount_to_f64;

    #[test]
    fn test_amount_to_f64() {
        assert_eq!(amount_to_f64(1_000_000_000_000_000_000, 18), 1.0);
        assert_eq!(
            amount_to_f64(1_000_000_000_000_000_000, 0),
            1_000_000_000_000_000_000.0
        );
        assert_eq!(
            amount_to_f64(1_000_000_000_000_000_000, 6),
            1_000_000_000_000.0
        );
    }

    #[test]
    fn test_negative_amount_to_f64() {
        assert_eq!(amount_to_f64(-1_000_000_000_000_000_000, 18), -1.0);
        assert_eq!(
            amount_to_f64(-1_000_000_000_000_000_000, 0),
            -1_000_000_000_000_000_000.0
        );
        assert_eq!(
            amount_to_f64(-1_000_000_000_000_000_000, 6),
            -1_000_000_000_000.0
        );
    }

    #[test]
    fn test_amount_to_f64_zero() {
        assert_eq!(amount_to_f64(0, 18), 0.0);
        assert_eq!(amount_to_f64(0, 0), 0.0);
        assert_eq!(amount_to_f64(0, 6), 0.0);
    }

    #[test]
    fn test_amount_to_f64_large() {
        assert_eq!(
            amount_to_f64(1_000_000_000_000_000_000_000_000_000_000_000, 18),
            1_000_000_000_000_000.0
        );
        assert_eq!(
            amount_to_f64(1_000_000_000_000_000_000_000_000_000_000_000, 0),
            1_000_000_000_000_000_000_000_000_000_000_000.0
        );
        assert_eq!(
            amount_to_f64(1_000_000_000_000_000_000_000_000_000_000_000, 6),
            1_000_000_000_000_000_000_000_000_000.0
        );
    }

    #[test]
    fn test_amount_to_f64_with_decimal_points() {
        assert_eq!(amount_to_f64(1_000_123_456, 12), 0.001_000_123_456);
        assert_eq!(amount_to_f64(1_000_123_456, 9), 1.000_123_456);
        assert_eq!(amount_to_f64(1_000_123_456, 6), 1_000.123_456);
        assert_eq!(amount_to_f64(1_000_123_456, 3), 1_000_123.456);
        assert_eq!(amount_to_f64(1_000_123_456, 0), 1_000_123_456.0);
    }

    #[test]
    fn test_amount_to_f64_with_negative_decimal_points() {
        assert_eq!(amount_to_f64(-1_000_123_456, 12), -0.001_000_123_456);
        assert_eq!(amount_to_f64(-1_000_123_456, 9), -1.000_123_456);
        assert_eq!(amount_to_f64(-1_000_123_456, 6), -1_000.123_456);
        assert_eq!(amount_to_f64(-1_000_123_456, 3), -1_000_123.456);
        assert_eq!(amount_to_f64(-1_000_123_456, 0), -1_000_123_456.0);
    }
}
