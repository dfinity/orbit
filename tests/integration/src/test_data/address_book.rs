use super::next_unique_id;
use crate::utils::{sha256_hex, submit_request, wait_for_request};
use candid::Principal;
use pocket_ic::PocketIc;

pub fn add_address_book_entry(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> station_api::AddressBookEntryDTO {
    let next_id = next_unique_id();

    let add_address_book_entry_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::AddAddressBookEntry(
            station_api::AddAddressBookEntryOperationInput {
                blockchain: "icp".to_string(),
                address_format: "icp_account_identifier".to_string(),
                labels: vec!["icp_native".to_string()],
                address_owner: format!("user-{}", next_id),
                metadata: Vec::new(),
                address: format!("{}{}", "0x", sha256_hex(&next_id.to_le_bytes())),
            },
        ),
    );

    let new_request = wait_for_request(
        env,
        requester,
        station_canister_id,
        add_address_book_entry_request,
    )
    .expect("Failed to add address book entry");

    match new_request.operation {
        station_api::RequestOperationDTO::AddAddressBookEntry(add_address_book_entry) => {
            add_address_book_entry.address_book_entry.unwrap()
        }
        _ => panic!("invalid request operation"),
    }
}

pub fn edit_address_book_entry_owner(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    address_book_entry_id: station_api::UuidDTO,
    address_owner: String,
) {
    let edit_address_book_entry_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::EditAddressBookEntry(
            station_api::EditAddressBookEntryOperationInput {
                address_book_entry_id,
                address_owner: Some(address_owner),
                change_metadata: None,
                labels: None,
            },
        ),
    );

    wait_for_request(
        env,
        requester,
        station_canister_id,
        edit_address_book_entry_request,
    )
    .expect("Failed to edit address book entry");
}
