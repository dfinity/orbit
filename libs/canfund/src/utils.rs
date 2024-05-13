use crate::{errors::Error, manager::record::CyclesBalance};

/// Converts the cycles from `candid::Nat` to `u128`.
pub fn cycles_nat_to_u128(cycles: candid::Nat) -> Result<u128, Error> {
    let cycles_text = cycles.0.to_string();

    match cycles.0.try_into() {
        Ok(cycles) => Ok(cycles),
        Err(_) => Err(Error::FailedCyclesConversion {
            cycles: cycles_text,
        }),
    }
}

/// Converts the cycles from `String` to `u128`.
pub fn cycles_str_to_u128(cycles: &str) -> Result<u128, Error> {
    match cycles.parse::<u128>() {
        Ok(cycles) => Ok(cycles),
        Err(_) => Err(Error::FailedCyclesConversion {
            cycles: cycles.to_string(),
        }),
    }
}

/// Calculates the estimated cycles per second based on the current and previous cycles balance.
pub fn calc_estimated_cycles_per_sec(
    current_cycles: &CyclesBalance,
    previous_cycles: &CyclesBalance,
) -> u128 {
    // The current cycles balance should be a measurement after the previous cycles balance.
    if current_cycles.timestamp <= previous_cycles.timestamp {
        return 0;
    }

    let consumed_cycles = previous_cycles.amount.saturating_sub(current_cycles.amount);

    // time_spent is never 0 because the timestamp is always increasing.
    let time_spent_in_ns = current_cycles.timestamp - previous_cycles.timestamp;
    let time_spent_in_milis = time_spent_in_ns / 1_000_000;
    let time_spent_in_secs = time_spent_in_milis / 1_000;

    if time_spent_in_secs == 0 {
        return 0;
    }

    consumed_cycles / time_spent_in_secs as u128
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_cycles_nat_to_u128() {
        let cycles = candid::Nat(BigUint::from(100u32));
        assert_eq!(cycles_nat_to_u128(cycles).unwrap(), 100);
    }

    #[test]
    fn test_cycles_str_to_u128() {
        assert_eq!(cycles_str_to_u128("100").unwrap(), 100);
    }

    #[test]
    fn test_cycles_str_to_u128_invalid() {
        assert_eq!(
            cycles_str_to_u128("invalid").unwrap_err(),
            Error::FailedCyclesConversion {
                cycles: "invalid".to_string(),
            }
        );
    }

    #[test]
    fn test_cycles_str_to_u128_empty() {
        assert_eq!(
            cycles_str_to_u128("").unwrap_err(),
            Error::FailedCyclesConversion {
                cycles: "".to_string(),
            }
        );
    }

    #[test]
    fn test_calc_estimated_cycles_per_sec() {
        let previous_cycles = CyclesBalance {
            amount: 75_000_000_000,
            timestamp: 10 * 1000 * 1_000_000,
        };
        let current_cycles = CyclesBalance {
            amount: 50_000_000_000,
            timestamp: 15 * 1000 * 1_000_000,
        };

        let estimated_cycles_per_sec =
            calc_estimated_cycles_per_sec(&current_cycles, &previous_cycles);

        assert_eq!(estimated_cycles_per_sec, 5_000_000_000);
    }

    #[test]
    fn test_calc_estimated_cycles_per_sec_zero_time_spent() {
        let previous_cycles = CyclesBalance {
            amount: 50_000_000_000,
            timestamp: 10 * 1000 * 1_000_000,
        };
        let current_cycles = CyclesBalance {
            amount: 50_000_000_000,
            timestamp: 10 * 1000 * 1_000_000,
        };

        let estimated_cycles_per_sec =
            calc_estimated_cycles_per_sec(&current_cycles, &previous_cycles);

        assert_eq!(estimated_cycles_per_sec, 0);
    }

    #[test]
    fn test_calc_estimated_cycles_per_sec_negative() {
        let previous_cycles = CyclesBalance {
            amount: 50_000_000_000,
            timestamp: 15 * 1000 * 1_000_000,
        };
        let current_cycles = CyclesBalance {
            amount: 75_000_000_000,
            timestamp: 10 * 1000 * 1_000_000,
        };

        let estimated_cycles_per_sec =
            calc_estimated_cycles_per_sec(&current_cycles, &previous_cycles);

        assert_eq!(estimated_cycles_per_sec, 0);
    }

    #[test]
    fn test_calc_estimated_cycles_per_sec_zero_consumed_cycles() {
        let previous_cycles = CyclesBalance {
            amount: 50_000_000_000,
            timestamp: 10 * 1000 * 1_000_000,
        };
        let current_cycles = CyclesBalance {
            amount: 50_000_000_000,
            timestamp: 15 * 1000 * 1_000_000,
        };

        let estimated_cycles_per_sec =
            calc_estimated_cycles_per_sec(&current_cycles, &previous_cycles);

        assert_eq!(estimated_cycles_per_sec, 0);
    }

    #[test]
    fn test_calc_estimated_cycles_per_sec_zero_previous_cycles() {
        let previous_cycles = CyclesBalance {
            amount: 50_000_000_000,
            timestamp: 0,
        };
        let current_cycles = CyclesBalance {
            amount: 0,
            timestamp: 10 * 1000 * 1_000_000,
        };

        let estimated_cycles_per_sec =
            calc_estimated_cycles_per_sec(&current_cycles, &previous_cycles);

        assert_eq!(estimated_cycles_per_sec, 5_000_000_000);
    }

    #[test]
    fn test_calc_estimated_cycles_per_sec_current_has_more_cycles() {
        let previous_cycles = CyclesBalance {
            amount: 75_000_000_000,
            timestamp: 10 * 1000 * 1_000_000,
        };
        let current_cycles = CyclesBalance {
            amount: 100_000_000_000,
            timestamp: 15 * 1000 * 1_000_000,
        };

        let estimated_cycles_per_sec =
            calc_estimated_cycles_per_sec(&current_cycles, &previous_cycles);

        assert_eq!(estimated_cycles_per_sec, 0);
    }
}
