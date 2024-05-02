import { EvaluationStatus, EvaluationSummaryReason } from '~/generated/station/station.did';
import { unreachable, variantIs } from './helper.utils';
import { i18n } from '~/plugins/i18n.plugin';

export function statusToI18nKeyPrefix(status: EvaluationStatus): string {
  if (variantIs(status, 'Approved')) {
    return 'approved';
  } else if (variantIs(status, 'Pending')) {
    return 'pending';
  } else if (variantIs(status, 'Rejected')) {
    return 'rejected';
  } else {
    return unreachable(status);
  }
}

export function summaryReasonToI18nKey(
  reason: EvaluationSummaryReason,
  status: EvaluationStatus,
): string {
  const prefix = statusToI18nKeyPrefix(status);

  if (variantIs(reason, 'ApprovalQuorum')) {
    return `requests.evaluation.${prefix}_reason_approval_quorum`;
  } else if (variantIs(reason, 'AllowList')) {
    return `requests.evaluation.${prefix}_reason_allowlist`;
  } else if (variantIs(reason, 'AllowListMetadata')) {
    return `requests.evaluation.${prefix}_reason_allowlist_metadata`;
  } else if (variantIs(reason, 'AutoApproved')) {
    return `requests.evaluation.reason_auto_approved`;
  } else {
    return unreachable(reason);
  }
}

export function statusToI18nKey(status: EvaluationStatus): string {
  if (variantIs(status, 'Approved')) {
    return 'requests.evaluation.summary_approved';
  } else if (variantIs(status, 'Pending')) {
    return 'requests.evaluation.summary_pending';
  } else if (variantIs(status, 'Rejected')) {
    return 'requests.evaluation.summary_rejected';
  } else {
    return unreachable(status);
  }
}

export function statusReasonsToTextSummary(
  finalStatus: EvaluationStatus,
  reasons: EvaluationSummaryReason[],
): string {
  const summaryKey = statusToI18nKey(finalStatus);
  const reasonList = reasons
    .map(reason => i18n.global.t(summaryReasonToI18nKey(reason, finalStatus)))
    .join(', ');

  return `${i18n.global.t(summaryKey, {
    count: reasons.length,
  })} ${reasonList}.`;
}
