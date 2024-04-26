import { describe } from 'node:test';
import { expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import AccountApprovalPolicySettings from './AccountApprovalPolicySettings.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('AccountApprovalPolicySettings', () => {
  it('mounts with default values', () => {
    const wrapper = mount(AccountApprovalPolicySettings, {
      props: {
        modelValue: {
          configurationCriteria: undefined,
          transferCriteria: undefined,
        },
      },
    });

    expect(wrapper.find('[data-test-id="update-approval-policy"]').exists()).toBeTruthy();
    expect(wrapper.find('[data-test-id="transfer-approval-policy"]').exists()).toBeTruthy();
  });
});
