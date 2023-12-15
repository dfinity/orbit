use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::Principal;
use ic_canister_core::cdk::api::management_canister::main::CanisterId;
use ic_cdk::api::management_canister::main::{CanisterSettings, UpdateSettingsArgument};
use pocket_ic::{with_candid, CallError, PocketIc};

pub fn controller_test_id() -> Principal {
    let mut bytes = 0_u64.to_le_bytes().to_vec();
    bytes.push(0xfd); // internal marker for controller test id
    bytes.push(0x01); // marker for opaque ids
    Principal::from_slice(&bytes)
}

pub fn minter_test_id() -> Principal {
    let mut bytes = 0_u64.to_le_bytes().to_vec();
    bytes.push(0xfc); // internal marker for minter test id
    bytes.push(0x01); // marker for opaque ids
    Principal::from_slice(&bytes)
}

pub fn user_test_id(n: u64) -> Principal {
    let mut bytes = n.to_le_bytes().to_vec();
    bytes.push(0xfe); // internal marker for user test ids
    bytes.push(0x01); // marker for opaque ids
    Principal::from_slice(&bytes)
}

pub fn update_canister_settings(
    env: &PocketIc,
    sender: Option<Principal>,
    canister_id: Principal,
    settings: CanisterSettings,
) {
    let args = UpdateSettingsArgument {
        settings,
        canister_id,
    };

    // the type () is required here due to rust not being able to infer the type of the return automatically
    let _: () = update_candid_as(
        env,
        Principal::management_canister(),
        sender.unwrap_or(Principal::anonymous()),
        "update_settings",
        (args,),
    )
    .unwrap();
}

/// Call a canister candid update method, authenticated. The sender can be impersonated (i.e., the
/// signature is not verified).
pub fn update_candid_as<Input, Output>(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    method: &str,
    input: Input,
) -> Result<Output, CallError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    with_candid(input, |bytes| {
        env.update_call(canister_id, sender, method, bytes)
    })
}
