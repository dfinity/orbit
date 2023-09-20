use super::ic::api::{management_canister, trap};
use candid::Principal;
use convert_case::{Case, Casing};
use uuid::Uuid;

pub fn to_upper_snake_case(input: String) -> String {
    input.to_case(Case::Snake).to_uppercase()
}

/// Generates a UUID v4 using the random number generator from the management canister.
pub async fn generate_uuid_v4() -> Uuid {
    let mut bytes = [0u8; 16];
    let raw_rand = match management_canister::main::raw_rand().await {
        Ok((res,)) => res.as_slice().to_owned(),
        Err((_, err)) => trap(format!("Failed to fetch random number: {}", err).as_str()),
    };
    let length = std::cmp::min(16, raw_rand.len());
    bytes[..length].copy_from_slice(&raw_rand[..length]);

    // Ensure it's a valid version 4 UUID (Randomly Generated)
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    Uuid::from_bytes(bytes)
}

pub fn extract_error_enum_variant_name<E: std::error::Error>(err: &E) -> String {
    let full_code = to_upper_snake_case(format!("{:?}", err));
    full_code
        .split(|c| c == '{' || c == '(')
        .next()
        .unwrap_or(&full_code)
        .to_string()
        .trim_matches('_')
        .to_string()
}

pub fn min_principal_id() -> Principal {
    Principal::from_slice(&[0u8; 29])
}

pub fn max_principal_id() -> Principal {
    Principal::from_slice(&[9u8; 29])
}
