import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import BtnCanisterSetup from './BtnCanisterSetup.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    getExternalCanisterByCanisterId: vi
      .fn()
      .mockImplementation(() => Promise.reject(new Error('Failed to load canister'))),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('BtnCanisterSetup', () => {
  it('dialog is closed by default', async () => {
    const wrapper = mount(BtnCanisterSetup);
    expect(wrapper.exists()).toBe(true);
    expect(document.querySelector('[data-test-id="canister-setup-dialog"]')).toBeNull();

    wrapper.unmount();
  });

  it('opens the dialog onclick', async () => {
    const wrapper = mount(BtnCanisterSetup);

    expect(wrapper.exists()).toBe(true);
    expect(document.querySelector('[data-test-id="canister-setup-dialog"]')).toBeNull();

    await wrapper.find('[data-test-id="btn-canister-setup"]').trigger('click');

    expect(document.querySelector('[data-test-id="canister-setup-dialog"]')).not.toBeNull();

    wrapper.unmount();
  });
});
