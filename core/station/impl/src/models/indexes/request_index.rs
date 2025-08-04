use crate::models::{
    resource::Resource, Request, RequestApprovalStatus, RequestId, RequestOperationFilterType,
    RequestStatus, RequestStatusCode, UserId,
};
use orbit_essentials::{storable, types::Timestamp};
use std::collections::BTreeSet;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestIndexFields {
    pub id: RequestId,
    pub requested_by: UserId,
    pub status: RequestStatusCode,
    pub operation_type: RequestOperationFilterType,
    pub created_at: Timestamp,
    pub expiration_dt: Timestamp,
    pub scheduled_at: Option<Timestamp>,
    pub last_modified_at: Timestamp,
    pub approved_by: BTreeSet<UserId>,
    pub rejected_by: BTreeSet<UserId>,
    pub resources: Vec<Resource>,
    pub deduplication_key: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum RequestIndexKeyKind {
    // Always created for each request with the creation timestamp
    CreatedAt(Timestamp),
    // Only created if the request is scheduled, with the scheduled timestamp
    ScheduledAt(Timestamp),
    // Always created for each request, with the status of the request
    Status(RequestStatusCode),
    // Only created if the request has a deduplication key, with the deduplication key
    DeduplicationKey(String),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RequestIndexKey {
    pub kind: RequestIndexKeyKind,
    pub request_id: RequestId,
}

impl Request {
    /// Converts the request to the corresponding index fields.
    pub fn index_fields(&self) -> RequestIndexFields {
        RequestIndexFields {
            id: self.id,
            requested_by: self.requested_by,
            status: self.status.clone().into(),
            operation_type: self.operation.clone().into(),
            created_at: self.created_timestamp,
            expiration_dt: self.expiration_dt,
            scheduled_at: match &self.status {
                RequestStatus::Scheduled { scheduled_at } => Some(*scheduled_at),
                _ => None,
            },
            last_modified_at: self.last_modification_timestamp,
            approved_by: self
                .approvals
                .iter()
                .filter_map(|approval| match approval.status {
                    RequestApprovalStatus::Approved => Some(approval.approver_id),
                    _ => None,
                })
                .collect(),
            rejected_by: self
                .approvals
                .iter()
                .filter_map(|approval| match approval.status {
                    RequestApprovalStatus::Rejected => Some(approval.approver_id),
                    _ => None,
                })
                .collect(),
            resources: self.operation.to_resources(),
            deduplication_key: self.deduplication_key.clone(),
            tags: self.tags.clone(),
        }
    }

    /// Converts the request to an index by its creation timestamp.
    fn to_index_by_created_at(&self) -> (RequestIndexKey, RequestIndexFields) {
        (
            RequestIndexKey {
                kind: RequestIndexKeyKind::CreatedAt(self.created_timestamp),
                request_id: self.id,
            },
            self.index_fields(),
        )
    }

    /// Converts the request to an index by its scheduled timestamp if it is scheduled.
    fn to_index_by_scheduled_at(&self) -> Option<(RequestIndexKey, RequestIndexFields)> {
        match &self.status {
            RequestStatus::Scheduled { scheduled_at } => Some((
                RequestIndexKey {
                    kind: RequestIndexKeyKind::ScheduledAt(*scheduled_at),
                    request_id: self.id,
                },
                self.index_fields(),
            )),
            _ => None,
        }
    }

    /// Converts the request to an index by its status.
    fn to_index_by_status(&self) -> (RequestIndexKey, RequestIndexFields) {
        (
            RequestIndexKey {
                kind: RequestIndexKeyKind::Status(self.status.clone().into()),
                request_id: self.id,
            },
            self.index_fields(),
        )
    }

    /// Converts the request to an index by its deduplication key if it has one.
    fn to_index_by_deduplication_key(&self) -> Option<(RequestIndexKey, RequestIndexFields)> {
        self.deduplication_key.as_ref().map(|deduplication_key| {
            (
                RequestIndexKey {
                    kind: RequestIndexKeyKind::DeduplicationKey(deduplication_key.clone()),
                    request_id: self.id,
                },
                self.index_fields(),
            )
        })
    }

    /// Converts the request to a list of indexes.
    pub fn to_indexes(&self) -> Vec<(RequestIndexKey, RequestIndexFields)> {
        let mut indexes = vec![self.to_index_by_status(), self.to_index_by_created_at()];

        if let Some(index) = self.to_index_by_scheduled_at() {
            indexes.push(index);
        }

        if let Some(index) = self.to_index_by_deduplication_key() {
            indexes.push(index);
        }

        indexes
    }
}
