use orbit_essentials::api::ApiError;

pub enum UpgraderApiError {
    NotController,
    Unauthorized,
    DisasterRecoveryInProgress,
    EmptyCommittee,
    Unexpected(String),
}

impl From<UpgraderApiError> for ApiError {
    fn from(err: UpgraderApiError) -> Self {
        match err {
            UpgraderApiError::NotController => ApiError {
                code: "NOT_CONTROLLER".to_owned(),
                message: Some("Caller is not the controller.".to_owned()),
                details: None,
            },
            UpgraderApiError::Unauthorized => ApiError {
                code: "UNAUTHORIZED".to_owned(),
                message: Some("Caller is not authorized.".to_owned()),
                details: None,
            },
            UpgraderApiError::DisasterRecoveryInProgress => ApiError {
                code: "DISASTER_RECOVERY_IN_PROGRESS".to_owned(),
                message: Some("Disaster recovery is in progress.".to_owned()),
                details: None,
            },
            UpgraderApiError::EmptyCommittee => ApiError {
                code: "EMPTY_COMMITTEE".to_owned(),
                message: Some("Committee cannot be empty.".to_owned()),
                details: None,
            },
            UpgraderApiError::Unexpected(err) => ApiError {
                code: "UNEXPECTED_ERROR".to_owned(),
                message: Some(err),
                details: None,
            },
        }
    }
}
