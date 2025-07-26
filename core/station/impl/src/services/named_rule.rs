use std::sync::Arc;

use lazy_static::lazy_static;
use orbit_essentials::{
    api::ServiceResult,
    model::{ModelKey, ModelValidator},
    pagination::{paginated_items, PaginatedData, PaginatedItemsArgs},
    repository::Repository,
    types::UUID,
};
use station_api::ListNamedRulesInput;
use uuid::Uuid;

use crate::{
    core::{authorization::Authorization, utils::retain_accessible_resources, CallContext},
    errors::NamedRuleError,
    models::{
        resource::{Resource, ResourceAction, ResourceId},
        AddNamedRuleOperationInput, EditNamedRuleOperationInput, NamedRule,
        NamedRuleCallerPrivileges, NamedRuleId, NamedRuleKey, RemoveNamedRuleOperationInput,
    },
    repositories::{
        NamedRuleRepository, RequestPolicyRepository, NAMED_RULE_REPOSITORY,
        REQUEST_POLICY_REPOSITORY,
    },
};

lazy_static! {
    pub static ref NAMED_RULE_SERVICE: Arc<NamedRuleService> = Arc::new(NamedRuleService::new(
        Arc::clone(&NAMED_RULE_REPOSITORY),
        Arc::clone(&REQUEST_POLICY_REPOSITORY)
    ));
}

#[derive(Default, Debug)]
pub struct NamedRuleService {
    named_rule_repository: Arc<NamedRuleRepository>,
    request_policy_repository: Arc<RequestPolicyRepository>,
}

impl NamedRuleService {
    pub const DEFAULT_LIST_NAMED_RULES_LIMIT: u16 = 100;
    pub const MAX_LIST_NAMED_RULES_LIMIT: u16 = 1000;

    pub fn new(
        named_rule_repository: Arc<NamedRuleRepository>,
        request_policy_repository: Arc<RequestPolicyRepository>,
    ) -> Self {
        Self {
            named_rule_repository,
            request_policy_repository,
        }
    }

    pub fn get(&self, named_rule_id: &NamedRuleId) -> ServiceResult<NamedRule> {
        let named_rule = self
            .named_rule_repository
            .get(&NamedRuleKey { id: *named_rule_id })
            .ok_or(NamedRuleError::NotFound {
                id: Uuid::from_bytes(*named_rule_id).hyphenated().to_string(),
            })?;

        Ok(named_rule)
    }

    pub fn list(
        &self,
        input: ListNamedRulesInput,
        ctx: Option<&CallContext>,
    ) -> ServiceResult<PaginatedData<NamedRule>> {
        let mut named_rules = self.named_rule_repository.list();

        if let Some(ctx) = ctx {
            // filter out named rules that the caller does not have access to read
            retain_accessible_resources(ctx, &mut named_rules, |named_rule| {
                Resource::NamedRule(crate::models::resource::ResourceAction::Read(
                    crate::models::resource::ResourceId::Id(named_rule.id),
                ))
            });
        }

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_LIST_NAMED_RULES_LIMIT),
            max_limit: Some(Self::MAX_LIST_NAMED_RULES_LIMIT),
            items: &named_rules,
        })?;

        Ok(result)
    }

    pub fn create_with_id(
        &self,
        input: AddNamedRuleOperationInput,
        with_named_rule_id: Option<UUID>,
    ) -> ServiceResult<NamedRule> {
        let id = with_named_rule_id.unwrap_or_else(|| *Uuid::new_v4().as_bytes());

        if self
            .named_rule_repository
            .get(&NamedRuleKey { id })
            .is_some()
        {
            Err(NamedRuleError::IdAlreadyExists {
                id: Uuid::from_bytes(id).hyphenated().to_string(),
            })?;
        }

        let named_rule = NamedRule {
            id,
            name: input.name,
            description: input.description,
            rule: input.rule,
        };

        named_rule.validate()?;

        self.named_rule_repository
            .insert(named_rule.key(), named_rule.clone());

        Ok(named_rule)
    }

    pub fn create(&self, input: AddNamedRuleOperationInput) -> ServiceResult<NamedRule> {
        self.create_with_id(input, None)
    }

    fn is_named_rule_in_use_by_request_policies(&self, named_rule_id: &NamedRuleId) -> bool {
        self.request_policy_repository
            .list()
            .iter()
            .any(|request_policy| request_policy.rule.has_named_rule_id(named_rule_id))
    }

    fn is_named_rule_in_use_by_named_rules(&self, named_rule_id: &NamedRuleId) -> bool {
        self.named_rule_repository
            .list()
            .iter()
            .any(|named_rule| named_rule.rule.has_named_rule_id(named_rule_id))
    }

    pub fn remove(&self, input: RemoveNamedRuleOperationInput) -> ServiceResult<NamedRule> {
        let named_rule = self.get(&input.named_rule_id)?;

        if self.is_named_rule_in_use_by_named_rules(&input.named_rule_id)
            | self.is_named_rule_in_use_by_request_policies(&input.named_rule_id)
        {
            return Err(NamedRuleError::InUse)?;
        }

        self.named_rule_repository.remove(&NamedRuleKey {
            id: input.named_rule_id,
        });

        Ok(named_rule)
    }

    pub fn edit(&self, input: EditNamedRuleOperationInput) -> ServiceResult<NamedRule> {
        let mut named_rule = self.get(&input.named_rule_id)?;

        if let Some(name) = input.name {
            named_rule.name = name;
        }

        if let Some(description) = input.description {
            named_rule.description = description;
        }

        if let Some(rule) = input.rule {
            named_rule.rule = rule;
        }

        named_rule.validate()?;

        self.named_rule_repository
            .insert(named_rule.key(), named_rule.clone());

        Ok(named_rule)
    }

    pub async fn get_caller_privileges_for_named_rule(
        &self,
        named_rule_id: &NamedRuleId,
        ctx: &CallContext,
    ) -> ServiceResult<NamedRuleCallerPrivileges> {
        Ok(NamedRuleCallerPrivileges {
            id: *named_rule_id,
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::NamedRule(ResourceAction::Update(ResourceId::Id(*named_rule_id))),
            ),
            can_delete: Authorization::is_allowed(
                ctx,
                &Resource::NamedRule(ResourceAction::Delete(ResourceId::Id(*named_rule_id))),
            ),
        })
    }
}

#[cfg(test)]
mod test {
    use orbit_essentials::{model::ModelKey, repository::Repository};

    use crate::{
        models::{
            AddNamedRuleOperationInput, EditNamedRuleOperationInput, NamedRule, RequestPolicy,
            RequestPolicyRule,
        },
        repositories::{NAMED_RULE_REPOSITORY, REQUEST_POLICY_REPOSITORY},
        services::NAMED_RULE_SERVICE,
    };

    #[test]
    fn test_get() {
        let named_rule = NamedRule {
            id: [0; 16],
            name: "test".to_string(),
            description: Some("test description".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };
        NAMED_RULE_REPOSITORY.insert(named_rule.key(), named_rule.clone());
        let result = NAMED_RULE_SERVICE.get(&named_rule.id);
        assert_eq!(result.unwrap(), named_rule);
    }

    #[test]
    fn test_create() {
        let input = crate::models::AddNamedRuleOperationInput {
            name: "test".to_string(),
            description: Some("test description".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };

        let result = NAMED_RULE_SERVICE.create(input.clone());
        assert!(result.is_ok());

        let named_rule = result.unwrap();
        assert_eq!(named_rule.name, input.name);
        assert_eq!(named_rule.description, input.description);
        assert_eq!(named_rule.rule, input.rule);
    }

    #[test]
    fn test_create_with_circular_reference() {
        let named_rule_id = [0; 16];
        let input = crate::models::AddNamedRuleOperationInput {
            name: "test".to_string(),
            description: Some("test description".to_string()),
            rule: RequestPolicyRule::NamedRule(named_rule_id),
        };

        let named_rule = NamedRule {
            id: named_rule_id,
            name: "circular".to_string(),
            description: Some("circular description".to_string()),
            rule: RequestPolicyRule::NamedRule(named_rule_id),
        };

        NAMED_RULE_REPOSITORY.insert(named_rule.key(), named_rule.clone());

        let result = NAMED_RULE_SERVICE.create(input.clone());
        assert!(result.is_err());
    }

    #[test]
    fn test_edit() {
        let named_rule = NamedRule {
            id: [0; 16],
            name: "test".to_string(),
            description: Some("test description".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };
        NAMED_RULE_REPOSITORY.insert(named_rule.key(), named_rule.clone());

        let input = crate::models::EditNamedRuleOperationInput {
            named_rule_id: named_rule.id,
            name: Some("edited".to_string()),
            description: Some(Some("edited description".to_string())),
            rule: Some(RequestPolicyRule::AutoApproved),
        };

        let result = NAMED_RULE_SERVICE.edit(input.clone());
        assert!(result.is_ok());

        let edited_named_rule = result.unwrap();
        assert_eq!(edited_named_rule.name, input.name.unwrap());
        assert_eq!(edited_named_rule.description, input.description.unwrap());
        assert_eq!(edited_named_rule.rule, input.rule.unwrap());
    }

    #[test]
    fn test_edit_with_circular_reference() {
        let named_rule_id = [0; 16];
        let named_rule = NamedRule {
            id: named_rule_id,
            name: "test".to_string(),
            description: Some("test description".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };
        NAMED_RULE_REPOSITORY.insert(named_rule.key(), named_rule.clone());

        let input = crate::models::EditNamedRuleOperationInput {
            named_rule_id: named_rule.id,
            name: Some("edited".to_string()),
            description: Some(Some("edited description".to_string())),
            rule: Some(RequestPolicyRule::NamedRule(named_rule_id)),
        };

        let result = NAMED_RULE_SERVICE.edit(input.clone());
        assert!(result.is_err());
    }

    #[test]
    fn test_remove() {
        let named_rule = NamedRule {
            id: [0; 16],
            name: "test".to_string(),
            description: Some("test description".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };
        NAMED_RULE_REPOSITORY.insert(named_rule.key(), named_rule.clone());

        // Attempt to remove the named rule that is not in use
        let result = NAMED_RULE_SERVICE.remove(crate::models::RemoveNamedRuleOperationInput {
            named_rule_id: named_rule.id,
        });
        assert!(result.is_ok());

        // Insert the named rule again
        NAMED_RULE_REPOSITORY.insert(named_rule.key(), named_rule.clone());

        // Simulate the named rule being in use by another rule
        let in_use_rule = NamedRule {
            id: [1; 16],
            name: "in use".to_string(),
            description: Some("in use description".to_string()),
            rule: RequestPolicyRule::NamedRule(named_rule.id),
        };
        NAMED_RULE_REPOSITORY.insert(in_use_rule.key(), in_use_rule.clone());

        // Attempt to remove the named rule that is in use
        let result = NAMED_RULE_SERVICE.remove(crate::models::RemoveNamedRuleOperationInput {
            named_rule_id: named_rule.id,
        });

        assert_eq!(
            result
                .expect_err("The named rule should not be removed because it is in use.")
                .code,
            "IN_USE"
        );

        NAMED_RULE_REPOSITORY.remove(&in_use_rule.key());

        let policy = RequestPolicy {
            id: [0; 16],
            rule: RequestPolicyRule::NamedRule(named_rule.id),
            specifier: crate::models::request_specifier::RequestSpecifier::AddUser,
        };

        REQUEST_POLICY_REPOSITORY.insert(policy.key(), policy.clone());

        // Attempt to remove the named rule that is in use
        let result = NAMED_RULE_SERVICE.remove(crate::models::RemoveNamedRuleOperationInput {
            named_rule_id: named_rule.id,
        });

        assert_eq!(
            result
                .expect_err("The named rule should not be removed because it is in use.")
                .code,
            "IN_USE"
        );

        REQUEST_POLICY_REPOSITORY.remove(&policy.key());

        // Attempt to remove the named rule that is not in use
        let result = NAMED_RULE_SERVICE.remove(crate::models::RemoveNamedRuleOperationInput {
            named_rule_id: named_rule.id,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_nonexistent_named_rule() {
        let non_existent_id = [99; 16];
        let result = NAMED_RULE_SERVICE.get(&non_existent_id);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "NOT_FOUND");
    }

    #[test]
    fn test_remove_nonexistent_named_rule() {
        let non_existent_id = [98; 16];
        let result = NAMED_RULE_SERVICE.remove(crate::models::RemoveNamedRuleOperationInput {
            named_rule_id: non_existent_id,
        });
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "NOT_FOUND");
    }

    #[test]
    fn test_create_with_empty_name() {
        let input = AddNamedRuleOperationInput {
            name: "".to_string(),
            description: Some("no name".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };
        let result = NAMED_RULE_SERVICE.create(input);
        assert!(
            result.is_err(),
            "Creating a named rule with an empty name should fail"
        );
        let err = result.unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[test]
    fn test_edit_nonexistent_named_rule() {
        let non_existent_id = [97; 16];
        let input = EditNamedRuleOperationInput {
            named_rule_id: non_existent_id,
            name: Some("should fail".to_string()),
            description: None,
            rule: Some(RequestPolicyRule::AutoApproved),
        };
        let result = NAMED_RULE_SERVICE.edit(input);
        assert!(
            result.is_err(),
            "Editing a non-existent named rule should fail"
        );
        let err = result.unwrap_err();
        assert_eq!(err.code, "NOT_FOUND");
    }
}
