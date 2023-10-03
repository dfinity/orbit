use super::canister_config;
use crate::{models::AccessRole, services::AccountService};
use candid::Principal;
use ic_canister_core::cdk::{api::trap, caller};

#[derive(Clone, Debug)]
pub struct CallContext {
    caller: Principal,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            caller: Principal::anonymous(),
        }
    }
}

impl CallContext {
    /// This method can only be used before any await has been called in the current call context,
    /// otherwise it will panic.
    pub fn get() -> Self {
        Self { caller: caller() }
    }

    pub fn caller(&self) -> Principal {
        self.caller
    }

    /// Checks if the caller is an admin.
    pub fn is_admin(&self) -> bool {
        let account = AccountService::default().find_account_by_identity(&self.caller);

        match account {
            Some(account) => account.access_roles.contains(&AccessRole::Admin),
            None => false,
        }
    }

    /// Checks if the caller has the required access role to perform the given action.
    pub fn check_access(&self, permission: &str) {
        check_access(permission, self.caller.to_owned())
    }
}

/// This function checks if the user has the required access role to perform the given action.
pub fn check_access(permission: &str, caller: Principal) {
    let permissions = canister_config().permissions;
    let permission = permissions
        .iter()
        .find(|p| p.permission_id == permission)
        .unwrap_or_else(|| trap(format!("Permission {} not found", permission).as_str()));

    if permission.access_roles.contains(&AccessRole::Guest) {
        return;
    }

    let account = AccountService::default()
        .find_account_by_identity(&caller)
        .unwrap_or_else(|| {
            trap(
                format!(
                    "Access denied for user with principal `{}`",
                    caller.to_text()
                )
                .as_str(),
            )
        });

    if account.access_roles.contains(&AccessRole::Admin) {
        // Admins have access to everything
        return;
    }

    let user_has_access = permission
        .access_roles
        .iter()
        .any(|required_role| account.access_roles.contains(required_role));

    if !user_has_access {
        trap(
            format!(
                "Access denied for user with principal `{}`",
                caller.to_text()
            )
            .as_str(),
        );
    }
}
