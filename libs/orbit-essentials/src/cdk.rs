#[cfg(not(test))]
pub use ic_cdk::*;

#[cfg(not(test))]
pub mod ic_cdk_extended {
    /// Returns the current time in nanoseconds since the epoch.
    ///
    /// This function increments the time by 1ns for each call in the same round.
    pub fn next_time() -> u64 {
        crate::utils::next_time(ic_cdk::api::time())
    }
}

#[cfg(not(test))]
pub use ic_cdk_extended::*;

#[cfg(test)]
pub use mocks::*;

// Mock ic system call api for tests.
#[cfg(not(target_arch = "wasm32"))]
pub mod mocks {
    use candid::Principal;
    use std::cell::RefCell;

    pub const TEST_CANISTER_ID: Principal = Principal::from_slice(&[u8::MAX; 29]);
    pub const TEST_CONTROLLER_ID: Principal = Principal::from_slice(&[u8::MAX - 1; 29]);

    thread_local! {
        static CALLER_ID : RefCell<Principal> = const { RefCell::new(Principal::anonymous()) };
    }

    pub fn set_caller(principal: Principal) {
        CALLER_ID.with(|c| *c.borrow_mut() = principal);
    }

    pub fn caller() -> Principal {
        CALLER_ID.with(|c| *c.borrow())
    }

    pub fn spawn<F: 'static + std::future::Future<Output = ()>>(_future: F) {
        // do nothing since this is a mock for testing purposes
    }

    pub fn next_time() -> u64 {
        // removes the ns precision of the current time to simulate a round time in nanoseconds
        let similated_round_duration_ns = 50_000_000;
        let similated_round_time_ns =
            api::time() / similated_round_duration_ns * similated_round_duration_ns;

        crate::utils::next_time(similated_round_time_ns)
    }

    pub mod api {
        use candid::Principal;
        use std::{
            cell::RefCell,
            time::{SystemTime, UNIX_EPOCH},
        };

        thread_local! {
            static IC_TIME: RefCell<SystemTime> = const { RefCell::new(UNIX_EPOCH) };
            static IC_CANISTER_BALANCE: RefCell<u64> = const { RefCell::new(100_000_000_000) };
        }

        pub fn data_certificate() -> Option<Vec<u8>> {
            None
        }

        pub fn set_certified_data(_data: &[u8]) {}

        pub fn set_mock_ic_time(time: SystemTime) {
            IC_TIME.with(|t| *t.borrow_mut() = time);
        }

        pub fn set_mock_canister_balance(balance: u64) {
            IC_CANISTER_BALANCE.with(|b| *b.borrow_mut() = balance);
        }

        pub fn time() -> u64 {
            IC_TIME.with(|t| t.borrow().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64)
        }

        pub fn id() -> Principal {
            super::TEST_CANISTER_ID
        }

        pub fn is_controller(principal: &Principal) -> bool {
            *principal == super::TEST_CONTROLLER_ID
        }

        pub fn trap(message: &str) -> ! {
            panic!("{}", message);
        }

        pub fn print<S: AsRef<str>>(s: S) {
            println!("{}", s.as_ref());
        }

        pub fn canister_balance() -> u64 {
            IC_CANISTER_BALANCE.with(|b| *b.borrow())
        }

        pub mod call {
            pub fn arg_data_raw_size() -> usize {
                42
            }
        }

        pub mod management_canister {
            pub mod main {
                use ic_cdk::api::call::CallResult;

                pub async fn raw_rand() -> CallResult<(Vec<u8>,)> {
                    Ok((vec![0; 32],))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mocks::{
        api::{set_mock_ic_time, trap},
        caller, next_time,
    };
    use candid::Principal;

    #[test]
    fn caller_is_anonymous() {
        assert_eq!(caller(), Principal::anonymous());
    }

    #[test]
    #[should_panic(expected = "this is an expected panic")]
    fn trap_panics_with_given_message() {
        trap("this is an expected panic")
    }

    #[test]
    fn next_time_correctly_increments() {
        set_mock_ic_time(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(100));

        // increments time by 1ns for each call in the same round
        assert_eq!(next_time(), 100_000_000);
        assert_eq!(next_time(), 100_000_001);
        assert_eq!(next_time(), 100_000_002);

        set_mock_ic_time(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(150));

        assert_eq!(next_time(), 150_000_000);
        assert_eq!(next_time(), 150_000_001);
        assert_eq!(next_time(), 150_000_002);

        // still same round, so increment continues. Rounds are simulated to last 50ms.
        set_mock_ic_time(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(175));
        assert_eq!(next_time(), 150_000_003);
        assert_eq!(next_time(), 150_000_004);
    }
}
