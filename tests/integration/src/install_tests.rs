use std::mem;

use candid::{Encode, Principal};
use pocket_ic::PocketIc;
use rstest::rstest;
use station_api::{
    AccountResourceActionDTO, AllowDTO, AuthScopeDTO, InitAccountInput, InitAssetInput,
    InitNamedRuleInput, InitPermissionInput, InitRequestPolicyInput, InitUserGroupInput,
    MetadataDTO, PermissionResourceActionDTO, RequestPolicyRuleDTO, RequestSpecifierDTO,
    ResourceActionDTO, ResourceDTO, ResourceIdDTO, SystemInit, SystemInstall, UserIdentityInput,
    UserInitInput, UserResourceActionDTO, UserStatusDTO, UuidDTO,
};
use uuid::Uuid;

use crate::{
    setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER},
    station_test_data::{
        account::list_accounts, asset::list_assets, named_rule::list_named_rules,
        permission::list_permissions, request_policy::list_request_policies, user::list_users,
        user_group::list_user_groups,
    },
    utils::{await_station_healthy, ADMIN_GROUP_ID, OPERATOR_GROUP_ID},
    TestEnv,
};

#[test]
fn install_with_default_policies() {
    let TestEnv {
        env, controller, ..
    } = setup_new_env();

    let canister_id = env.create_canister_with_settings(Some(controller), None);

    env.set_controllers(canister_id, Some(controller), vec![canister_id, controller])
        .expect("failed to set canister controller");

    env.add_cycles(canister_id, 5_000_000_000_000);
    let station_wasm = get_canister_wasm("station").to_vec();
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();

    let asset_1_id = Uuid::new_v4().hyphenated().to_string();
    let asset_2_id = Uuid::new_v4().hyphenated().to_string();

    let account_1_id = Uuid::new_v4().hyphenated().to_string();

    let users = vec![
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: WALLET_ADMIN_USER,
            }],
            name: "station-admin".to_string(),
            groups: None,
            id: None,
            status: None,
        },
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: Principal::from_slice(&[2; 29]),
            }],
            name: "inactive-station-admin".to_string(),
            groups: Some(vec![ADMIN_GROUP_ID.hyphenated().to_string()]),
            id: None,
            status: Some(station_api::UserStatusDTO::Inactive),
        },
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: Principal::from_slice(&[3; 29]),
            }],
            name: "other-user".to_string(),
            groups: Some(vec![]),
            id: None,
            status: Some(station_api::UserStatusDTO::Active),
        },
    ];

    let accounts = vec![station_api::InitAccountInput {
        name: "test-account".to_string(),
        id: Some(account_1_id.clone()),
        metadata: vec![MetadataDTO {
            key: "test-key".to_string(),
            value: "test-value".to_string(),
        }],
        assets: vec![asset_1_id.clone(), asset_2_id.clone()],
        seed: Uuid::new_v4().to_bytes_le(),
    }];

    let assets = vec![
        station_api::InitAssetInput {
            name: "test-asset-1".to_string(),
            id: asset_1_id.clone(),
            blockchain: "icp".to_string(),
            standards: vec!["icp_native".to_owned()],
            metadata: vec![],
            symbol: "TEST1".to_string(),
            decimals: 8,
        },
        station_api::InitAssetInput {
            name: "test-asset-2".to_string(),
            id: asset_2_id.clone(),
            blockchain: "icp".to_string(),
            standards: vec!["icp_native".to_owned()],
            metadata: vec![],
            symbol: "TEST2".to_string(),
            decimals: 2,
        },
    ];

    let station_init_args = SystemInstall::Init(SystemInit {
        name: "Station".to_string(),
        users: users.clone(),
        upgrader: station_api::SystemUpgraderInput::Deploy(
            station_api::DeploySystemUpgraderInput {
                wasm_module: upgrader_wasm,
                initial_cycles: Some(1_000_000_000_000),
            },
        ),
        fallback_controller: Some(controller),
        quorum: None,
        entries: Some(station_api::InitialEntries::WithDefaultPolicies {
            accounts: accounts.clone(),
            assets: assets.clone(),
        }),
    });
    env.install_canister(
        canister_id,
        station_wasm,
        Encode!(&station_init_args).unwrap(),
        Some(controller),
    );

    await_station_healthy(&env, canister_id, WALLET_ADMIN_USER);

    assert_initial_users(
        &env,
        canister_id,
        WALLET_ADMIN_USER,
        &users,
        &vec![ADMIN_GROUP_ID.hyphenated().to_string()],
    )
    .expect("failed to assert initial users");

    assert_initial_assets(&env, canister_id, WALLET_ADMIN_USER, &assets)
        .expect("failed to assert initial assets");

    assert_initial_accounts(&env, canister_id, WALLET_ADMIN_USER, &accounts)
        .expect("failed to assert initial accounts");

    assert_default_policies_and_permissions_exist(&env, canister_id, WALLET_ADMIN_USER);

    assert_default_groups_exist(&env, canister_id, WALLET_ADMIN_USER);
}

#[test]
fn install_with_all_entries() {
    let TestEnv {
        env, controller, ..
    } = setup_new_env();

    let canister_id = env.create_canister_with_settings(Some(controller), None);

    env.set_controllers(canister_id, Some(controller), vec![canister_id, controller])
        .expect("failed to set canister controller");

    env.add_cycles(canister_id, 5_000_000_000_000);
    let station_wasm = get_canister_wasm("station").to_vec();
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();

    let custom_user_group_id = Uuid::new_v4().hyphenated().to_string();

    let asset_1_id = Uuid::new_v4().hyphenated().to_string();
    let asset_2_id = Uuid::new_v4().hyphenated().to_string();

    let account_1_id = Uuid::new_v4().hyphenated().to_string();

    let allow_custom_user_group_users = AllowDTO {
        auth_scope: AuthScopeDTO::Restricted,
        users: vec![],
        user_groups: vec![custom_user_group_id.clone()],
    };

    let one_from_custom_user_group = RequestPolicyRuleDTO::Quorum(station_api::QuorumDTO {
        approvers: station_api::UserSpecifierDTO::Group(vec![custom_user_group_id.clone()]),
        min_approved: 1,
    });

    let request_policy_add_user_id = Uuid::new_v4().hyphenated().to_string();
    let named_rule_dependent_id = Uuid::new_v4().hyphenated().to_string();

    let permissions = vec![
        InitPermissionInput {
            resource: ResourceDTO::User(UserResourceActionDTO::List),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::User(UserResourceActionDTO::Read(ResourceIdDTO::Any)),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::UserGroup(ResourceActionDTO::List),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::UserGroup(ResourceActionDTO::Read(ResourceIdDTO::Any)),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::Account(AccountResourceActionDTO::List),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::Account(AccountResourceActionDTO::Read(ResourceIdDTO::Any)),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::Asset(ResourceActionDTO::List),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::Asset(ResourceActionDTO::Read(ResourceIdDTO::Any)),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::NamedRule(ResourceActionDTO::List),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::NamedRule(ResourceActionDTO::Read(ResourceIdDTO::Any)),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::RequestPolicy(ResourceActionDTO::List),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::RequestPolicy(ResourceActionDTO::Read(ResourceIdDTO::Any)),
            allow: allow_custom_user_group_users.clone(),
        },
        InitPermissionInput {
            resource: ResourceDTO::Permission(PermissionResourceActionDTO::Read),
            allow: allow_custom_user_group_users.clone(),
        },
    ];

    let request_policies = vec![
        // edit specific request policy, in the wrong order on purpose
        InitRequestPolicyInput {
            id: None,
            specifier: RequestSpecifierDTO::EditRequestPolicy(station_api::ResourceIdsDTO::Ids(
                vec![request_policy_add_user_id.clone()],
            )),
            rule: RequestPolicyRuleDTO::AutoApproved,
        },
        // create user
        InitRequestPolicyInput {
            id: Some(request_policy_add_user_id),
            specifier: RequestSpecifierDTO::AddUser,
            rule: RequestPolicyRuleDTO::AutoApproved,
        },
    ];

    let accounts = vec![station_api::InitAccountWithPermissionsInput {
        account_init: station_api::InitAccountInput {
            name: "test-account".to_string(),
            id: Some(account_1_id.clone()),
            metadata: vec![MetadataDTO {
                key: "test-key".to_string(),
                value: "test-value".to_string(),
            }],
            assets: vec![asset_1_id.clone(), asset_2_id.clone()],
            seed: Uuid::new_v4().to_bytes_le(),
        },
        permissions: station_api::InitAccountPermissionsInput {
            read_permission: allow_custom_user_group_users.clone(),
            configs_permission: allow_custom_user_group_users.clone(),
            transfer_permission: allow_custom_user_group_users.clone(),
            configs_request_policy: Some(one_from_custom_user_group.clone()),
            transfer_request_policy: Some(one_from_custom_user_group.clone()),
        },
    }];

    let users = vec![
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: WALLET_ADMIN_USER,
            }],
            name: "station-admin".to_string(),
            groups: Some(vec![custom_user_group_id.clone()]),
            id: None,
            status: None,
        },
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: Principal::from_slice(&[2; 29]),
            }],
            name: "inactive-station-admin".to_string(),
            groups: Some(vec![custom_user_group_id.clone()]),
            id: None,
            status: Some(station_api::UserStatusDTO::Inactive),
        },
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: Principal::from_slice(&[3; 29]),
            }],
            name: "other-user".to_string(),
            groups: Some(vec![]),
            id: None,
            status: Some(station_api::UserStatusDTO::Active),
        },
    ];

    let assets = vec![
        station_api::InitAssetInput {
            name: "test-asset-1".to_string(),
            id: asset_1_id.clone(),
            blockchain: "icp".to_string(),
            standards: vec!["icp_native".to_owned()],
            metadata: vec![],
            symbol: "TEST1".to_string(),
            decimals: 8,
        },
        station_api::InitAssetInput {
            name: "test-asset-2".to_string(),
            id: asset_2_id.clone(),
            blockchain: "icp".to_string(),
            standards: vec!["icp_native".to_owned()],
            metadata: vec![],
            symbol: "TEST2".to_string(),
            decimals: 2,
        },
    ];

    let named_rules = vec![
        InitNamedRuleInput {
            id: Uuid::new_v4().hyphenated().to_string(),
            name: "custom-named-rule-with-dependency".to_string(),
            description: None,
            rule: RequestPolicyRuleDTO::NamedRule(named_rule_dependent_id.clone()),
        },
        InitNamedRuleInput {
            id: named_rule_dependent_id.clone(),
            name: "custom-named-rule".to_string(),
            description: None,
            rule: RequestPolicyRuleDTO::AutoApproved,
        },
    ];

    let user_groups = vec![InitUserGroupInput {
        id: custom_user_group_id.clone(),
        name: "custom-user-group".to_string(),
    }];

    let station_init_args = SystemInstall::Init(SystemInit {
        name: "Station".to_string(),
        users: users.clone(),
        upgrader: station_api::SystemUpgraderInput::Deploy(
            station_api::DeploySystemUpgraderInput {
                wasm_module: upgrader_wasm,
                initial_cycles: Some(1_000_000_000_000),
            },
        ),
        fallback_controller: Some(controller),
        quorum: None,
        entries: Some(station_api::InitialEntries::Complete {
            accounts: accounts.clone(),
            assets: assets.clone(),
            permissions: permissions.clone(),
            request_policies: request_policies.clone(),
            user_groups: user_groups.clone(),
            named_rules: named_rules.clone(),
        }),
    });
    env.install_canister(
        canister_id,
        station_wasm,
        Encode!(&station_init_args).unwrap(),
        Some(controller),
    );

    await_station_healthy(&env, canister_id, WALLET_ADMIN_USER);

    // assert that the users are in the right groups
    assert_initial_users(&env, canister_id, WALLET_ADMIN_USER, &users, &vec![])
        .expect("failed to assert initial users");

    // assert the number of request policies
    assert_initial_request_policies(
        &env,
        canister_id,
        WALLET_ADMIN_USER,
        &request_policies,
        accounts.len() * 2,
    )
    .expect("failed to assert initial request policies");

    // assert the number of permissions
    assert_initial_permissions(
        &env,
        canister_id,
        WALLET_ADMIN_USER,
        &permissions,
        accounts.len() * 3,
    )
    .expect("failed to assert initial permissions");

    // assert the named rules
    assert_initial_named_rules(&env, canister_id, WALLET_ADMIN_USER, &named_rules)
        .expect("failed to assert initial named rules");

    // assert the names of the user groups
    assert_initial_user_groups(&env, canister_id, WALLET_ADMIN_USER, &user_groups)
        .expect("failed to assert initial user groups");

    // assert the number of accounts and that they have addresses
    assert_initial_accounts(
        &env,
        canister_id,
        WALLET_ADMIN_USER,
        &accounts
            .iter()
            .map(|init| init.account_init.clone())
            .collect(),
    )
    .expect("failed to assert initial accounts");

    // assert the number of assets and their names
    assert_initial_assets(&env, canister_id, WALLET_ADMIN_USER, &assets)
        .expect("failed to assert initial assets");
}

#[test]
fn install_with_all_defaults() {
    let TestEnv {
        env, controller, ..
    } = setup_new_env();

    let canister_id = env.create_canister_with_settings(Some(controller), None);

    env.set_controllers(canister_id, Some(controller), vec![canister_id, controller])
        .expect("failed to set canister controller");

    env.add_cycles(canister_id, 5_000_000_000_000);
    let station_wasm = get_canister_wasm("station").to_vec();
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();

    let users = vec![
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: WALLET_ADMIN_USER,
            }],
            name: "station-admin".to_string(),
            groups: None,
            id: None,
            status: None,
        },
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: Principal::from_slice(&[2; 29]),
            }],
            name: "inactive-operator".to_string(),
            groups: Some(vec![OPERATOR_GROUP_ID.hyphenated().to_string()]),
            id: None,
            status: Some(station_api::UserStatusDTO::Inactive),
        },
        UserInitInput {
            identities: vec![UserIdentityInput {
                identity: Principal::from_slice(&[3; 29]),
            }],
            name: "other-user".to_string(),
            groups: Some(vec![]),
            id: None,
            status: Some(station_api::UserStatusDTO::Active),
        },
    ];

    let station_init_args = SystemInstall::Init(SystemInit {
        name: "Station".to_string(),
        users: users.clone(),
        upgrader: station_api::SystemUpgraderInput::Deploy(
            station_api::DeploySystemUpgraderInput {
                wasm_module: upgrader_wasm,
                initial_cycles: Some(1_000_000_000_000),
            },
        ),
        fallback_controller: Some(controller),
        quorum: None,
        entries: None,
    });
    env.install_canister(
        canister_id,
        station_wasm,
        Encode!(&station_init_args).unwrap(),
        Some(controller),
    );

    await_station_healthy(&env, canister_id, WALLET_ADMIN_USER);

    assert_initial_users(
        &env,
        canister_id,
        WALLET_ADMIN_USER,
        &users,
        &vec![ADMIN_GROUP_ID.hyphenated().to_string()],
    )
    .expect("failed to assert initial users");

    assert_default_assets_exist(&env, canister_id, WALLET_ADMIN_USER);

    assert_default_policies_and_permissions_exist(&env, canister_id, WALLET_ADMIN_USER);

    assert_default_groups_exist(&env, canister_id, WALLET_ADMIN_USER);

    assert_initial_accounts(&env, canister_id, WALLET_ADMIN_USER, &vec![])
        .expect("failed to assert initial accounts");
}

#[rstest]
#[should_panic]
#[case::empty_entries(station_api::InitialEntries::Complete {
    accounts: vec![],
    assets: vec![],
    permissions: vec![],
    request_policies: vec![],
    user_groups: vec![], // no user groups yet user is referencing the ADMIN group
    named_rules: vec![],
})]
#[should_panic]
#[case::circular_named_rules({
    let id_1 = Uuid::new_v4().hyphenated().to_string();
    let id_2 = Uuid::new_v4().hyphenated().to_string();

    station_api::InitialEntries::Complete {
        accounts: vec![],
        assets: vec![],
        permissions: vec![],
        request_policies: vec![],
        user_groups: vec![
            InitUserGroupInput {
                id: ADMIN_GROUP_ID.hyphenated().to_string(),
                name: "admin".to_string(),
            },
        ],
        named_rules: vec![
            // circular reference
            InitNamedRuleInput {
                id: id_1.clone(),
                name: "named_rule".to_string(),
                description: None,
                rule: RequestPolicyRuleDTO::NamedRule(id_2.clone()),
            },
            InitNamedRuleInput {
                id: id_2.clone(),
                name: "named_rule_2".to_string(),
                description: None,
                rule: RequestPolicyRuleDTO::NamedRule(id_1.clone()),
            },
        ],
    }
})]
#[should_panic]
#[case::non_existent_asset_id({
    let id_1 = Uuid::new_v4().hyphenated().to_string();
    station_api::InitialEntries::WithDefaultPolicies {
        accounts: vec![InitAccountInput {
            name: "account".to_string(),
            metadata: vec![],
            assets: vec![
                id_1.clone(), // non-existent asset id
            ],
            id: Some(id_1.clone()),
            seed: Uuid::new_v4().to_bytes_le(),
        }],
        assets: vec![],
    }
})]
#[should_panic]
#[case::non_existent_policy_id({
    let id_1 = Uuid::new_v4().hyphenated().to_string();
    station_api::InitialEntries::Complete {
        accounts: vec![],
        assets: vec![],
        permissions: vec![],
        request_policies: vec![InitRequestPolicyInput {
            id: None,
            rule: RequestPolicyRuleDTO::AutoApproved,
            specifier: RequestSpecifierDTO::EditRequestPolicy(
                station_api::ResourceIdsDTO::Ids(vec![id_1.clone()]), // non-existent policy id
            ),
        }],
        user_groups: vec![
            InitUserGroupInput {
                id: ADMIN_GROUP_ID.hyphenated().to_string(),
                name: "admin".to_string(),
            },
        ],
        named_rules: vec![],
    }
})]
fn install_with_bad_input(#[case] bad_input: station_api::InitialEntries) {
    let TestEnv {
        env, controller, ..
    } = setup_new_env();

    let canister_id = env.create_canister_with_settings(Some(controller), None);

    env.set_controllers(canister_id, Some(controller), vec![canister_id, controller])
        .expect("failed to set canister controller");

    env.add_cycles(canister_id, 5_000_000_000_000);
    let station_wasm = get_canister_wasm("station").to_vec();
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();

    let users = vec![UserInitInput {
        identities: vec![UserIdentityInput {
            identity: WALLET_ADMIN_USER,
        }],
        name: "station-admin".to_string(),
        groups: Some(vec![ADMIN_GROUP_ID.hyphenated().to_string()]),
        id: None,
        status: None,
    }];

    let station_init_args = SystemInstall::Init(SystemInit {
        name: "Station".to_string(),
        users: users.clone(),
        upgrader: station_api::SystemUpgraderInput::Deploy(
            station_api::DeploySystemUpgraderInput {
                wasm_module: upgrader_wasm.clone(),
                initial_cycles: Some(1_000_000_000_000),
            },
        ),
        fallback_controller: Some(controller),
        quorum: None,
        entries: Some(bad_input),
    });
    env.install_canister(
        canister_id,
        station_wasm.clone(),
        Encode!(&station_init_args).unwrap(),
        Some(controller),
    );

    await_station_healthy(&env, canister_id, WALLET_ADMIN_USER);
}

fn assert_initial_users(
    env: &PocketIc,
    canister_id: Principal,
    requester: Principal,
    expected_users: &Vec<UserInitInput>,
    default_groups: &Vec<UuidDTO>,
) -> Result<(), String> {
    let listed_users = list_users(env, canister_id, requester)
        .expect("failed to get users")
        .0
        .expect("failed to get users");

    if expected_users.len() != listed_users.users.len() {
        return Err(format!(
            "expected {} users, got {}",
            expected_users.len(),
            listed_users.users.len()
        ));
    }

    for expected_user in expected_users {
        let user = listed_users
            .users
            .iter()
            .find(|user| user.name == expected_user.name)
            .ok_or(format!("user {} not found", expected_user.name))?;

        expected_user.identities.iter().all(|identity| {
            user.identities
                .iter()
                .any(|user_identity| user_identity == &identity.identity)
        });

        expected_user
            .groups
            .as_ref()
            .unwrap_or(default_groups)
            .iter()
            .all(|group| user.groups.iter().any(|user_group| &user_group.id == group));

        let expected_status = expected_user
            .status
            .as_ref()
            .unwrap_or(&UserStatusDTO::Active);

        if mem::discriminant(&user.status) != mem::discriminant(expected_status) {
            return Err(format!(
                "user {} has status {:?}, expected {:?}",
                expected_user.name, user.status, expected_status
            ));
        }
    }

    Ok(())
}

fn assert_initial_user_groups(
    env: &PocketIc,
    canister_id: Principal,
    requester: Principal,
    expected_user_groups: &Vec<InitUserGroupInput>,
) -> Result<(), String> {
    let listed_user_groups = list_user_groups(env, canister_id, requester)
        .expect("failed to get user groups")
        .0
        .expect("failed to get user groups");

    if expected_user_groups.len() != listed_user_groups.user_groups.len() {
        return Err(format!(
            "expected {} user groups, got {}",
            expected_user_groups.len(),
            listed_user_groups.user_groups.len()
        ));
    }

    for expected_user_group in expected_user_groups {
        let _user_group = listed_user_groups
            .user_groups
            .iter()
            .find(|user_group| user_group.name == expected_user_group.name)
            .ok_or(format!("user group {} not found", expected_user_group.name))?;
    }

    Ok(())
}

fn assert_initial_permissions(
    env: &PocketIc,
    canister_id: Principal,
    requester: Principal,
    expected_permissions: &[InitPermissionInput],
    expected_extra_permissions: usize,
) -> Result<(), String> {
    let listed_permissions = list_permissions(env, canister_id, requester)
        .expect("failed to get permissions")
        .0
        .expect("failed to get permissions");

    if listed_permissions.permissions.len()
        != expected_permissions.len() + expected_extra_permissions
    {
        return Err(format!(
            "expected {} permissions, got {}",
            expected_permissions.len() + expected_extra_permissions,
            listed_permissions.permissions.len()
        ));
    }

    Ok(())
}

fn assert_initial_request_policies(
    env: &PocketIc,
    canister_id: Principal,
    requester: Principal,
    expected_request_policies: &[InitRequestPolicyInput],
    expected_extra_request_policies: usize,
) -> Result<(), String> {
    let listed_request_policies = list_request_policies(env, canister_id, requester)
        .expect("failed to get request policies")
        .0
        .expect("failed to get request policies");

    if listed_request_policies.policies.len()
        != expected_request_policies.len() + expected_extra_request_policies
    {
        return Err(format!(
            "expected {} request policies, got {}",
            expected_request_policies.len() + expected_extra_request_policies,
            listed_request_policies.policies.len()
        ));
    }

    Ok(())
}

fn assert_initial_named_rules(
    env: &PocketIc,
    canister_id: Principal,
    requester: Principal,
    expected_named_rules: &Vec<InitNamedRuleInput>,
) -> Result<(), String> {
    let listed_named_rules = list_named_rules(env, canister_id, requester)
        .expect("failed to get named rules")
        .0
        .expect("failed to get named rules");

    if expected_named_rules.len() != listed_named_rules.named_rules.len() {
        return Err(format!(
            "expected {} named rules, got {}",
            expected_named_rules.len(),
            listed_named_rules.named_rules.len()
        ));
    }

    for expected_named_rule in expected_named_rules {
        let _named_rule = listed_named_rules
            .named_rules
            .iter()
            .find(|named_rule| named_rule.name == expected_named_rule.name)
            .ok_or(format!("named rule {} not found", expected_named_rule.name))?;
    }

    Ok(())
}

fn assert_initial_assets(
    env: &PocketIc,
    canister_id: Principal,
    requester: Principal,
    expected_assets: &Vec<InitAssetInput>,
) -> Result<(), String> {
    let listed_assets = list_assets(env, canister_id, requester)
        .expect("failed to call list_assets")
        .0
        .expect("failed to list assets");

    if expected_assets.len() != listed_assets.assets.len() {
        return Err(format!(
            "expected {} assets, got {}",
            expected_assets.len(),
            listed_assets.assets.len()
        ));
    }

    for expected_asset in expected_assets {
        let asset = listed_assets
            .assets
            .iter()
            .find(|asset| asset.id == expected_asset.id)
            .ok_or(format!("asset {} not found", expected_asset.id))?;

        if asset.id != expected_asset.id
            || asset.name != expected_asset.name
            || asset.blockchain != expected_asset.blockchain
            || asset.standards != expected_asset.standards
            || asset.metadata != expected_asset.metadata
            || asset.symbol != expected_asset.symbol
            || asset.decimals != expected_asset.decimals
        {
            return Err(format!("asset {} does not match expected asset", asset.id));
        }
    }

    Ok(())
}

fn compare_arrays<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.len() == b.len() && a.iter().all(|item| b.contains(item)) && b.iter().all(|item| a.contains(item))
}

fn assert_initial_accounts(
    env: &PocketIc,
    canister_id: Principal,
    requester: Principal,
    expected_accounts: &Vec<InitAccountInput>,
) -> Result<(), String> {
    let listed_accounts = list_accounts(env, canister_id, requester)
        .expect("failed to get account")
        .0
        .expect("failed to get account");

    if expected_accounts.len() != listed_accounts.accounts.len() {
        return Err(format!(
            "expected {} accounts, got {}",
            expected_accounts.len(),
            listed_accounts.accounts.len()
        ));
    }

    for expected_account in expected_accounts {
        let account = listed_accounts
            .accounts
            .iter()
            .find(|account| account.name == expected_account.name)
            .ok_or(format!("account {} not found", expected_account.name))?;

        if !expected_account.assets.is_empty() && account.addresses.is_empty() {
            return Err(format!(
                "account {} has no addresses, expected some",
                expected_account.name
            ));
        }

        if account.name != expected_account.name
            || account.metadata != expected_account.metadata
            || !compare_arrays(
                &account
                    .assets
                    .iter()
                    .map(|asset| asset.asset_id.clone())
                    .collect::<Vec<String>>(),
                &expected_account.assets,
            )
        {
            return Err(format!(
                "account {} does not match expected account",
                account.id
            ));
        }
    }

    Ok(())
}

fn assert_default_groups_exist(env: &PocketIc, canister_id: Principal, requester: Principal) {
    let listed_user_groups = list_user_groups(env, canister_id, requester)
        .expect("failed to get user groups")
        .0
        .expect("failed to get user groups");

    assert_eq!(listed_user_groups.user_groups.len(), 2);
    assert_eq!(
        listed_user_groups.user_groups[0].id,
        ADMIN_GROUP_ID.hyphenated().to_string()
    );
    assert_eq!(
        listed_user_groups.user_groups[1].id,
        OPERATOR_GROUP_ID.hyphenated().to_string()
    );
}

fn assert_default_policies_and_permissions_exist(
    env: &PocketIc,
    canister_id: Principal,
    requester: Principal,
) {
    let listed_policies = list_request_policies(env, canister_id, requester)
        .expect("failed to get request policies")
        .0
        .expect("failed to get request policies");

    assert!(!listed_policies.policies.is_empty());

    let listed_permissions = list_permissions(env, canister_id, requester)
        .expect("failed to get permissions")
        .0
        .expect("failed to get permissions");

    assert!(!listed_permissions.permissions.is_empty());

    let listed_named_rules = list_named_rules(env, canister_id, requester)
        .expect("failed to get named rules")
        .0
        .expect("failed to get named rules");

    assert!(!listed_named_rules.named_rules.is_empty());
}

fn assert_default_assets_exist(env: &PocketIc, canister_id: Principal, requester: Principal) {
    let listed_assets = list_assets(env, canister_id, requester)
        .expect("failed to get assets")
        .0
        .expect("failed to get assets");

    assert!(!listed_assets.assets.is_empty());
}
