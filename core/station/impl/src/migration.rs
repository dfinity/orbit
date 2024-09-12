use std::collections::BTreeSet;

use orbit_essentials::model::ModelKey;
use orbit_essentials::repository::Repository;
use uuid::Uuid;

use crate::core::ic_cdk::api::trap;
use crate::core::{read_system_info, write_system_info};

use crate::models::permission::AuthScope;
use crate::models::request_specifier::RequestSpecifier;
use crate::models::resource::{
    Resource, ResourceAction, ResourceId, ResourceIds, SystemResourceAction,
};
use crate::models::{
    AddRequestPolicyOperationInput, Asset, Blockchain, BlockchainStandard,
    EditPermissionOperationInput, Metadata,
};
use crate::repositories::permission::PERMISSION_REPOSITORY;
use crate::repositories::{
    ASSET_REPOSITORY, REQUEST_POLICY_REPOSITORY, USER_GROUP_REPOSITORY, USER_REPOSITORY,
};
use crate::services::permission::PERMISSION_SERVICE;
use crate::services::REQUEST_POLICY_SERVICE;
use crate::STABLE_MEMORY_VERSION;

/// Handles stable memory schema migrations for the station canister.
///
/// Stable memory migration conditions:
///
/// - The migration is only applied once per each version.
/// - Stable memory versions can't be skipped, the upgrade must be sequential.
/// - The migration is applied is the previous version is `STABLE_MEMORY_VERSION - 1`.
pub struct MigrationHandler;

impl MigrationHandler {
    /// Run migrations for the station canister to ensure the stable memory schema is up-to-date.
    ///
    /// WARNING: This needs to be run before any other access to stable memory happens.
    pub fn run() {
        let mut system_info = read_system_info();
        let stored_version = system_info.get_stable_memory_version();

        if stored_version == STABLE_MEMORY_VERSION {
            return;
        }

        if stored_version > STABLE_MEMORY_VERSION {
            trap(&format!(
                "Cannot downgrade the station from memory layout version {} to {}",
                stored_version, STABLE_MEMORY_VERSION
            ));
        }

        apply_migration();

        // Update the stable memory version to the latest version.
        system_info.set_stable_memory_version(STABLE_MEMORY_VERSION);
        write_system_info(system_info);
    }
}

/// The migration to apply to the station canister stable memory.
///
/// Please include the migration steps in the `apply_migration` function.
fn apply_migration() {
    // add new asset permissions: resources available to all users
    let public_resources = [
        Resource::Asset(ResourceAction::List),
        Resource::Asset(ResourceAction::Read(ResourceId::Any)),
    ];

    // build cache so that model validation can pass
    USER_GROUP_REPOSITORY.build_cache();
    USER_REPOSITORY.build_cache();
    PERMISSION_REPOSITORY.build_cache();

    for resource in public_resources {
        let _ = PERMISSION_SERVICE.edit_permission(EditPermissionOperationInput {
            resource,
            auth_scope: Some(AuthScope::Authenticated),
            user_groups: None,
            users: None,
        });
    }

    // add new asset permissions: inherit config from ManageSystemInfo
    let manage_system_info_permissions_allow = PERMISSION_SERVICE
        .get_permission(&Resource::System(SystemResourceAction::ManageSystemInfo))
        .allow;

    let sensitive_resources = [
        Resource::Asset(ResourceAction::Create),
        Resource::Asset(ResourceAction::Update(ResourceId::Any)),
        Resource::Asset(ResourceAction::Delete(ResourceId::Any)),
    ];

    for resource in sensitive_resources {
        if let Err(err) = PERMISSION_SERVICE.edit_permission(EditPermissionOperationInput {
            resource,
            auth_scope: Some(manage_system_info_permissions_allow.auth_scope.clone()),
            user_groups: Some(manage_system_info_permissions_allow.user_groups.clone()),
            users: Some(manage_system_info_permissions_allow.users.clone()),
        }) {
            ic_cdk::println!("Failed to create new asset permission: {:?}", err);
        }
    }

    // add new asset policies
    let policy_specifiers = [
        RequestSpecifier::AddAsset,
        RequestSpecifier::EditAsset(ResourceIds::Any),
        RequestSpecifier::RemoveAsset(ResourceIds::Any),
    ];

    let policies_to_copy = REQUEST_POLICY_REPOSITORY
        .find_by_resource(Resource::System(SystemResourceAction::ManageSystemInfo));

    for policy in policies_to_copy {
        for specifier in policy_specifiers.iter() {
            if let Err(err) =
                REQUEST_POLICY_SERVICE.add_request_policy(AddRequestPolicyOperationInput {
                    specifier: specifier.clone(),
                    rule: policy.rule.clone(),
                })
            {
                ic_cdk::println!("Failed to create new asset policy: {:?}", err);
            }
        }
    }

    // add ICP asset
    let icp_asset = Asset {
        blockchain: Blockchain::InternetComputer,
        decimals: 8,
        id: Uuid::new_v4().as_bytes().to_owned(),
        metadata: Metadata::default(),
        name: "Internet Computer".to_string(),
        standards: BTreeSet::from([BlockchainStandard::Native]),
        symbol: "ICP".to_string(),
    };

    ASSET_REPOSITORY.insert(icp_asset.key(), icp_asset);
}
