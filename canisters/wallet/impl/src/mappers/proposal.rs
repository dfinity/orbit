use crate::{
    core::ic_cdk::api::time,
    models::{
        Proposal, ProposalAdditionalInfo, ProposalCallerPrivileges, ProposalExecutionPlan,
        ProposalOperation, ProposalStatus, UserId,
    },
};
use ic_canister_core::{
    types::{Timestamp, UUID},
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use uuid::Uuid;
use wallet_api::{ProposalDTO, ProposalExecutionScheduleDTO};

impl Proposal {
    pub fn new(
        proposal_id: UUID,
        proposer: UserId,
        expiration_dt: Timestamp,
        operation: ProposalOperation,
        execution_plan: ProposalExecutionPlan,
        title: String,
        summary: Option<String>,
    ) -> Proposal {
        Proposal {
            id: proposal_id,
            title,
            operation,
            summary,
            proposed_by: proposer,
            status: ProposalStatus::Created,
            expiration_dt,
            execution_plan,
            votes: vec![],
            created_timestamp: time(),
            last_modification_timestamp: time(),
        }
    }

    pub fn to_dto(self) -> ProposalDTO {
        ProposalDTO {
            id: Uuid::from_bytes(self.id).hyphenated().to_string(),
            proposed_by: Uuid::from_bytes(self.proposed_by).hyphenated().to_string(),
            status: self.status.into(),
            operation: self.operation.into(),
            title: self.title,
            summary: self.summary,
            expiration_dt: timestamp_to_rfc3339(&self.expiration_dt),
            execution_plan: self.execution_plan.into(),
            created_at: timestamp_to_rfc3339(&self.created_timestamp),
            votes: self
                .votes
                .iter()
                .map(|vote| vote.to_owned().into())
                .collect(),
        }
    }
}

impl From<ProposalExecutionScheduleDTO> for ProposalExecutionPlan {
    fn from(dto: ProposalExecutionScheduleDTO) -> Self {
        match dto {
            ProposalExecutionScheduleDTO::Immediate => Self::Immediate,
            ProposalExecutionScheduleDTO::Scheduled { execution_time } => Self::Scheduled {
                execution_time: rfc3339_to_timestamp(&execution_time),
            },
        }
    }
}

impl From<ProposalExecutionPlan> for ProposalExecutionScheduleDTO {
    fn from(plan: ProposalExecutionPlan) -> Self {
        match plan {
            ProposalExecutionPlan::Immediate => Self::Immediate,
            ProposalExecutionPlan::Scheduled { execution_time } => Self::Scheduled {
                execution_time: timestamp_to_rfc3339(&execution_time),
            },
        }
    }
}

impl From<ProposalCallerPrivileges> for wallet_api::ProposalCallerPrivilegesDTO {
    fn from(privileges: ProposalCallerPrivileges) -> Self {
        Self {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_vote: privileges.can_vote,
        }
    }
}

impl From<ProposalAdditionalInfo> for wallet_api::ProposalAdditionalInfoDTO {
    fn from(info: ProposalAdditionalInfo) -> Self {
        Self {
            proposer: info.proposer.into(),
            voters: info.voters.into_iter().map(|voter| voter.into()).collect(),
        }
    }
}
