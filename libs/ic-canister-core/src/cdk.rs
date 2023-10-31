#[cfg(not(test))]
pub use ic_cdk::*;

#[cfg(test)]
pub use mocks::*;

// Mock ic system call api for tests.
pub mod mocks {
    use candid::Principal;

    pub const TEST_CANISTER_ID: Principal = Principal::from_slice(&[u8::MAX; 29]);

    pub fn caller() -> Principal {
        Principal::anonymous()
    }

    pub fn spawn<F: 'static + Send + std::future::Future<Output = ()>>(future: F) {
        tokio::task::spawn(future);
    }

    pub mod api {
        use candid::Principal;

        pub fn time() -> u64 {
            use std::time::SystemTime;

            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
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
    use super::mocks::{
        api::{time, trap},
        caller,
    };
    use candid::Principal;
    use std::time::SystemTime;

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
    fn time_gives_back_current_system_time() {
        let retrieved_time = time();
        let expected_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert_eq!(retrieved_time, expected_time)
    }
}
