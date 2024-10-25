use crate::DfxOrbit;
use station_api::{
    EvaluatedRequestPolicyRuleDTO, EvaluationStatusDTO, GetRequestResponse,
    RequestAdditionalInfoDTO, RequestApprovalDTO, RequestApprovalStatusDTO, RequestDTO,
    RequestOperationDTO, RequestStatusDTO,
};
use std::{collections::BTreeMap, fmt::Write};

impl DfxOrbit {
    pub(crate) fn display_get_request_response(
        &self,
        request: GetRequestResponse,
    ) -> anyhow::Result<String> {
        let base_info = request.request;
        let add_info = request.additional_info;

        let mut output = String::new();

        // General request information
        writeln!(output, "=== REQUEST ===")?;
        writeln!(output, "ID: {}", base_info.id)?;
        writeln!(
            output,
            "Operation: {}",
            display_request_operation(&base_info.operation)
        )?;
        writeln!(
            output,
            "Request URL: {}",
            self.station.request_url(&base_info.id)
        )?;
        writeln!(output, "Title: {}", base_info.title)?;
        if let Some(ref summary) = base_info.summary {
            writeln!(output, "Summary: {}", summary)?
        }
        writeln!(output, "Requested by: {}", add_info.requester_name)?;

        display_poll_state_overiew(&mut output, &base_info, &add_info)?;
        display_approvers_and_rejectors(&mut output, &base_info, &add_info)?;

        writeln!(
            output,
            "Execution Status: {}",
            display_request_status(&base_info.status)
        )?;
        if let Some(additional_status) = display_additional_stats_info(&base_info.status) {
            writeln!(output, "{}", additional_status)?;
        }

        match base_info.operation {
            RequestOperationDTO::ChangeExternalCanister(op) => {
                self.display_change_canister_operation(&mut output, op.as_ref())?;
            }
            RequestOperationDTO::CallExternalCanister(op) => {
                self.display_call_canister_operation(&mut output, op.as_ref())?;
            }
            // TODO: CreateCanister Additional information
            // TODO: ConfigureCanister Additional information
            _ => (),
        };

        Ok(output)
    }
}

fn display_approvers_and_rejectors<W: Write>(
    writer: &mut W,
    base_info: &RequestDTO,
    add_info: &RequestAdditionalInfoDTO,
) -> anyhow::Result<()> {
    let usernames: BTreeMap<String, String> = add_info
        .approvers
        .iter()
        .map(|user| (user.id.clone(), user.name.clone()))
        .collect();

    let (approvers, rejectors): (Vec<_>, Vec<_>) = base_info
        .approvals
        .iter()
        .partition(|approval| approval.status == RequestApprovalStatusDTO::Approved);

    if !approvers.is_empty() {
        write!(writer, "Approved by: ")?;
        display_request_approvals(writer, approvers, &usernames)?;
    }
    if !rejectors.is_empty() {
        write!(writer, "Rejected by: ")?;
        display_request_approvals(writer, rejectors, &usernames)?;
    }

    Ok(())
}

fn display_poll_state_overiew<W: Write>(
    writer: &mut W,
    base_info: &RequestDTO,
    add_info: &RequestAdditionalInfoDTO,
) -> anyhow::Result<()> {
    let Some(evaluation_result) = &add_info.evaluation_result else {
        return Ok(());
    };

    let approval_status: BTreeMap<String, RequestApprovalStatusDTO> = base_info
        .approvals
        .iter()
        .map(|approval| (approval.approver_id.clone(), approval.status.clone()))
        .collect();

    for result in &evaluation_result.policy_results {
        let status = match result.status {
            EvaluationStatusDTO::Approved => "Approved",
            EvaluationStatusDTO::Rejected => "Rejected",
            EvaluationStatusDTO::Pending => "Pending",
        };
        writeln!(writer, "Poll State: {status}")?;

        display_evaluated_rule(writer, &result.evaluated_rule, &approval_status)?;
    }

    Ok(())
}

fn display_evaluated_rule<W: Write>(
    writer: &mut W,
    rule: &EvaluatedRequestPolicyRuleDTO,
    status: &BTreeMap<String, RequestApprovalStatusDTO>,
) -> anyhow::Result<()> {
    match rule {
        EvaluatedRequestPolicyRuleDTO::AutoApproved => {
            writeln!(writer, "The request will be auto-approved")?
        }
        EvaluatedRequestPolicyRuleDTO::QuorumPercentage {
            total_possible_approvers,
            min_approved,
            approvers,
        } => display_quorum_state(
            writer,
            *total_possible_approvers,
            *min_approved,
            approvers,
            status,
        )?,
        EvaluatedRequestPolicyRuleDTO::Quorum {
            total_possible_approvers,
            min_approved,
            approvers,
        } => display_quorum_state(
            writer,
            *total_possible_approvers,
            *min_approved,
            approvers,
            status,
        )?,
        EvaluatedRequestPolicyRuleDTO::AllowListedByMetadata { metadata } => writeln!(
            writer,
            "By evaluating metadata: {}: {}",
            metadata.key, metadata.value
        )?,
        EvaluatedRequestPolicyRuleDTO::AllowListed => {
            writeln!(writer, "The request is allow-listed")?
        }
        // TODO: Implement nested rules (requires some refactoring in this file)
        EvaluatedRequestPolicyRuleDTO::AnyOf(_)
        | EvaluatedRequestPolicyRuleDTO::AllOf(_)
        | EvaluatedRequestPolicyRuleDTO::Not(_) => {
            writeln!(writer, "Displaying nested rules is currently unsupported")?
        }
    }

    Ok(())
}

fn display_quorum_state<W: Write>(
    writer: &mut W,
    eligible: usize,
    required: usize,
    approvers: &[String],
    status: &BTreeMap<String, RequestApprovalStatusDTO>,
) -> anyhow::Result<()> {
    write!(writer, "Number of eligible voters: {eligible},")?;
    write!(writer, " necessary quorum: {required},")?;
    write!(writer, " voted: {},", approvers.len())?;

    let approved = approvers
        .iter()
        .filter_map(|voter| status.get(voter))
        .filter(|&status| status == &RequestApprovalStatusDTO::Approved)
        .count();
    write!(writer, " approved: {approved},")?;

    let rejected = approvers
        .iter()
        .filter_map(|voter| status.get(voter))
        .filter(|&status| status == &RequestApprovalStatusDTO::Rejected)
        .count();
    writeln!(writer, " rejected: {rejected}")?;

    Ok(())
}

fn display_request_approvals<W: Write>(
    writer: &mut W,
    list: Vec<&RequestApprovalDTO>,
    usernames: &BTreeMap<String, String>,
) -> anyhow::Result<()> {
    for user in list {
        let name = usernames
            .get(&user.approver_id)
            .unwrap_or(&user.approver_id);
        write!(writer, "\n\t{}", name)?;
        if let Some(reason) = &user.status_reason {
            write!(writer, " (Reason: \"{}\")", reason)?;
        }
    }
    writeln!(writer)?;
    Ok(())
}

pub(super) fn display_request_operation(op: &RequestOperationDTO) -> &'static str {
    match op {
        RequestOperationDTO::Transfer(_) => "Transfer",
        RequestOperationDTO::AddAccount(_) => "AddAccount",
        RequestOperationDTO::EditAccount(_) => "EditAccount",
        RequestOperationDTO::AddAddressBookEntry(_) => "AddAddressBookEntry",
        RequestOperationDTO::EditAddressBookEntry(_) => "EditAddressBookEntry",
        RequestOperationDTO::RemoveAddressBookEntry(_) => "RemoveAddressBookEntry",
        RequestOperationDTO::AddUser(_) => "AddUser",
        RequestOperationDTO::EditUser(_) => "EditUser",
        RequestOperationDTO::AddUserGroup(_) => "AddUserGroup",
        RequestOperationDTO::EditUserGroup(_) => "EditUserGroup",
        RequestOperationDTO::RemoveUserGroup(_) => "RemoveUserGroup",
        RequestOperationDTO::SystemUpgrade(_) => "SystemUpgrade",
        RequestOperationDTO::SetDisasterRecovery(_) => "SetDisasterRecovery",
        RequestOperationDTO::ChangeExternalCanister(_) => "ChangeExternalCanister",
        RequestOperationDTO::CreateExternalCanister(_) => "CreateExternalCanister",
        RequestOperationDTO::ConfigureExternalCanister(_) => "ConfigureExternalCanister",
        RequestOperationDTO::CallExternalCanister(_) => "CallExternalCanister",
        RequestOperationDTO::FundExternalCanister(_) => "FundExternalCanister",
        RequestOperationDTO::EditPermission(_) => "EditPermission",
        RequestOperationDTO::AddRequestPolicy(_) => "AddRequestPolicy",
        RequestOperationDTO::EditRequestPolicy(_) => "EditRequestPolicy",
        RequestOperationDTO::RemoveRequestPolicy(_) => "RemoveRequestPolicy",
        RequestOperationDTO::ManageSystemInfo(_) => "ManageSystemInfo",
    }
}

pub(super) fn display_request_status(status: &RequestStatusDTO) -> &'static str {
    match status {
        RequestStatusDTO::Created => "Created",
        RequestStatusDTO::Approved => "Approved",
        RequestStatusDTO::Rejected => "Rejected",
        RequestStatusDTO::Cancelled { .. } => "Cancelled",
        RequestStatusDTO::Scheduled { .. } => "Scheduled",
        RequestStatusDTO::Processing { .. } => "Processing",
        RequestStatusDTO::Completed { .. } => "Completed",
        RequestStatusDTO::Failed { .. } => "Failed",
    }
}

fn display_additional_stats_info(status: &RequestStatusDTO) -> Option<String> {
    match status {
        RequestStatusDTO::Cancelled { reason } => {
            reason.clone().map(|reason| format!("Reason: {}", reason))
        }
        RequestStatusDTO::Failed { reason } => {
            reason.clone().map(|reason| format!("Reason: {}", reason))
        }
        _ => None,
    }
}
