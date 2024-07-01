use orbit_essentials::api::ApiError;

pub enum UpgraderApiError {
    NotController,
    Unauthorized,
    DisasterRecoveryInProgress,
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
        }
    }
}
