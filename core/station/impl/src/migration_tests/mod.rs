#[cfg(test)]
mod test {

    use std::fs;

    use ic_cdk::api::stable::WASM_PAGE_SIZE_IN_BYTES;
    use ic_stable_structures::{memory_manager::MemoryId, Memory};
    use orbit_essentials::{model::ModelKey, repository::Repository};

    use crate::{
        core::{
            with_memory_manager, ACCOUNT_MEMORY_ID, ADDRESS_BOOK_MEMORY_ID, REQUEST_MEMORY_ID,
            TRANSFER_MEMORY_ID,
        },
        models::{
            permission::Allow, Account, AccountAddress, AccountAsset, AccountBalance,
            AddAccountOperation, AddAccountOperationInput, AddAddressBookEntryOperation,
            AddAddressBookEntryOperationInput, AddressBookEntry, AddressFormat, Blockchain,
            ChangeAssets, EditAccountOperation, EditAccountOperationInput, Metadata, Request,
            RequestExecutionPlan, RequestOperation, RequestPolicyRule, RequestPolicyRuleInput,
            RequestStatus, TokenStandard, Transfer, TransferOperation, TransferOperationInput,
            TransferStatus,
        },
        repositories::{
            ACCOUNT_REPOSITORY, ADDRESS_BOOK_REPOSITORY, REQUEST_REPOSITORY, TRANSFER_REPOSITORY,
        },
        services::{INITIAL_ICP_ASSET, INITIAL_ICP_ASSET_ID},
        STABLE_MEMORY_VERSION,
    };

    fn save_memory_snapshot(label: &str, memory_id: MemoryId) {
        let snapshot = with_memory_manager(|memory_manager| {
            let mem = memory_manager.get(memory_id);
            let mut snapshot = vec![0; mem.size() as usize * WASM_PAGE_SIZE_IN_BYTES as usize];
            mem.read(0, &mut snapshot);
            snapshot
        });

        fs::write(
            format!(
                "src/migration_tests/snapshots/{}_v{}.bin",
                label, STABLE_MEMORY_VERSION
            ),
            snapshot,
        )
        .unwrap();
    }

    fn generate_address_book_repo_snapshot() {
        let entries: Vec<AddressBookEntry> = vec![
            AddressBookEntry {
                id: [0u8; 16],
                address: "0x1234567890abcdef".to_string(),
                address_format: AddressFormat::ICPAccountIdentifier,
                address_owner: "Alice".to_string(),
                blockchain: crate::models::Blockchain::InternetComputer,
                labels: vec!["Alice".to_string(), "Bob".to_string()],
                last_modification_timestamp: 0,
                metadata: Metadata::default(),
            },
            AddressBookEntry {
                id: [1u8; 16],
                address: "0x1234567890abcdef".to_string(),
                address_format: AddressFormat::ICPAccountIdentifier,
                address_owner: "Alice".to_string(),
                blockchain: crate::models::Blockchain::InternetComputer,
                labels: vec!["Alice".to_string(), "Bob".to_string()],
                last_modification_timestamp: 0,
                metadata: Metadata::new(
                    [
                        ("key1".to_string(), "value1".to_string()),
                        ("key2".to_string(), "value2".to_string()),
                    ]
                    .into_iter()
                    .collect(),
                ),
            },
        ];

        for entry in entries {
            ADDRESS_BOOK_REPOSITORY.insert(entry.key(), entry);
        }

        save_memory_snapshot("address_book_repository", ADDRESS_BOOK_MEMORY_ID);
    }

    fn generate_transfer_repo_snapshot() {
        let entries: Vec<Transfer> = vec![Transfer {
            id: [0u8; 16],
            initiator_user: [0u8; 16],
            from_account: [0u8; 16],
            from_asset: [0u8; 16],
            with_standard: TokenStandard::InternetComputerNative,
            to_address: "0x1234567890abcdef".to_string(),
            status: TransferStatus::Completed {
                signature: None,
                hash: None,
                completed_at: 0,
            },
            amount: 100u64.into(),
            request_id: [0u8; 16],
            fee: 100u64.into(),
            blockchain_network: "mainnet".to_string(),
            metadata: Metadata::default(),
            last_modification_timestamp: 0,
            created_timestamp: 0,
        }];

        for entry in entries {
            TRANSFER_REPOSITORY.insert(entry.key(), entry);
        }

        save_memory_snapshot("transfer_repository", TRANSFER_MEMORY_ID);
    }

    fn generate_account_repo_snapshot() {
        let entries: Vec<Account> = vec![Account {
            id: [0u8; 16],
            name: "Test account".to_string(),
            assets: vec![AccountAsset {
                asset_id: [0u8; 16],
                balance: Some(AccountBalance {
                    balance: 100u64.into(),
                    last_modification_timestamp: 0,
                }),
            }],
            seed: [0u8; 16],
            addresses: vec![
                AccountAddress {
                    address: "0x1234567890abcdef".to_string(),
                    format: AddressFormat::ICPAccountIdentifier,
                },
                AccountAddress {
                    address: "0x1234567890abcdef".to_string(),
                    format: AddressFormat::ICRC1Account,
                },
            ],
            metadata: Metadata::default(),
            transfer_request_policy_id: None,
            configs_request_policy_id: None,
            last_modification_timestamp: 0,
        }];

        for entry in entries {
            ACCOUNT_REPOSITORY.insert(entry.key(), entry);
        }

        save_memory_snapshot("account_repository", ACCOUNT_MEMORY_ID);
    }

    fn generate_request_repo_snapshot() {
        let entries: Vec<Request> = vec![
            Request {
                id: [0u8; 16],
                title: "Test transfer".to_string(),
                summary: None,
                requested_by: [0u8; 16],
                status: RequestStatus::Approved,
                operation: RequestOperation::Transfer(TransferOperation {
                    fee: None,
                    transfer_id: Some([0u8; 16]),
                    asset: INITIAL_ICP_ASSET.clone(),
                    input: TransferOperationInput {
                        from_account_id: [0u8; 16],
                        from_asset_id: INITIAL_ICP_ASSET_ID,
                        with_standard: TokenStandard::InternetComputerNative,
                        to: "0x1234567890abcdef".to_string(),
                        amount: 100u64.into(),
                        metadata: Metadata::default(),
                        network: "mainnet".to_string(),
                        fee: None,
                    },
                }),
                expiration_dt: 0,
                execution_plan: RequestExecutionPlan::Immediate,
                approvals: vec![],
                created_timestamp: 0,
                last_modification_timestamp: 0,
                tags: vec!["test_tag1".to_string(), "test_tag2".to_string()],
            },
            Request {
                id: [1u8; 16],
                title: "Test add account".to_string(),
                summary: None,
                requested_by: [0u8; 16],
                status: RequestStatus::Approved,
                operation: RequestOperation::AddAccount(AddAccountOperation {
                    account_id: None,
                    input: AddAccountOperationInput {
                        name: "Test account".to_string(),
                        assets: vec![[0u8; 16]],
                        metadata: Metadata::new(
                            [
                                ("key1".to_string(), "value1".to_string()),
                                ("key2".to_string(), "value2".to_string()),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                        read_permission: Allow::default(),
                        configs_permission: Allow::default(),
                        transfer_permission: Allow::default(),
                        configs_request_policy: Some(RequestPolicyRule::AutoApproved),
                        transfer_request_policy: Some(RequestPolicyRule::AutoApproved),
                    },
                }),
                expiration_dt: 0,
                execution_plan: RequestExecutionPlan::Immediate,
                approvals: vec![],
                created_timestamp: 0,
                last_modification_timestamp: 0,
                tags: vec!["test_tag1".to_string()],
            },
            Request {
                id: [2u8; 16],
                title: "Test edit account".to_string(),
                summary: None,
                requested_by: [0u8; 16],
                status: RequestStatus::Approved,
                operation: RequestOperation::EditAccount(EditAccountOperation {
                    input: EditAccountOperationInput {
                        account_id: [0u8; 16],
                        name: Some("Test account".to_string()),
                        change_assets: Some(ChangeAssets::Change {
                            add_assets: vec![[0u8; 16], [1u8; 16]],
                            remove_assets: vec![[2u8; 16], [3u8; 16]],
                        }),
                        read_permission: Some(Allow::default()),
                        configs_permission: Some(Allow::default()),
                        transfer_permission: Some(Allow::default()),
                        configs_request_policy: Some(RequestPolicyRuleInput::Set(
                            RequestPolicyRule::AutoApproved,
                        )),
                        transfer_request_policy: Some(RequestPolicyRuleInput::Remove),
                    },
                }),
                expiration_dt: 0,
                execution_plan: RequestExecutionPlan::Immediate,
                approvals: vec![],
                created_timestamp: 0,
                last_modification_timestamp: 0,
                tags: vec![],
            },
            Request {
                id: [3u8; 16],
                title: "Test add address book entry".to_string(),
                summary: None,
                requested_by: [0u8; 16],
                status: RequestStatus::Approved,
                operation: RequestOperation::AddAddressBookEntry(AddAddressBookEntryOperation {
                    input: AddAddressBookEntryOperationInput {
                        address_owner: "Alice".to_string(),
                        address: "0x1234567890abcdef".to_string(),
                        address_format: AddressFormat::ICPAccountIdentifier,
                        blockchain: Blockchain::InternetComputer,
                        labels: vec!["label_1".to_string(), "label_2".to_string()],
                        metadata: vec![],
                    },
                    address_book_entry_id: Some([0u8; 16]),
                }),
                expiration_dt: 0,
                execution_plan: RequestExecutionPlan::Immediate,
                approvals: vec![],
                created_timestamp: 0,
                last_modification_timestamp: 0,
                tags: vec!["test_tag1".to_string(), "test_tag2".to_string()],
            },
        ];

        for entry in entries {
            REQUEST_REPOSITORY.insert(entry.key(), entry);
        }

        save_memory_snapshot("request_repository", REQUEST_MEMORY_ID);
    }

    #[test]
    fn make_repository_snapshots() {
        generate_address_book_repo_snapshot();
        generate_transfer_repo_snapshot();
        generate_account_repo_snapshot();
        generate_request_repo_snapshot();
    }
}
