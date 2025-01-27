use candid::CandidType;
use serde::Deserialize;

use crate::{PaginationInput, RequestPolicyRuleDTO, UuidDTO};

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct NamedRuleDTO {
    pub id: UuidDTO,
    pub name: String,
    pub description: Option<String>,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct AddNamedRuleOperationDTO {
    pub named_rule: Option<NamedRuleDTO>,
    pub input: AddNamedRuleOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct AddNamedRuleOperationInput {
    pub name: String,
    pub description: Option<String>,
    pub rule: RequestPolicyRuleDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct EditNamedRuleOperationDTO {
    pub input: EditNamedRuleOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct EditNamedRuleOperationInput {
    pub named_rule_id: UuidDTO,
    pub name: Option<String>,
    pub description: Option<String>,
    pub rule: Option<RequestPolicyRuleDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RemoveNamedRuleOperationDTO {
    pub input: RemoveNamedRuleOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct RemoveNamedRuleOperationInput {
    pub named_rule_id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListNamedRulesInput {
    pub paginate: Option<PaginationInput>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct ListNamedRulesResponse {
    pub named_rules: Vec<NamedRuleDTO>,
    pub next_offset: Option<u64>,
    pub total: u64,
    pub privileges: Vec<NamedRuleCallerPrivilegesDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct NamedRuleCallerPrivilegesDTO {
    pub id: UuidDTO,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetNamedRuleInput {
    pub named_rule_id: UuidDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Debug, Clone)]
pub struct GetNamedRuleResponse {
    pub named_rule: NamedRuleDTO,
    pub privileges: NamedRuleCallerPrivilegesDTO,
}
