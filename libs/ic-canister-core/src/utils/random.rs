use crate::cdk::api::{management_canister, trap};
use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use uuid::Uuid;

thread_local! {
  static RNG: RefCell<Option<ChaCha20Rng>> = RefCell::new(None);
}

#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
))]
/// A getrandom implementation that works in the IC.
pub fn ic_random_bytes(dest: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG.with(|maybe_rng| {
        let mut maybe_rng = maybe_rng.borrow_mut();
        let rng = maybe_rng.as_mut().expect("missing random number generator");
        rng.fill_bytes(dest);
    });

    Ok(())
}

#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
))]
getrandom::register_custom_getrandom!(ic_random_bytes);

pub async fn random_bytes<const N: usize>() -> [u8; N] {
    let is_initialized = RNG.with(|rng| rng.borrow().is_some());
    let seed: [u8; 32];
    if !is_initialized {
        let (created_seed,) = management_canister::main::raw_rand()
            .await
            .unwrap_or_else(|_| trap("call to raw_rand failed"));

        seed = created_seed
            .try_into()
            .unwrap_or_else(|_| trap("raw_rand not 32 bytes"));

        maybe_initialize_rng(seed);
    }

    random_bytes_gen::<N>()
}

/// Initializes the random number generator with the given seed, if it hasn't been initialized yet.
///
/// If the random number generator has already been initialized, this function does nothing.
pub fn maybe_initialize_rng(seed: [u8; 32]) {
    let is_initialized = RNG.with(|rng| rng.borrow().is_some());
    if !is_initialized {
        RNG.with(|rng| {
            let new_rng = ChaCha20Rng::from_seed(seed);
            let _ = rng.borrow_mut().insert(new_rng);
        });
    }
}

pub fn random_bytes_gen<const N: usize>() -> [u8; N] {
    RNG.with(|maybe_rng| {
        let mut maybe_rng = maybe_rng.borrow_mut();
        let rng = maybe_rng.as_mut().expect("missing random number generator");
        let mut bytes = [0u8; N];
        rng.fill_bytes(&mut bytes);
        bytes
    })
}

pub async fn generate_uuid_v4() -> Uuid {
    let bytes = random_bytes::<16>().await;
    Uuid::from_bytes(bytes)
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
