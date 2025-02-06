use std::collections::HashSet;

use orbit_essentials::{
    model::{ModelKey, ModelValidator, ModelValidatorResult},
    repository::Repository,
    storable,
    types::UUID,
};

use crate::{
    core::utils::format_unique_string, errors::NamedRuleError, repositories::NAMED_RULE_REPOSITORY,
};

use super::{indexes::unique_index::UniqueIndexKey, RequestPolicyRule};

pub type NamedRuleId = UUID;

#[storable]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NamedRule {
    pub id: NamedRuleId,
    pub name: String,
    pub description: Option<String>,
    pub rule: RequestPolicyRule,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NamedRuleKey {
    /// The named rule id, which is a UUID.
    pub id: NamedRuleId,
}

impl ModelKey<NamedRuleKey> for NamedRule {
    fn key(&self) -> NamedRuleKey {
        NamedRuleKey { id: self.id }
    }
}

impl NamedRule {
    pub const MIN_NAME_LENGTH: u8 = 1;
    pub const MAX_NAME_LENGTH: u8 = 64;

    pub const MIN_DESCRIPTION_LENGTH: u8 = 0;
    pub const MAX_DESCRIPTION_LENGTH: u8 = 255;

    pub fn to_unique_index(&self) -> (UniqueIndexKey, UUID) {
        (Self::to_unique_index_key(&self.name), self.id)
    }

    pub fn to_unique_index_key(name: &str) -> UniqueIndexKey {
        UniqueIndexKey::NamedRuleName(format_unique_string(name))
    }
}

fn validate_uniqueness(
    named_rule_id: &NamedRuleId,
    name: &str,
) -> ModelValidatorResult<NamedRuleError> {
    if let Some(existing_named_rule_id) = NAMED_RULE_REPOSITORY.exists_unique(name) {
        if existing_named_rule_id != *named_rule_id {
            return Err(NamedRuleError::AlreadyExists {
                name: name.to_string(),
            });
        }
    }

    Ok(())
}

enum NamedRuleInputBy {
    Value(NamedRule),
    Id(NamedRuleId),
}

fn find_circular_reference(
    current: NamedRuleInputBy,
    visited: &mut HashSet<NamedRuleId>,
    branch: &mut HashSet<NamedRuleId>,
) -> bool {
    let current_id = match &current {
        NamedRuleInputBy::Value(named_rule) => named_rule.id,
        NamedRuleInputBy::Id(id) => *id,
    };

    if !visited.contains(&current_id) {
        visited.insert(current_id);
        branch.insert(current_id);

        if let Some(named_rule) = match &current {
            NamedRuleInputBy::Value(named_rule) => Some(named_rule.clone()),
            NamedRuleInputBy::Id(id) => NAMED_RULE_REPOSITORY.get(&NamedRuleKey { id: *id }),
        } {
            for child_id in collect_child_ids(&named_rule.rule) {
                if branch.contains(&child_id)
                    || !visited.contains(&child_id)
                        && find_circular_reference(NamedRuleInputBy::Id(child_id), visited, branch)
                {
                    return true;
                }
            }
        }
    }
    branch.remove(&current_id);
    false
}

fn collect_child_ids(rule: &RequestPolicyRule) -> Vec<NamedRuleId> {
    let mut ids = Vec::new();
    match rule {
        RequestPolicyRule::And(rules) | RequestPolicyRule::Or(rules) => {
            for r in rules {
                ids.extend(collect_child_ids(r));
            }
        }
        RequestPolicyRule::NamedRule(id) => ids.push(*id),
        RequestPolicyRule::AutoApproved
        | RequestPolicyRule::QuorumPercentage(..)
        | RequestPolicyRule::Quorum(..)
        | RequestPolicyRule::AllowListedByMetadata(..)
        | RequestPolicyRule::AllowListed => {}
        RequestPolicyRule::Not(request_policy_rule) => {
            ids.extend(collect_child_ids(request_policy_rule));
        }
    }
    ids
}

/// Validates that the named rule does not have a circular reference.
fn validate_circular_reference(rule: &NamedRule) -> ModelValidatorResult<NamedRuleError> {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    if find_circular_reference(
        NamedRuleInputBy::Value(rule.clone()),
        &mut visited,
        &mut stack,
    ) {
        return Err(NamedRuleError::CircularReference);
    }
    Ok(())
}

fn validate_name(name: &str) -> ModelValidatorResult<NamedRuleError> {
    if name.len() < NamedRule::MIN_NAME_LENGTH as usize
        || name.len() > NamedRule::MAX_NAME_LENGTH as usize
    {
        return Err(NamedRuleError::InvalidName {
            min_length: NamedRule::MIN_NAME_LENGTH as usize,
            max_length: NamedRule::MAX_NAME_LENGTH as usize,
        });
    }

    Ok(())
}

fn validate_description(description: &Option<String>) -> ModelValidatorResult<NamedRuleError> {
    if let Some(description) = description {
        if description.len() < NamedRule::MIN_DESCRIPTION_LENGTH as usize
            || description.len() > NamedRule::MAX_DESCRIPTION_LENGTH as usize
        {
            return Err(NamedRuleError::InvalidDescription {
                min_length: NamedRule::MIN_DESCRIPTION_LENGTH as usize,
                max_length: NamedRule::MAX_DESCRIPTION_LENGTH as usize,
            });
        }
    }

    Ok(())
}

impl ModelValidator<NamedRuleError> for NamedRule {
    fn validate(&self) -> ModelValidatorResult<NamedRuleError> {
        validate_name(&self.name)?;
        validate_description(&self.description)?;

        self.rule
            .validate()
            .map_err(|e| NamedRuleError::InvalidRule {
                error: e.to_string(),
            })?;

        validate_uniqueness(&self.id, &self.name)?;
        validate_circular_reference(self)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct NamedRuleCallerPrivileges {
    pub id: NamedRuleId,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[cfg(test)]
mod test {
    use orbit_essentials::{
        model::{ModelKey, ModelValidator},
        repository::Repository,
    };

    use crate::{
        errors::NamedRuleError,
        models::{NamedRule, RequestPolicyRule},
        repositories::NAMED_RULE_REPOSITORY,
    };

    use super::{validate_description, validate_name};

    #[test]
    fn test_name_validation() {
        assert!(validate_name("").is_err());
        assert!(
            validate_name("a".repeat(NamedRule::MAX_NAME_LENGTH as usize + 1).as_str()).is_err()
        );
        assert!(validate_name("a").is_ok());
    }

    #[test]
    fn test_description_validation() {
        assert!(validate_description(&Some(
            "a".repeat(NamedRule::MAX_DESCRIPTION_LENGTH as usize + 1)
        ))
        .is_err());
        assert!(validate_description(&Some(
            "a".repeat(NamedRule::MAX_DESCRIPTION_LENGTH as usize)
        ))
        .is_ok());
        assert!(validate_description(&None).is_ok());
    }

    #[test]
    fn test_named_rule() {
        let named_rule = NamedRule {
            id: [0; 16],
            name: "test".to_string(),
            description: Some("test".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };

        assert!(named_rule.validate().is_ok());
    }

    #[test]
    fn test_self_referencing_rule() {
        let mut named_rule = NamedRule {
            id: [0; 16],
            name: "test".to_string(),
            description: Some("test".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };

        NAMED_RULE_REPOSITORY.insert(named_rule.key(), named_rule.clone());

        named_rule.rule = RequestPolicyRule::NamedRule(named_rule.id);
        assert!(matches!(
            named_rule.validate(),
            Err(NamedRuleError::CircularReference)
        ));

        named_rule.rule = RequestPolicyRule::And(vec![RequestPolicyRule::NamedRule(named_rule.id)]);
        assert!(matches!(
            named_rule.validate(),
            Err(NamedRuleError::CircularReference)
        ));
    }

    #[test]
    fn test_circular_referencing_rule() {
        let mut named_rule_1 = NamedRule {
            id: [0; 16],
            name: "test1".to_string(),
            description: Some("test".to_string()),
            rule: RequestPolicyRule::AutoApproved,
        };

        let named_rule_2 = NamedRule {
            id: [1; 16],
            name: "test2".to_string(),
            description: Some("test".to_string()),
            rule: RequestPolicyRule::NamedRule(named_rule_1.id),
        };

        let named_rule_3 = NamedRule {
            id: [2; 16],
            name: "test3".to_string(),
            description: Some("test".to_string()),
            rule: RequestPolicyRule::NamedRule(named_rule_2.id),
        };

        NAMED_RULE_REPOSITORY.insert(named_rule_1.key(), named_rule_1.clone());
        NAMED_RULE_REPOSITORY.insert(named_rule_2.key(), named_rule_2.clone());
        NAMED_RULE_REPOSITORY.insert(named_rule_3.key(), named_rule_3.clone());

        named_rule_1
            .validate()
            .expect("Named rule 1 should be valid.");
        named_rule_2
            .validate()
            .expect("Named rule 2 should be valid.");
        named_rule_3
            .validate()
            .expect("Named rule 3 should be valid.");

        named_rule_1.rule = RequestPolicyRule::NamedRule(named_rule_3.id);

        assert!(matches!(
            named_rule_1.validate(),
            Err(NamedRuleError::CircularReference)
        ));
    }
}
