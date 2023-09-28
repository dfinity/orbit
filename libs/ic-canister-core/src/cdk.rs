// Internet computer api system calls.
#[cfg(not(test))]
mod production {
    pub use ic_cdk::*;
}

// Mock ic system call api for tests.
#[cfg(test)]
mod test {
    use candid::Principal;

    pub fn caller() -> Principal {
        Principal::anonymous()
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
            Principal::anonymous()
        }

        pub fn trap(message: &str) -> ! {
            panic!("{}", message);
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

// Use the correct module based on the environment
#[cfg(not(test))]
pub use production::*;

#[cfg(test)]
pub use test::*;

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::{
        api::{time, trap},
        caller,
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
    fn time_gives_back_current_system_time() {
        let retrieved_time = time();
        let expected_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert_eq!(retrieved_time, expected_time)
    }
}
