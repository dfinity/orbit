use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::resource::{Resource, ResourceAction},
    services::NamedRuleService,
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    GetNamedRuleInput, GetNamedRuleResponse, ListNamedRulesInput, ListNamedRulesResponse,
    NamedRuleCallerPrivilegesDTO,
};

#[query(name = "get_named_rule")]
async fn get_named_rule(input: GetNamedRuleInput) -> ApiResult<GetNamedRuleResponse> {
    CONTROLLER.get_named_rule(input).await
}

#[query(name = "list_named_rules")]
async fn list_named_rules(input: ListNamedRulesInput) -> ApiResult<ListNamedRulesResponse> {
    CONTROLLER.list_named_rules(input).await
}

lazy_static! {
    static ref CONTROLLER: NamedRuleController =
        NamedRuleController::new(NamedRuleService::default());
}

#[derive(Debug)]
pub struct NamedRuleController {
    named_rule_service: NamedRuleService,
}

impl NamedRuleController {
    pub fn new(named_rule_service: NamedRuleService) -> Self {
        Self { named_rule_service }
    }

    #[with_middleware(guard = authorize(&call_context(),  &[Resource::from(&input)]))]
    async fn get_named_rule(&self, input: GetNamedRuleInput) -> ApiResult<GetNamedRuleResponse> {
        let ctx = call_context();
        let named_rule = self
            .named_rule_service
            .get(HelperMapper::to_uuid(input.named_rule_id)?.as_bytes())?;
        let privileges = self
            .named_rule_service
            .get_caller_privileges_for_named_rule(&named_rule.id, &ctx)
            .await?;

        Ok(GetNamedRuleResponse {
            named_rule: named_rule.into(),
            privileges: privileges.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::NamedRule(ResourceAction::List)]))]
    async fn list_named_rules(
        &self,
        input: ListNamedRulesInput,
    ) -> ApiResult<ListNamedRulesResponse> {
        let ctx = call_context();
        let result = self.named_rule_service.list(input, Some(&ctx))?;
        let mut privileges = Vec::new();

        for rule in &result.items {
            let rule_privileges = self
                .named_rule_service
                .get_caller_privileges_for_named_rule(&rule.id, &ctx)
                .await?;
            privileges.push(NamedRuleCallerPrivilegesDTO::from(rule_privileges));
        }

        Ok(ListNamedRulesResponse {
            named_rules: result.items.into_iter().map(Into::into).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }
}
