use crate::{
    core::ic_cdk::api::time,
    models::{Proposal, ProposalExecutionPlan, ProposalOperation, ProposalStatus, UserId},
};
use ic_canister_core::{
    types::Timestamp,
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use uuid::Uuid;
use wallet_api::{ProposalDTO, ProposalExecutionScheduleDTO};

impl From<Proposal> for ProposalDTO {
    fn from(proposal: Proposal) -> ProposalDTO {
        ProposalDTO {
            id: Uuid::from_bytes(proposal.id).hyphenated().to_string(),
            proposed_by: Uuid::from_bytes(proposal.proposed_by)
                .hyphenated()
                .to_string(),
            status: proposal.status.into(),
            metadata: proposal.metadata,
            operation: proposal.operation.into(),
            title: proposal.title,
            summary: proposal.summary,
            expiration_dt: timestamp_to_rfc3339(&proposal.expiration_dt),
            execution_plan: proposal.execution_plan.into(),
            created_at: timestamp_to_rfc3339(&proposal.created_timestamp),
            votes: proposal
                .votes
                .iter()
                .map(|vote| vote.to_owned().into())
                .collect(),
        }
    }
}

impl Proposal {
    pub fn new(
        proposal_id: Uuid,
        proposer: UserId,
        expiration_dt: Timestamp,
        operation: ProposalOperation,
        execution_plan: ProposalExecutionPlan,
        title: String,
        summary: Option<String>,
    ) -> Proposal {
        Proposal {
            id: *proposal_id.as_bytes(),
            title,
            operation,
            summary,
            proposed_by: proposer,
            status: ProposalStatus::Created,
            expiration_dt,
            execution_plan,
            votes: vec![],
            metadata: vec![],
            created_timestamp: time(),
            last_modification_timestamp: time(),
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
