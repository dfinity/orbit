use super::HelperMapper;
use crate::models::{
    access_policy::{AccessPolicy, Allow, AuthScope},
    resource::ResourceIds,
};
use ic_canister_core::types::UUID;
use uuid::Uuid;

impl From<wallet_api::AuthScopeDTO> for AuthScope {
    fn from(dto: wallet_api::AuthScopeDTO) -> Self {
        match dto {
            wallet_api::AuthScopeDTO::Public => AuthScope::Public,
            wallet_api::AuthScopeDTO::Authenticated => AuthScope::Authenticated,
            wallet_api::AuthScopeDTO::Restricted => AuthScope::Restricted,
        }
    }
}

impl From<AuthScope> for wallet_api::AuthScopeDTO {
    fn from(auth: AuthScope) -> Self {
        match auth {
            AuthScope::Public => wallet_api::AuthScopeDTO::Public,
            AuthScope::Authenticated => wallet_api::AuthScopeDTO::Authenticated,
            AuthScope::Restricted => wallet_api::AuthScopeDTO::Restricted,
        }
    }
}

impl From<wallet_api::AllowDTO> for Allow {
    fn from(dto: wallet_api::AllowDTO) -> Self {
        Allow {
            auth_scope: dto.auth_scope.into(),
            users: dto
                .users
                .iter()
                .map(|id| {
                    *HelperMapper::to_uuid(id.to_owned())
                        .expect("Invalid user id")
                        .as_bytes()
                })
                .collect(),
            user_groups: dto
                .user_groups
                .iter()
                .map(|id| {
                    *HelperMapper::to_uuid(id.to_owned())
                        .expect("Invalid user group id")
                        .as_bytes()
                })
                .collect(),
        }
    }
}

impl From<Allow> for wallet_api::AllowDTO {
    fn from(allow: Allow) -> Self {
        wallet_api::AllowDTO {
            auth_scope: allow.auth_scope.into(),
            users: allow
                .users
                .iter()
                .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                .collect(),
            user_groups: allow
                .user_groups
                .iter()
                .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                .collect(),
        }
    }
}

impl From<wallet_api::ResourceIdsDTO> for ResourceIds {
    fn from(dto: wallet_api::ResourceIdsDTO) -> Self {
        match dto {
            wallet_api::ResourceIdsDTO::Any => ResourceIds::Any,
            wallet_api::ResourceIdsDTO::Ids(ids) => ResourceIds::Ids(
                ids.iter()
                    .map(|id| {
                        *HelperMapper::to_uuid(id.to_owned())
                            .expect("Invalid resource id")
                            .as_bytes()
                    })
                    .collect::<Vec<UUID>>(),
            ),
        }
    }
}

impl From<ResourceIds> for wallet_api::ResourceIdsDTO {
    fn from(id: ResourceIds) -> Self {
        match id {
            ResourceIds::Any => wallet_api::ResourceIdsDTO::Any,
            ResourceIds::Ids(ids) => wallet_api::ResourceIdsDTO::Ids(
                ids.iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect(),
            ),
        }
    }
}

impl From<AccessPolicy> for wallet_api::AccessPolicyDTO {
    fn from(policy: AccessPolicy) -> Self {
        wallet_api::AccessPolicyDTO {
            resource: policy.resource.into(),
            allow: policy.allow.into(),
        }
    }
}
