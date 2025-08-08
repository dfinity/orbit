import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import CanisterInstallDialog from './CanisterInstallDialog.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    changeExternalCanister: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterInstallDialog', () => {
  it('renders default card open is true', () => {
    const wrapper = mount(CanisterInstallDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    const container = dialog.find('[data-test-id="canister-install-card"]');

    expect(container).not.toBeNull();

    wrapper.unmount();
  });

  it('triggers submit when on click of save button', async () => {
    const wrapper = mount(CanisterInstallDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const form = wrapper.findComponent({ name: 'CanisterInstallForm' });
    expect(form.exists()).toBe(true);

    const wrapperVm = wrapper.vm as typeof wrapper.vm & { triggerFormSubmit: boolean };

    expect(wrapperVm.triggerFormSubmit).toBe(false);

    const saveBtn = wrapper.find('[data-test-id="canister-install-save-button"]');
    await saveBtn.trigger('click');

    // triggerSubmit is only set to false if the parent component first sets it to true first,
    // instead of checking on the parent data directly, we check on the emitted event
    // to avoid race conditions
    expect(form.emitted('update:triggerSubmit')?.[0]).toEqual([false]);

    wrapper.unmount();
  });
});
