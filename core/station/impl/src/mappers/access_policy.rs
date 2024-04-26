use super::HelperMapper;
use crate::models::{
    access_policy::{AccessPolicy, Allow, AuthScope},
    resource::ResourceIds,
};
use orbit_essentials::types::UUID;
use uuid::Uuid;

impl From<station_api::AuthScopeDTO> for AuthScope {
    fn from(dto: station_api::AuthScopeDTO) -> Self {
        match dto {
            station_api::AuthScopeDTO::Public => AuthScope::Public,
            station_api::AuthScopeDTO::Authenticated => AuthScope::Authenticated,
            station_api::AuthScopeDTO::Restricted => AuthScope::Restricted,
        }
    }
}

impl From<AuthScope> for station_api::AuthScopeDTO {
    fn from(auth: AuthScope) -> Self {
        match auth {
            AuthScope::Public => station_api::AuthScopeDTO::Public,
            AuthScope::Authenticated => station_api::AuthScopeDTO::Authenticated,
            AuthScope::Restricted => station_api::AuthScopeDTO::Restricted,
        }
    }
}

impl From<station_api::AllowDTO> for Allow {
    fn from(dto: station_api::AllowDTO) -> Self {
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

impl From<Allow> for station_api::AllowDTO {
    fn from(allow: Allow) -> Self {
        station_api::AllowDTO {
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

impl From<station_api::ResourceIdsDTO> for ResourceIds {
    fn from(dto: station_api::ResourceIdsDTO) -> Self {
        match dto {
            station_api::ResourceIdsDTO::Any => ResourceIds::Any,
            station_api::ResourceIdsDTO::Ids(ids) => ResourceIds::Ids(
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

impl From<ResourceIds> for station_api::ResourceIdsDTO {
    fn from(id: ResourceIds) -> Self {
        match id {
            ResourceIds::Any => station_api::ResourceIdsDTO::Any,
            ResourceIds::Ids(ids) => station_api::ResourceIdsDTO::Ids(
                ids.iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect(),
            ),
        }
    }
}

impl From<AccessPolicy> for station_api::AccessPolicyDTO {
    fn from(policy: AccessPolicy) -> Self {
        station_api::AccessPolicyDTO {
            resource: policy.resource.into(),
            allow: policy.allow.into(),
        }
    }
}
