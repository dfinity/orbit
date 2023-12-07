use super::{evaluation::Evaluate, CallContext};
use crate::{
    errors::{AccessControlError, EvaluateError},
    models::{
        access_control::{AccessControlPolicy, AccessModifier, Resource},
        specifier::UserSpecifier,
    },
    repositories::{access_control::ACCESS_CONTROL_REPOSITORY, USER_REPOSITORY},
};
use async_trait::async_trait;
use ic_canister_core::repository::Repository;

pub struct AccessControlEvaluator<'ctx> {
    pub call_context: &'ctx CallContext,
    pub resource: Resource,
    pub access_modifier: AccessModifier,
}

impl AccessControlEvaluator<'_> {
    pub fn new<'ctx>(
        call_context: &'ctx CallContext,
        resource: Resource,
        access_modifier: AccessModifier,
    ) -> AccessControlEvaluator<'ctx> {
        AccessControlEvaluator {
            call_context,
            resource,
            access_modifier,
        }
    }
}

#[async_trait]
impl Evaluate<bool> for AccessControlEvaluator<'_> {
    async fn evaluate(&self) -> Result<bool, EvaluateError> {
        if self.call_context.caller_is_controller_or_self() {
            // If the call is made by the system, then the access is granted by default.
            return Ok(true);
        }

        let access_policies = ACCESS_CONTROL_REPOSITORY
            .list()
            .into_iter()
            .filter(|access_control| access_control.resource == self.resource)
            .collect::<Vec<AccessControlPolicy>>();

        if access_policies.is_empty() {
            // If there is no access control policy for the given resource, then the access is denied by default.
            return Ok(false);
        }

        let user = USER_REPOSITORY.find_by_identity(&self.call_context.caller());
        for access_policy in access_policies {
            if access_policy.access != self.access_modifier {
                continue;
            }

            match (access_policy.specifier, user) {
                (UserSpecifier::Any, Some(user)) => {
                    return Ok(true);
                }
                (UserSpecifier::Id(user_ids), Some(user)) => {
                    let user_has_access = user_ids.iter().any(|user_id| {
                        if user.id == *user_id {
                            return true;
                        }
                        false
                    });

                    if user_has_access {
                        return Ok(true);
                    }
                }
                (UserSpecifier::Group(groups), Some(user)) => {
                    let user_has_access = groups.iter().any(|group| {
                        if user.groups.contains(group) {
                            return true;
                        }
                        false
                    });

                    if user_has_access {
                        return Ok(true);
                    }
                }
                _ => {}
            }
        }

        Ok(false)
    }
}

/// This function checks if the user has the required access role to perform the given action.
///
/// It uses the access control policies defined in the canister configuration.
async fn evaluate_caller_access(
    ctx: &CallContext,
    resource: &Resource,
    access_modifier: &AccessModifier,
) -> Result<(), AccessControlError> {
    let evaluator =
        AccessControlEvaluator::new(ctx, resource.to_owned(), access_modifier.to_owned());
    let has_access = evaluator
        .evaluate()
        .await
        .map_err(|e| AccessControlError::UnexpectedError(e.into()))?;

    if !has_access {
        return Err(AccessControlError::Unauthorized {
            resource: resource.to_string(),
            access_modifier: access_modifier.to_string(),
        });
    }

    Ok(())
}
