use crate::cdk::api::{management_canister, trap};
use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use uuid::{Builder, Uuid};

thread_local! {
  static RNG: RefCell<Option<ChaCha20Rng>> = RefCell::new(None);
}

#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
))]
/// A getrandom implementation that works in the IC.
pub fn ic_custom_getrandom_bytes_impl(dest: &mut [u8]) -> Result<(), getrandom::Error> {
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
getrandom::register_custom_getrandom!(ic_custom_getrandom_bytes_impl);

pub async fn random_bytes<const N: usize>() -> [u8; N] {
    maybe_initialize_rng().await;

    random_bytes_gen::<N>()
}

/// Initializes the random number generator if it has not been initialized yet.
///
/// This function is async because it may need to call into the management canister to get a seed with `raw_rand``.
pub async fn maybe_initialize_rng() {
    let is_initialized = RNG.with(|rng| rng.borrow().is_some());
    if !is_initialized {
        let (created_seed,) = management_canister::main::raw_rand()
            .await
            .unwrap_or_else(|_| trap("call to raw_rand failed"));

        let seed = created_seed
            .try_into()
            .unwrap_or_else(|_| trap("raw_rand not 32 bytes"));

        initialize_rng_from_seed(seed);
    }
}

/// Initializes the random number generator with the given seed.
pub fn initialize_rng_from_seed(seed: [u8; 32]) {
    RNG.with(|rng| {
        let new_rng = ChaCha20Rng::from_seed(seed);
        let _ = rng.borrow_mut().insert(new_rng);
    });
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
    Builder::from_bytes(bytes).into_uuid()
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
