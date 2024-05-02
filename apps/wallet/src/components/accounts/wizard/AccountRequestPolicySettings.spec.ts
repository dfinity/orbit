import { describe } from 'node:test';
import { expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import AccountRequestPolicySettings from './AccountRequestPolicySettings.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('AccountRequestPolicySettings', () => {
  it('mounts with default values', () => {
    const wrapper = mount(AccountRequestPolicySettings, {
      props: {
        modelValue: {
          configurationRule: undefined,
          transferRule: undefined,
        },
      },
    });

    expect(wrapper.find('[data-test-id="update-approval-policy"]').exists()).toBeTruthy();
    expect(wrapper.find('[data-test-id="transfer-approval-policy"]').exists()).toBeTruthy();
  });
});
