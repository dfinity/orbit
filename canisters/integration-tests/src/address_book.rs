use crate::interfaces::{default_account, get_icp_balance, send_icp_to_account, ICP, ICP_FEE};
use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{execute_proposal, get_user, update_candid_as, user_test_id};
use crate::TestEnv;
use ic_ledger_types::AccountIdentifier;

use wallet_api::{
    AccountPoliciesDTO, AddAccountOperationInput, AddAddressBookEntryOperationInput, ApiErrorDTO,
    ChangeMetadataDTO, CriteriaDTO, EditAddressBookEntryOperationInput,
    GetAddressBookEntryInputDTO, GetAddressBookEntryResponseDTO, ListAddressBookEntriesInputDTO,
    ListAddressBookEntriesResponseDTO, MetadataDTO, PaginationInput, ProposalOperationDTO,
    ProposalOperationInput, ProposalStatusDTO, RemoveAddressBookEntryOperationInput,
    TransferOperationInput,
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

    // creating address book entry with duplicate address should fail
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
    let list_res = res.0.unwrap();
    assert_eq!(list_res.total, 2);
    assert_eq!(list_res.next_offset, None);
    let list_address_book_entries = list_res.address_book_entries;
    assert_eq!(list_address_book_entries.len(), 2);
    assert!(list_address_book_entries.contains(&address_book_entry));
    assert!(list_address_book_entries.contains(&next_address_book_entry));

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
    let mut old_address_book_entry = address_book_entry.clone();
    old_address_book_entry.metadata[0].value = "true".to_string();
    assert_eq!(get_address_book_entry, old_address_book_entry);

    // remove the address book entry for John Doe
    let remove_address_book_entry =
        ProposalOperationInput::RemoveAddressBookEntry(RemoveAddressBookEntryOperationInput {
            address_book_entry_id: address_book_entry.id.clone(),
        });
    execute_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        remove_address_book_entry,
    )
    .unwrap();

    // list address book entries and check that the address book entry for John Doe is indeed deleted
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
    let list_res = res.0.unwrap();
    assert_eq!(list_res.total, 1);
    assert_eq!(list_res.next_offset, None);
    let list_address_book_entries = list_res.address_book_entries;
    assert_eq!(
        list_address_book_entries,
        vec![next_address_book_entry.clone()]
    );
}

#[test]
fn check_address_book_for_transfer() {
    let TestEnv {
        env,
        canister_ids,
        controller,
        ..
    } = setup_new_env();

    // create address book entry for John Doe
    let john_doe_id = user_test_id(1);
    let john_doe_account = default_account(john_doe_id);
    let add_address_book_entry =
        ProposalOperationInput::AddAddressBookEntry(AddAddressBookEntryOperationInput {
            address_owner: "John Doe".to_string(),
            address: john_doe_account.clone(),
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

    // get admin user
    let admin_user = get_user(&env, WALLET_ADMIN_USER, canister_ids.wallet);

    // create account for admin user
    let add_account = ProposalOperationInput::AddAccount(AddAccountOperationInput {
        owners: vec![admin_user.id],
        name: "admin".to_string(),
        blockchain: "icp".to_string(),
        standard: "native".to_string(),
        policies: AccountPoliciesDTO {
            transfer: Some(CriteriaDTO::HasAddressBookMetadata(MetadataDTO {
                key: "kyc".to_string(),
                value: "true".to_string(),
            })),
            edit: None,
        },
        metadata: vec![],
    });
    let add_account_proposal =
        execute_proposal(&env, WALLET_ADMIN_USER, canister_ids.wallet, add_account).unwrap();
    let admin_account = match add_account_proposal.operation {
        ProposalOperationDTO::AddAccount(add_account) => add_account.account.unwrap(),
        _ => panic!("unexpected proposal operation"),
    };

    // send ICP to admin user's wallet account
    let admin_account_address = AccountIdentifier::from_hex(&admin_account.address).unwrap();
    send_icp_to_account(&env, controller, admin_account_address, ICP + ICP_FEE, 0).unwrap();

    // try transfer from admin account to John Doe
    // and check that transfer proposal gets rejected
    let transfer = ProposalOperationInput::Transfer(TransferOperationInput {
        from_account_id: admin_account.id,
        to: john_doe_account,
        amount: ICP.into(),
        fee: None,
        metadata: vec![],
        network: None,
    });
    let transfer_error = execute_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        transfer.clone(),
    )
    .unwrap_err();
    match transfer_error {
        Some(ProposalStatusDTO::Rejected { .. }) => (),
        _ => panic!("unexpected transfer status"),
    };

    // check John Doe's balance
    let old_balance = get_icp_balance(&env, john_doe_id);
    assert_eq!(old_balance, 0);

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

    // try transfer from admin account to John Doe again
    // and check that transfer proposal succeeds
    execute_proposal(&env, WALLET_ADMIN_USER, canister_ids.wallet, transfer).unwrap();

    // check John Doe's balance
    let new_balance = get_icp_balance(&env, john_doe_id);
    assert_eq!(new_balance, ICP);
}
