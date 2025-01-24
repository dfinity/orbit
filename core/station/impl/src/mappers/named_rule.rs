use station_api::NamedRuleDTO;
use uuid::Uuid;

use crate::models::{NamedRule, NamedRuleCallerPrivileges};

impl From<NamedRule> for NamedRuleDTO {
    fn from(rule: NamedRule) -> Self {
        NamedRuleDTO {
            id: Uuid::from_bytes(rule.id).hyphenated().to_string(),
            name: rule.name,
            description: rule.description,
            rule: rule.rule.into(),
        }
    }
}

impl From<NamedRuleCallerPrivileges> for station_api::NamedRuleCallerPrivilegesDTO {
    fn from(input: NamedRuleCallerPrivileges) -> station_api::NamedRuleCallerPrivilegesDTO {
        station_api::NamedRuleCallerPrivilegesDTO {
            id: Uuid::from_bytes(input.id).hyphenated().to_string(),
            can_edit: input.can_edit,
            can_delete: input.can_delete,
        }
    }
}
