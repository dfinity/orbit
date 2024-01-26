use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{execute_proposal, update_candid_as};
use crate::TestEnv;

use wallet_api::{
    AddAddressBookEntryOperationInput, ApiErrorDTO, ChangeMetadataDTO,
    EditAddressBookEntryOperationInput, GetAddressBookEntryInputDTO,
    GetAddressBookEntryResponseDTO, ListAddressBookEntriesInputDTO,
    ListAddressBookEntriesResponseDTO, MetadataDTO, PaginationInput, ProposalOperationDTO,
    ProposalOperationInput,
};

#[test]
fn address_book_entry_lifecycle() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create address book entry
    let add_address_book_entry =
        ProposalOperationInput::AddAddressBookEntry(AddAddressBookEntryOperationInput {
            address_owner: "John Doe".to_string(),
            address: "0x1234".to_string(),
            blockchain: "icp".to_string(),
            standard: "native".to_string(),
            metadata: vec![MetadataDTO {
                key: "kyc".to_string(),
                value: "false".to_string(),
            }],
        });
    let add_address_book_entry_proposal = execute_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        add_address_book_entry,
    )
    .unwrap();
    let address_book_entry = match add_address_book_entry_proposal.operation {
        ProposalOperationDTO::AddAddressBookEntry(operation) => {
            operation.address_book_entry.unwrap()
        }
        _ => panic!("unexpected proposal operation"),
    };
    assert_eq!(address_book_entry.address_owner, "John Doe".to_string());
    assert_eq!(address_book_entry.address, "0x1234".to_string());
    assert_eq!(address_book_entry.blockchain, "icp".to_string());
    assert_eq!(address_book_entry.standard, "native".to_string());
    assert_eq!(
        address_book_entry.metadata,
        vec![MetadataDTO {
            key: "kyc".to_string(),
            value: "false".to_string(),
        }]
    );

    // create address book entry with duplicate address
    let add_address_book_entry =
        ProposalOperationInput::AddAddressBookEntry(AddAddressBookEntryOperationInput {
            address_owner: "Max Mustermann".to_string(),
            address: "0x1234".to_string(),
            blockchain: "icp".to_string(),
            standard: "native".to_string(),
            metadata: vec![MetadataDTO {
                key: "kyc".to_string(),
                value: "true".to_string(),
            }],
        });
    execute_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        add_address_book_entry,
    )
    .unwrap_err();

    // create one more address book entry
    let add_address_book_entry =
        ProposalOperationInput::AddAddressBookEntry(AddAddressBookEntryOperationInput {
            address_owner: "Max Mustermann".to_string(),
            address: "0x5678".to_string(),
            blockchain: "icp".to_string(),
            standard: "native".to_string(),
            metadata: vec![MetadataDTO {
                key: "kyc".to_string(),
                value: "true".to_string(),
            }],
        });
    let add_address_book_entry_proposal = execute_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        add_address_book_entry,
    )
    .unwrap();
    let next_address_book_entry = match add_address_book_entry_proposal.operation {
        ProposalOperationDTO::AddAddressBookEntry(operation) => {
            operation.address_book_entry.unwrap()
        }
        _ => panic!("unexpected proposal operation"),
    };
    assert_eq!(
        next_address_book_entry.address_owner,
        "Max Mustermann".to_string()
    );
    assert_eq!(next_address_book_entry.address, "0x5678".to_string());
    assert_eq!(next_address_book_entry.blockchain, "icp".to_string());
    assert_eq!(next_address_book_entry.standard, "native".to_string());
    assert_eq!(
        next_address_book_entry.metadata,
        vec![MetadataDTO {
            key: "kyc".to_string(),
            value: "true".to_string(),
        }]
    );

    // list address book entries
    let list_address_book_entries_args = ListAddressBookEntriesInputDTO {
        blockchain: "icp".to_string(),
        standard: "native".to_string(),
        paginate: PaginationInput {
            offset: None,
            limit: None,
        },
    };
    let res: (Result<ListAddressBookEntriesResponseDTO, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "list_address_book_entries",
        (list_address_book_entries_args,),
    )
    .unwrap();
    let list_address_book_entries = res.0.unwrap().address_book_entries;
    assert_eq!(
        list_address_book_entries,
        vec![address_book_entry.clone(), next_address_book_entry.clone()]
    );

    // update the address book entry for John Doe setting "kyc" to "true"
    let edit_address_book_entry =
        ProposalOperationInput::EditAddressBookEntry(EditAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id.clone(),
            address_owner: None,
            change_metadata: Some(ChangeMetadataDTO::OverrideSpecifiedBy(vec![MetadataDTO {
                key: "kyc".to_string(),
                value: "true".to_string(),
            }])),
        });
    execute_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        edit_address_book_entry,
    )
    .unwrap();

    // retrieve the updated address book entry and check that "kyc" is indeed "true"
    let get_address_book_entry_args = GetAddressBookEntryInputDTO {
        address_book_entry_id: address_book_entry.id.clone(),
    };
    let res: (Result<GetAddressBookEntryResponseDTO, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "get_address_book_entry",
        (get_address_book_entry_args,),
    )
    .unwrap();
    let get_address_book_entry = res.0.unwrap().address_book_entry;
    let mut old_address_book_entry = address_book_entry;
    old_address_book_entry.metadata[0].value = "true".to_string();
    assert_eq!(get_address_book_entry, old_address_book_entry);
}
