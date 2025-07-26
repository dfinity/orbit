import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import CanisterTopUpDialog from './CanisterTopUpDialog.vue';
import { flushPromises } from '@vue/test-utils';
import { services } from '~/plugins/services.plugin';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    fundExternalCanister: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterSetupDialog', () => {
  it('renders default card open is true', () => {
    const wrapper = mount(CanisterTopUpDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    const container = dialog.find('[data-test-id="canister-top-up-card"]');

    expect(container).not.toBeNull();

    wrapper.unmount();
  });

  it('triggers submit when on click of save button', async () => {
    const fundMethod = vi
      .spyOn(services().station, 'fundExternalCanister')
      .mockImplementation(() => Promise.reject());

    const wrapper = mount(CanisterTopUpDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const cyclesInput = wrapper.find('[name="cycles"]');
    await cyclesInput.setValue('10');

    await flushPromises();

    const saveBtn = wrapper.find('[data-test-id="canister-top-up-save-button"]');
    await saveBtn.trigger('click');

    await flushPromises();

    expect(fundMethod).toHaveBeenCalledOnce();

    wrapper.unmount();
  });
});
