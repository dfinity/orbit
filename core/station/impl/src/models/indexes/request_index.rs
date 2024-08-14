use crate::models::{
    Request, RequestApprovalStatus, RequestId, RequestOperationFilterType, RequestStatus,
    RequestStatusCode, UserId,
};
use orbit_essentials::{storable, types::Timestamp};
use std::collections::BTreeSet;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequestIndexFields {
    pub requested_by: UserId,
    pub status: RequestStatusCode,
    pub operation_type: RequestOperationFilterType,
    pub created_at: Timestamp,
    pub expiration_dt: Timestamp,
    pub scheduled_at: Option<Timestamp>,
    pub last_modified_at: Timestamp,
    pub approved_by: BTreeSet<UserId>,
    pub rejected_by: BTreeSet<UserId>,
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
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RequestIndexKey {
    pub kind: RequestIndexKeyKind,
    pub request_id: RequestId,
}

impl Request {
    /// Converts the request to the corresponding index fields.
    fn index_fields(&self) -> RequestIndexFields {
        RequestIndexFields {
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

    /// Converts the request to a list of indexes.
    pub fn to_indexes(&self) -> Vec<(RequestIndexKey, RequestIndexFields)> {
        let mut indexes = vec![self.to_index_by_status(), self.to_index_by_created_at()];

        if let Some(index) = self.to_index_by_scheduled_at() {
            indexes.push(index);
        }

        indexes
    }
}
