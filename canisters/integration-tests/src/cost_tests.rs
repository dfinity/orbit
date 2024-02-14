use std::{
    borrow::Borrow,
    sync::{Arc, RwLock},
    time::Duration,
};

use candid::Principal;
use lazy_static::lazy_static;
use wallet_api::{
    AddUserOperationInput, ApiErrorDTO, CreateProposalInput, CreateProposalResponse, ProposalDTO,
};

use crate::{
    domain_utils::{CyclesTracker, TestOrg},
    setup::{setup_new_env, WALLET_ADMIN_USER},
    utils::update_candid_as,
    TestEnv,
};

/// Simulate an origanization's workflows over months and output cycle usage.
///
/// Users: all employees
///
/// Accounts:
///   - treasury
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
///

#[test]
fn cost_tests() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let mut org = TestOrg::new(env, canister_ids.wallet);

    let group_admin = org.get_group_by_name("Admin");

    let (proposal, finance_user_1) = org.create_user(
        WALLET_ADMIN_USER,
        "Finance User 1",
        vec![group_admin.id.clone()],
    );
    assert!(matches!(
        proposal.status,
        wallet_api::ProposalStatusDTO::Adopted
    ));

    org.advance_time(Duration::from_secs(10), 2);

    let (proposal, infrasec_user) = org.create_user(
        WALLET_ADMIN_USER,
        "Infrasec User",
        vec![group_admin.id.clone()],
    );

    org.advance_time(Duration::from_secs(10), 2);

    org.pass_proposal(
        &proposal.id,
        vec![
            // WALLET_ADMIN_USER,
            finance_user_1,
        ],
    );

    org.cycles.make_snapshot(&org.env);

    // println!("{:#?}", proposal);

    // println!("{:#?}", org.group_users_map);

    // org.create_user("name", groups)
}
