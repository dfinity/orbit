use std::time::Duration;

use candid::Principal;
use ic_cdk::println;
use ic_ledger_types::AccountIdentifier;
use wallet_api::{
    AccountPoliciesDTO, AddAccountOperationInput, AddAddressBookEntryOperationInput,
    AddUserOperationInput, ApiErrorDTO, ApprovalThresholdDTO, CommonActionSpecifierDTO,
    CommonSpecifierDTO, CreateProposalInput, CreateProposalResponse, CriteriaDTO,
    ListProposalsInput, ListUsersInput, MetadataDTO, MinimumVotesDTO, PaginationInput, ProposalDTO,
    ProposalStatusDTO, ResourceSpecifierDTO, UserGroupDTO, UserSpecifierDTO, UuidDTO,
};

use crate::{
    domain_utils::{
        advance_time, create_add_access_policy_proposal, create_add_account_proposal,
        create_add_address_book_entry_proposal, create_add_user_group_proposal,
        create_add_user_proposal, create_transfer_proposal, fetch_accounts,
        fetch_addressbook_entries, fetch_groups, fetch_proposals, fetch_users,
        get_estimated_instructions_from_cycles, get_group_by_name, get_group_users,
        get_icp_balance, get_proposal, measure_instruction_count_of_call, mint_icp, pass_proposal,
        CyclesTracker, UnderscoreFormatter, UserIdGenerator,
    },
    interfaces::default_account,
    setup::{setup_new_env, WALLET_ADMIN_USER},
    utils::{get_user, update_candid_as},
    TestEnv,
};

/// Simulate an origanization's workflows over months and output cycle usage.
///
/// Users: all employees
///
/// Accounts:
///   - treasury
///   - icp-to-go
///
/// User Groups:
///   Admin:
///     - upgrade the canisters every month
///   Finance:
///     - make transfers to employees every month (periodic)
///     - approve employee, growth/marketing, grant transfers
///   Grants:
///     - make transfers to grantees every month
///     - approve grant transfers
///   Engineering:
///     - approve icp-to-go requests to non-kyc'ed addresses
///   Growth/Marketing:
///     - request transfers to pay for various services
///     - approve growth/marketing requests
///
///   /every user/:
///     - request small amounts to pay for various services (eg. icp-to-go)
///
/// Address book:
///   - kyc'ed employee addresses
///   - kyc'ed grantee addresses
///   - kyc'ed service provider addresses
///
/// Permissions (Access policies)
///
/// Policies (Proposal policies)
///  - finance+grants+engineering+growth/marketing can create transfers for the treasury account
///  - all transfers from the treasury account must go to a kyc'ed address
///  - all users can create transfers for the icp-to-go account

#[derive(Debug)]
enum OrgDepartment {
    Finance,
    Grants,
    Engineering,
    GrowthMarketing,
    Other,
}

#[derive(Debug)]
struct OrgUser {
    identity: Principal,
    account_id: String,
    name: String,
    department: OrgDepartment,
    is_admin: bool,
    is_management: bool,
}

#[test]
fn cost_tests() {
    let TestEnv {
        env,
        canister_ids,
        minter,
        ..
    } = setup_new_env();

    let mut cycles = CyclesTracker::new(&env, canister_ids.wallet);
    let mut user_id_generator = UserIdGenerator::new();

    let mut users: Vec<OrgUser> = vec![];

    let formatter = UnderscoreFormatter::new();

    // finance team, make first as admin, make 4 as management
    for i in 0..5 {
        let identity = user_id_generator.get_next_user_id();
        users.push(OrgUser {
            identity,
            account_id: default_account(identity),
            name: format!("Finance User {}", i + 1),
            department: OrgDepartment::Finance,
            is_admin: i == 0,
            is_management: i < 4,
        });
    }

    // engineering team, make first as admin, make 10 as management
    for i in 0..20 {
        let identity = user_id_generator.get_next_user_id();
        users.push(OrgUser {
            identity,
            account_id: default_account(identity),
            name: format!("Engineering User {}", i + 1),
            department: OrgDepartment::Engineering,
            is_admin: i == 0,
            is_management: i < 10,
        });
    }

    // growth/marketing team, make first as admin, make 5 as management
    for i in 0..5 {
        let identity = user_id_generator.get_next_user_id();
        users.push(OrgUser {
            identity,
            account_id: default_account(identity),
            name: format!("Growth/Marketing User {}", i + 1),
            department: OrgDepartment::GrowthMarketing,
            is_admin: i == 0,
            is_management: i < 5,
        });
    }

    // no particular team
    for i in 0..10 {
        let identity = user_id_generator.get_next_user_id();
        users.push(OrgUser {
            identity,
            account_id: default_account(identity),
            name: format!("Other User {}", i + 1),
            department: OrgDepartment::Other,
            is_admin: false,
            is_management: false,
        });
    }

    // grants team, make first as admin, make 1 as management
    for i in 0..3 {
        let identity = user_id_generator.get_next_user_id();
        users.push(OrgUser {
            identity,
            account_id: default_account(identity),
            name: format!("Grants User {}", i + 1),
            department: OrgDepartment::Grants,
            is_admin: i == 0,
            is_management: i < 1,
        });
    }

    let grantees = (0..100)
        .map(|_| {
            let identity = user_id_generator.get_next_user_id();
            default_account(identity)
        })
        .collect::<Vec<_>>();

    let service_providers = (0..5)
        .map(|_| {
            let identity = user_id_generator.get_next_user_id();
            default_account(identity)
        })
        .collect::<Vec<_>>();

    let other_recipients = (0..200)
        .map(|_| {
            let identity = user_id_generator.get_next_user_id();
            default_account(identity)
        })
        .collect::<Vec<_>>();

    // -- User groups --------------------------------

    // auto-approved since there's only one admin
    create_add_user_group_proposal(&env, canister_ids.wallet, WALLET_ADMIN_USER, "Finance");
    create_add_user_group_proposal(&env, canister_ids.wallet, WALLET_ADMIN_USER, "Engineering");
    create_add_user_group_proposal(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "Growth/Marketing",
    );
    create_add_user_group_proposal(&env, canister_ids.wallet, WALLET_ADMIN_USER, "Grants");
    advance_time(&env, Duration::from_secs(10), 2);

    // -- Users --------------------------------

    println!("Creating {} users...", users.len());

    for (index, user) in users.iter().enumerate() {
        let mut groups = vec![];

        if user.is_admin {
            groups.push(get_group_by_name(&env, canister_ids.wallet, "Admin").id);
        }

        if user.is_management {
            groups.push(
                get_group_by_name(
                    &env,
                    canister_ids.wallet,
                    match user.department {
                        OrgDepartment::Finance => "Finance",
                        OrgDepartment::Engineering => "Engineering",
                        OrgDepartment::GrowthMarketing => "Growth/Marketing",
                        OrgDepartment::Grants => "Grants",
                        OrgDepartment::Other => "Admin",
                    },
                )
                .id,
            );
        }

        println!("Creating user: {}/{}", index + 1, users.len());

        let proposal = create_add_user_proposal(
            &env,
            canister_ids.wallet,
            user.identity,
            WALLET_ADMIN_USER,
            &user.name,
            groups,
        );

        advance_time(&env, Duration::from_secs(10), 2);

        pass_proposal(
            &env,
            canister_ids.wallet,
            &proposal.id,
            &get_group_users(
                &env,
                canister_ids.wallet,
                &get_group_by_name(&env, canister_ids.wallet, "Admin").id,
            ),
            false,
        );
    }

    // -- Accounts ------------------------------------

    println!("Creating Treasury account...");

    let proposal = create_add_account_proposal(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        AddAccountOperationInput {
            blockchain: "icp".to_owned(),
            metadata: vec![],
            name: "Treasury".to_owned(),
            owners: get_group_users(
                &env,
                canister_ids.wallet,
                &get_group_by_name(&env, canister_ids.wallet, "Finance").id,
            )
            .iter()
            .map(|u| u.id.clone())
            .collect::<Vec<_>>(),
            standard: "native".to_owned(),
            policies: AccountPoliciesDTO {
                edit: Some(CriteriaDTO::ApprovalThreshold(ApprovalThresholdDTO {
                    threshold: 100,
                    voters: UserSpecifierDTO::Group(vec![
                        get_group_by_name(&env, canister_ids.wallet, "Finance").id,
                    ]),
                })),
                transfer: Some(CriteriaDTO::And(vec![
                    CriteriaDTO::ApprovalThreshold(ApprovalThresholdDTO {
                        threshold: 100,
                        voters: UserSpecifierDTO::Group(vec![
                            get_group_by_name(&env, canister_ids.wallet, "Finance").id,
                        ]),
                    }),
                    CriteriaDTO::HasAddressBookMetadata(MetadataDTO {
                        key: "kyc".to_owned(),
                        value: "yes".to_owned(),
                    }),
                ])),
            },
        },
    );
    advance_time(&env, Duration::from_secs(10), 2);

    pass_proposal(
        &env,
        canister_ids.wallet,
        &proposal.id,
        &get_group_users(
            &env,
            canister_ids.wallet,
            &get_group_by_name(&env, canister_ids.wallet, "Admin").id,
        ),
        true,
    );

    println!("Creating icp-to-go account...");

    let proposal = create_add_account_proposal(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        AddAccountOperationInput {
            blockchain: "icp".to_owned(),
            metadata: vec![],
            name: "icp-to-go".to_owned(),
            owners: get_group_users(
                &env,
                canister_ids.wallet,
                &get_group_by_name(&env, canister_ids.wallet, "Engineering").id,
            )
            .iter()
            .map(|u| u.id.clone())
            .collect::<Vec<_>>(),
            standard: "native".to_owned(),
            policies: AccountPoliciesDTO {
                edit: Some(CriteriaDTO::ApprovalThreshold(ApprovalThresholdDTO {
                    threshold: 100,
                    voters: UserSpecifierDTO::Group(vec![
                        get_group_by_name(&env, canister_ids.wallet, "Engineering").id,
                    ]),
                })),
                transfer: Some(CriteriaDTO::And(vec![CriteriaDTO::MinimumVotes(
                    MinimumVotesDTO {
                        minimum: 1,
                        voters: UserSpecifierDTO::Group(vec![
                            get_group_by_name(&env, canister_ids.wallet, "Engineering").id,
                        ]),
                    },
                )])),
            },
        },
    );

    pass_proposal(
        &env,
        canister_ids.wallet,
        &proposal.id,
        &get_group_users(
            &env,
            canister_ids.wallet,
            &get_group_by_name(&env, canister_ids.wallet, "Admin").id,
        ),
        true,
    );

    advance_time(&env, Duration::from_secs(10), 2);

    // -- Get accounts --------------------------------

    let finance_user = users
        .iter()
        .find(|u| u.is_admin && u.is_management && matches!(u.department, OrgDepartment::Finance))
        .unwrap();

    let finance_accounts = fetch_accounts(&env, canister_ids.wallet, finance_user.identity);

    let treasury_account = finance_accounts
        .iter()
        .find(|a| a.name == "Treasury")
        .unwrap();

    //

    let engineering_user = users
        .iter()
        .find(|u| {
            u.is_admin && u.is_management && matches!(u.department, OrgDepartment::Engineering)
        })
        .unwrap();

    let engineering_accounts = fetch_accounts(&env, canister_ids.wallet, engineering_user.identity);

    let icp_to_go_account = engineering_accounts
        .iter()
        .find(|a| a.name == "icp-to-go")
        .unwrap();

    // -- Mint test ICP -------------------------------

    for account in [&treasury_account, &icp_to_go_account] {
        mint_icp(
            &env,
            minter,
            AccountIdentifier::from_hex(&account.address).unwrap(),
            1000000 * 100_000_000,
        )
        .expect("ICP NOT MINTED");
    }

    println!(
        "{}",
        get_icp_balance(
            &env,
            AccountIdentifier::from_hex(&treasury_account.address).unwrap(),
            WALLET_ADMIN_USER
        )
    );
    println!(
        "{}",
        get_icp_balance(
            &env,
            AccountIdentifier::from_hex(&treasury_account.address).unwrap(),
            WALLET_ADMIN_USER
        )
    );

    // -- Address book --------------------------------

    // chain all addresses together
    let all_addresses = users
        .iter()
        .map(|u| u.account_id.clone())
        .chain(grantees.iter().cloned())
        .chain(service_providers.iter().cloned())
        .chain(other_recipients.iter().cloned())
        .collect::<Vec<_>>();

    println!("Creating {} address book entries", all_addresses.len());

    for (index, address) in all_addresses.iter().enumerate() {
        let admins = get_group_users(
            &env,
            canister_ids.wallet,
            &get_group_by_name(&env, canister_ids.wallet, "Admin").id,
        );

        let sender = admins[index % admins.len()].identities.first().unwrap(); // pick a random admin

        println!(
            "Creating address book entry {}/{}",
            index + 1,
            all_addresses.len()
        );

        let proposal = create_add_address_book_entry_proposal(
            &env,
            canister_ids.wallet,
            *sender,
            AddAddressBookEntryOperationInput {
                address: address.clone(),
                address_owner: "123".to_owned(),
                blockchain: "icp".to_owned(),
                metadata: vec![MetadataDTO {
                    key: "kyc".to_owned(),
                    value: "yes".to_owned(),
                }],
                standard: "native".to_owned(),
            },
        );

        advance_time(&env, Duration::from_secs(10), 2);

        pass_proposal(
            &env,
            canister_ids.wallet,
            &proposal.id,
            &get_group_users(
                &env,
                canister_ids.wallet,
                &get_group_by_name(&env, canister_ids.wallet, "Admin").id,
            ),
            false,
        );
    }

    advance_time(&env, Duration::from_secs(100), 50);

    println!(
        "Cycles after creation: {}",
        formatter.format(cycles.make_snapshot(&env))
    );

    println!(
        "Memory size: {} MB",
        env.get_stable_memory(canister_ids.wallet).len() / 1024 / 1024
    );

    // -- make some policy changes --------------------

    // Finance team can list address book entries

    let proposal = create_add_access_policy_proposal(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        wallet_api::AddAccessPolicyOperationInput {
            user: CommonSpecifierDTO::Group(vec![
                get_group_by_name(&env, canister_ids.wallet, "Finance").id,
            ]),
            resource: ResourceSpecifierDTO::AddressBook(CommonActionSpecifierDTO::List),
        },
    );

    advance_time(&env, Duration::from_secs(10), 2);

    pass_proposal(
        &env,
        canister_ids.wallet,
        &proposal.id,
        &get_group_users(
            &env,
            canister_ids.wallet,
            &get_group_by_name(&env, canister_ids.wallet, "Admin").id,
        ),
        true,
    );

    // -- make transfers --------------------------------

    let finance_management_user = users
        .iter()
        .find(|u| u.is_management && matches!(u.department, OrgDepartment::Finance))
        .unwrap();

    let address_book_entries =
        fetch_addressbook_entries(&env, canister_ids.wallet, finance_management_user.identity);

    println!(
        "Fetched address book entries: {}",
        address_book_entries.len()
    );

    for month in 0..5 {
        println!(
            "[Month {}] Making transfers to all address book entries...",
            month + 1
        );

        for (index, entry) in address_book_entries.iter().enumerate() {
            // println!("Making transfer to: {}", entry.address);

            println!(
                "[Month {}] Creating transfer {}/{}",
                month + 1,
                index + 1,
                address_book_entries.len()
            );

            let proposal = create_transfer_proposal(
                &env,
                canister_ids.wallet,
                finance_management_user.identity,
                wallet_api::TransferOperationInput {
                    from_account_id: treasury_account.id.clone(),
                    to: entry.address.clone(),
                    amount: (10 * 100_000_000).into(), // 10 ICP,
                    fee: None,
                    metadata: vec![],
                    network: None,
                },
            );

            advance_time(&env, Duration::from_secs(10), 2);

            pass_proposal(
                &env,
                canister_ids.wallet,
                &proposal.id,
                &get_group_users(
                    &env,
                    canister_ids.wallet,
                    &get_group_by_name(&env, canister_ids.wallet, "Finance").id,
                ),
                false,
            );
        }

        println!(
            "[Month {}] Cycle cost of transfers: {}",
            month + 1,
            formatter.format(cycles.make_snapshot(&env))
        );

        println!(
            "[Month {}] Memory size: {} MB",
            month + 1,
            env.get_stable_memory(canister_ids.wallet).len() / 1024 / 1024
        );

        let instructions = measure_instruction_count_of_call::<_, ()>(
            &env,
            canister_ids.wallet,
            13,
            WALLET_ADMIN_USER,
            "list_proposals",
            (ListProposalsInput {
                created_from_dt: None,
                created_to_dt: None,
                expiration_from_dt: None,
                expiration_to_dt: None,
                operation_types: None,
                voter_ids: None,
                proposer_ids: None,
                statuses: None,
                paginate: Some(PaginationInput {
                    limit: Some(100),
                    offset: None,
                }),
                sort_by: None,
            },),
        );

        println!("Estimated instructions: {}", formatter.format(instructions));
    }
}
