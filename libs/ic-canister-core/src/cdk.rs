#[cfg(not(test))]
pub use ic_cdk::*;

#[cfg(test)]
pub use mocks::*;

// Mock ic system call api for tests.
#[cfg(not(target_arch = "wasm32"))]
pub mod mocks {
    use candid::Principal;

    pub const TEST_CANISTER_ID: Principal = Principal::from_slice(&[u8::MAX; 29]);

    pub fn caller() -> Principal {
        Principal::anonymous()
    }

    pub fn spawn<F: 'static + std::future::Future<Output = ()>>(_future: F) {
        // do nothing since this is a mock for testing purposes
    }

    pub mod api {
        use candid::Principal;
        use std::time::{SystemTime, UNIX_EPOCH};

        static mut IC_TIME: SystemTime = UNIX_EPOCH;

        pub fn set_mock_ic_time(time: SystemTime) {
            unsafe {
                IC_TIME = time;
            }
        }

        pub fn time() -> u64 {
            unsafe { IC_TIME.duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64 }
        }

        pub fn id() -> Principal {
            super::TEST_CANISTER_ID
        }

        pub fn is_controller(principal: &Principal) -> bool {
            principal == &id()
        }

        pub fn trap(message: &str) -> ! {
            panic!("{}", message);
        }

        pub fn print<S: AsRef<str>>(s: S) {
            println!("{}", s.as_ref());
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
    use super::mocks::{api::trap, caller};
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
}
