import { describe, expect, it } from 'vitest';
import { statusReasonToI18nKey, statusReasonsToTextSummary } from './evaluation.utils';
import { i18n } from '~/plugins/i18n.plugin';

describe('Evaluation utilities', () => {
  it('should contain the right terms', () => {
    const reasons = [
      { ApprovalThreshold: null },
      { AddressBook: null },
      { AddressBookMetadata: null },
      { AutoApproved: null },
    ];

    const status = { Approved: null };

    const summary = statusReasonsToTextSummary(status, reasons);

    for (const reason of reasons) {
      expect(summary).toContain(i18n.global.t(statusReasonToI18nKey(reason, status)));
    }
  });
});
