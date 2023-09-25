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
        pub fn time() -> u64 {
            use std::time::SystemTime;

            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
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
