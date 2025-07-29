import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import CanisterIcSettingsDialog from './CanisterIcSettingsDialog.vue';
import { flushPromises } from '@vue/test-utils';
import { services } from '~/plugins/services.plugin';
import CanisterIcSettingsForm from '~/components/external-canisters/CanisterIcSettingsForm.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    editCanisterIcSettings: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterIcSettingsDialog', () => {
  it('renders default card open is true', () => {
    const wrapper = mount(CanisterIcSettingsDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    const container = dialog.find('[data-test-id="canister-ic-settings-card"]');

    expect(container).not.toBeNull();

    wrapper.unmount();
  });

  it('triggers submit when on click of save button', async () => {
    const editCanisterIcSettingsMethod = vi
      .spyOn(services().station, 'editCanisterIcSettings')
      .mockImplementation(() => Promise.reject());

    const wrapper = mount(CanisterIcSettingsDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        canisterSettings: {
          controllers: [],
          compute_allocation: BigInt(0),
          freezing_threshold: BigInt(0),
          memory_allocation: BigInt(0),
          reserved_cycles_limit: BigInt(0),
          log_visibility: { controllers: null },
          wasm_memory_limit: BigInt(0),
        },
        attach: true, // disables teleport in VDialog
      },
    });

    const form = wrapper.findComponent(CanisterIcSettingsForm);

    const input = form.find('[name="reserved_cycles_limit"]');
    await input.setValue('10000');

    await flushPromises();

    const saveBtn = wrapper.find('[data-test-id="canister-ic-settings-save-button"]');
    await saveBtn.trigger('click');

    await flushPromises();

    expect(editCanisterIcSettingsMethod).toHaveBeenCalledOnce();

    wrapper.unmount();
  });
});
