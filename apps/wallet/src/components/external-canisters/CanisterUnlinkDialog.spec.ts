import { Principal } from '@icp-sdk/core/principal';
import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import CanisterUnlinkDialog from './CanisterUnlinkDialog.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    unlinkExternalCanister: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterUnlinkDialog', () => {
  it('renders default card open is true', () => {
    const wrapper = mount(CanisterUnlinkDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    const container = dialog.find('[data-test-id="canister-unlink-card"]');

    expect(container).not.toBeNull();

    wrapper.unmount();
  });

  it('triggers submit when on click of save button', async () => {
    const fundMethod = vi
      .spyOn(services().station, 'unlinkExternalCanister')
      .mockImplementation(() => Promise.reject());

    const wrapper = mount(CanisterUnlinkDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const saveBtn = wrapper.find('[data-test-id="canister-unlink-save-button"]');
    await saveBtn.trigger('click');

    await flushPromises();

    expect(fundMethod).toHaveBeenCalledOnce();
    expect(fundMethod).toHaveBeenCalledWith({
      canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
      softDelete: true,
    });

    wrapper.unmount();
  });

  it('triggers submit when on click of save button with soft delete false', async () => {
    const fundMethod = vi
      .spyOn(services().station, 'unlinkExternalCanister')
      .mockImplementation(() => Promise.reject());

    const wrapper = mount(CanisterUnlinkDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const softDeleteCheckbox = wrapper.find('[name="soft_delete"]');
    await softDeleteCheckbox.setValue(false);

    const saveBtn = wrapper.find('[data-test-id="canister-unlink-save-button"]');
    await saveBtn.trigger('click');

    await flushPromises();

    expect(fundMethod).toHaveBeenCalledOnce();
    expect(fundMethod).toHaveBeenCalledWith({
      canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
      softDelete: false,
    });

    wrapper.unmount();
  });
});
