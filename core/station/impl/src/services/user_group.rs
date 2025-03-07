use super::SystemService;
use crate::core::authorization::Authorization;
use crate::core::ic_cdk::next_time;
use crate::core::utils::{
    paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs,
};
use crate::core::{generate_uuid_v4, CallContext};
use crate::errors::UserGroupError;
use crate::models::resource::{Resource, ResourceAction, ResourceId};
use crate::models::{
    AddUserGroupOperationInput, EditUserGroupOperationInput, UserGroup, UserGroupCallerPrivileges,
};
use crate::repositories::{UseGroupWhereClause, UserGroupRepository};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::model::ModelValidator;
use orbit_essentials::repository::Repository;
use orbit_essentials::types::UUID;
use station_api::ListUserGroupsInput;
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref USER_GROUP_SERVICE: Arc<UserGroupService> =
        Arc::new(UserGroupService::default());
}

#[derive(Default, Debug)]
pub struct UserGroupService {
    system_service: Arc<SystemService>,
    user_group_repository: Arc<UserGroupRepository>,
}

impl UserGroupService {
    const DEFAULT_USER_GROUP_LIST_LIMIT: u16 = 100;
    const MAX_USER_GROUP_LIST_LIMIT: u16 = 1000;

    pub fn get(&self, user_group_id: &UUID) -> ServiceResult<UserGroup> {
        let user_group =
            self.user_group_repository
                .get(user_group_id)
                .ok_or(UserGroupError::NotFound {
                    id: Uuid::from_bytes(*user_group_id).hyphenated().to_string(),
                })?;

        Ok(user_group)
    }

    pub async fn get_caller_privileges_for_user_group(
        &self,
        user_group_id: &UUID,
        ctx: &CallContext,
    ) -> ServiceResult<UserGroupCallerPrivileges> {
        Ok(UserGroupCallerPrivileges {
            id: *user_group_id,
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::UserGroup(ResourceAction::Update(ResourceId::Id(*user_group_id))),
            ),
            can_delete: Authorization::is_allowed(
                ctx,
                &Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(*user_group_id))),
            ),
        })
    }

    pub async fn list(
        &self,
        input: ListUserGroupsInput,
        ctx: Option<&CallContext>,
    ) -> ServiceResult<PaginatedData<UserGroup>> {
        let mut user_groups = self.user_group_repository.find_where(UseGroupWhereClause {
            search_term: input.search_term.to_owned(),
        });

        // filter out user groups that the caller does not have access to read
        if let Some(ctx) = ctx {
            retain_accessible_resources(ctx, &mut user_groups, |user_group| {
                Resource::UserGroup(ResourceAction::Read(ResourceId::Id(user_group.id)))
            });
        }

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_USER_GROUP_LIST_LIMIT),
            max_limit: Some(Self::MAX_USER_GROUP_LIST_LIMIT),
            items: &user_groups,
        })?;

        Ok(result)
    }

    pub async fn create(&self, input: AddUserGroupOperationInput) -> ServiceResult<UserGroup> {
        self.create_with_id(input, None).await
    }

    pub async fn create_with_id(
        &self,
        input: AddUserGroupOperationInput,
        with_user_group_id: Option<UUID>,
    ) -> ServiceResult<UserGroup> {
        let user_group_id = match with_user_group_id {
            Some(id) => Uuid::from_bytes(id),
            None => generate_uuid_v4().await,
        };

        let user_group = UserGroup {
            id: *user_group_id.as_bytes(),
            name: input.name.to_string(),
            last_modification_timestamp: next_time(),
        };

        user_group.validate()?;

        self.user_group_repository
            .insert(user_group.id, user_group.clone());

        Ok(user_group)
    }

    pub async fn edit(&self, input: EditUserGroupOperationInput) -> ServiceResult<UserGroup> {
        let mut user_group = self.get(&input.user_group_id)?;

        user_group.name = input.name.to_string();
        user_group.last_modification_timestamp = next_time();

        user_group.validate()?;

        self.user_group_repository
            .insert(user_group.id, user_group.clone());

        Ok(user_group)
    }

    pub async fn remove(&self, id: &UUID) -> ServiceResult<()> {
        let user_group = self.get(id)?;

        let system_info = self.system_service.get_system_info();

        if let Some(committee) = system_info.get_disaster_recovery_committee() {
            if committee.user_group_id == user_group.id {
                return Err(UserGroupError::CannotDeleteDisasterRecoveryCommittee {
                    id: Uuid::from_bytes(user_group.id).hyphenated().to_string(),
                }
                .into());
            }
        }

        self.user_group_repository.remove(&user_group.id);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        core::write_system_info,
        models::{AddUserGroupOperationInput, SystemInfo},
    };

    use super::USER_GROUP_SERVICE;

    #[tokio::test]
    async fn test_deleting_disaster_recovery_committee_user_group() {
        let user_group = USER_GROUP_SERVICE
            .create(AddUserGroupOperationInput {
                name: "Test".to_string(),
            })
            .await
            .expect("Failed to create user group");

        let mut system_info = SystemInfo::default();

        system_info.set_disaster_recovery_committee(Some(
            crate::models::DisasterRecoveryCommittee {
                user_group_id: user_group.id,
                quorum: 1,
            },
        ));

        write_system_info(system_info);

        let result = USER_GROUP_SERVICE.remove(&user_group.id).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert_eq!(error.code, "CANNOT_DELETE_DISASTER_RECOVERY_COMMITTEE");
    }
}
