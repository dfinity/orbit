import { describe, expect, it } from 'vitest';
import { summaryReasonToI18nKey, statusReasonsToTextSummary } from './evaluation.utils';
import { i18n } from '~/plugins/i18n.plugin';
import { EvaluationSummaryReason } from '~/generated/station/station.did';

describe('Evaluation utilities', () => {
  it('should contain the right terms', () => {
    const reasons: EvaluationSummaryReason[] = [
      { ApprovalQuorum: null },
      { AllowList: null },
      { AllowListMetadata: null },
      { AutoApproved: null },
    ];

    const status = { Approved: null };

    const summary = statusReasonsToTextSummary(status, reasons);

    for (const reason of reasons) {
      expect(summary).toContain(i18n.global.t(summaryReasonToI18nKey(reason, status)));
    }
  });
});
