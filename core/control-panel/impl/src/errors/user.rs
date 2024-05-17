use control_panel_api::UserSubscriptionStatusDTO;
use orbit_essentials::api::DetailableError;
use thiserror::Error;

/// Container for station errors.
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum UserError {
    /// The identity already has an associated user.
    #[error(r#"The identity already has an associated user."#)]
    IdentityAlreadyHasUser {
        /// The associated user of the identity.
        user: String,
    },
    /// An user associated with the identity was not found.
    #[error(r#"A user associated with the identity was not found."#)]
    AssociatedUserIdentityNotFound {
        /// The given identity.
        identity: String,
    },
    /// The requested user was not found.
    #[error(r#"The requested user was not found."#)]
    NotFound {
        /// The requested user.
        user: String,
    },
    /// You don't have permission to access the requested user.
    #[error(r#"You don't have permission to access the requested user."#)]
    Forbidden {
        /// The requested user.
        user: String,
    },
    /// The user has failed validation.
    #[error(r#"The user has failed validation."#)]
    ValidationError { info: String },
    /// The deploy station quota was exceeded.
    #[error(r#"Deploy station quota exceeded."#)]
    DeployStationQuotaExceeded,
    /// The user has an inappropriate subscription status for the operation.
    #[error(r#"The user has an inappropriate subscription status for the operation."#)]
    BadUserSubscriptionStatus {
        subscription_status: UserSubscriptionStatusDTO,
    },
    /// The user station has too many labels.
    #[error(r#"The user station has too many labels, must be at most {max_labels}."#)]
    StationHasTooManyLabels {
        /// The maximum number of labels allowed.
        max_labels: usize,
    },
    /// Concurrent station canister deployment.
    #[error(r#"Concurrent station canister deployment is not allowed."#)]
    ConcurrentStationDeployment,
}

impl DetailableError for UserError {
    fn details(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut details = std::collections::HashMap::new();
        match self {
            UserError::IdentityAlreadyHasUser { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::NotFound { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::Forbidden { user } => {
                details.insert("user".to_string(), user.to_string());
                Some(details)
            }
            UserError::ValidationError { info } => {
                details.insert("info".to_string(), info.to_string());
                Some(details)
            }
            UserError::AssociatedUserIdentityNotFound { identity } => {
                details.insert("identity".to_string(), identity.to_string());
                Some(details)
            }
            UserError::BadUserSubscriptionStatus {
                subscription_status,
            } => {
                details.insert(
                    "subscription_status".to_string(),
                    subscription_status.to_string(),
                );
                Some(details)
            }
            UserError::StationHasTooManyLabels { max_labels } => {
                details.insert("max_labels".to_string(), max_labels.to_string());
                Some(details)
            }
            _ => None,
        }
    }
}
