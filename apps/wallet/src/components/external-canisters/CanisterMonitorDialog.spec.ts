import { Principal } from '@dfinity/principal';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import { flushPromises } from '@vue/test-utils';
import { services } from '~/plugins/services.plugin';
import CanisterMonitorDialog from '~/components/external-canisters/CanisterMonitorDialog.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    monitorExternalCanister: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterMonitorDialog', () => {
  it('renders default card open is true', () => {
    const wrapper = mount(CanisterMonitorDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    const container = dialog.find('[data-test-id="canister-monitor-card"]');

    expect(container).not.toBeNull();

    wrapper.unmount();
  });

  it('triggers submit when on click of save button', async () => {
    const monitorMethod = vi
      .spyOn(services().station, 'monitorExternalCanister')
      .mockImplementation(() => Promise.reject());

    const wrapper = mount(CanisterMonitorDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const strategyInput = wrapper.findComponent({ name: 'VSelect' });
    await strategyInput.setValue('Always');

    await flushPromises();

    const nextBtn = wrapper.find('[data-test-id="monitor-dialog-stepper-next"]');
    await nextBtn.trigger('click');

    const obtainInput = wrapper.findComponent({ name: 'VSelect' });
    await obtainInput.setValue('StationDefault');

    const saveBtn = wrapper.find('[data-test-id="monitor-dialog-submit"]');
    await saveBtn.trigger('click');

    await flushPromises();

    expect(monitorMethod).toHaveBeenCalledOnce();

    wrapper.unmount();
  });
});
