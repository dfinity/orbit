use crate::cdk::api::{management_canister, time};
use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use uuid::{Builder, Uuid};

thread_local! {
  static RNG: RefCell<ChaCha20Rng> = {
    let mut seed = [42; 32];
    seed[..8].copy_from_slice(&time().to_le_bytes());
    RefCell::new(ChaCha20Rng::from_seed(seed))
  };
  static RAW_RAND_SUCCESSFUL: RefCell<bool> = const { RefCell::new(false) };
}

#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
))]
#[no_mangle]
/// A getrandom implementation that works in the IC.
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), getrandom::Error> {
    RNG.with(|rng| {
        let buf = unsafe {
            // create mutable byte slice
            core::slice::from_raw_parts_mut(dest, len)
        };
        let mut rng = rng.borrow_mut();
        rng.fill_bytes(buf);
    });

    Ok(())
}

pub async fn random_bytes<const N: usize>() -> [u8; N] {
    random_bytes_gen::<N>()
}

/// Initializes the random number generator if it has not been initialized yet.
///
/// This function is async because it may need to call into the management canister to get a seed with `raw_rand``.
pub async fn initialize_rng() -> Result<(), String> {
    ic_cdk::print("started to initialize rng");

    let (created_seed,) = management_canister::main::raw_rand()
        .await
        .map_err(|e| e.1)?;

    let seed = created_seed
        .try_into()
        .map_err(|_| "raw_rand not 32 bytes".to_string())?;

    initialize_rng_from_seed(seed);

    RAW_RAND_SUCCESSFUL.with(|b| *b.borrow_mut() = true);

    ic_cdk::print("rng succesfully initialized");
    Ok(())
}

/// Initializes the random number generator with the given seed.
pub fn initialize_rng_from_seed(seed: [u8; 32]) {
    RNG.with(|rng| {
        let new_rng = ChaCha20Rng::from_seed(seed);
        *rng.borrow_mut() = new_rng;
    });
}

pub fn random_bytes_gen<const N: usize>() -> [u8; N] {
    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        let mut bytes = [0u8; N];
        rng.fill_bytes(&mut bytes);
        bytes
    })
}

pub async fn generate_uuid_v4() -> Uuid {
    let bytes = random_bytes::<16>().await;
    Builder::from_random_bytes(bytes).into_uuid()
}

pub fn raw_rand_successful() -> bool {
    RAW_RAND_SUCCESSFUL.with(|b| *b.borrow())
}

#[cfg(test)]
mod tests {
    use super::{generate_uuid_v4, random_bytes};
    use std::collections::HashSet;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_random_bytes() {
        let bytes = random_bytes::<32>().await;
        assert_eq!(bytes.len(), 32);
    }

    #[tokio::test]
    async fn generate_uuid_v4_creates_different_ids() {
        let expected_uniq_ids: usize = 50;
        let mut generated_ids: HashSet<Uuid> = HashSet::with_capacity(expected_uniq_ids);
        for _ in 0..expected_uniq_ids {
            let uuid = generate_uuid_v4().await;
            generated_ids.insert(uuid);
        }

        assert_eq!(expected_uniq_ids, generated_ids.len());
    }
}
