import { Principal } from '@icp-sdk/core/principal';
import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { GetExternalCanisterResult } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import { ExtractOk } from '~/types/helper.types';
import CanisterSetupDialog from './CanisterSetupDialog.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    getExternalCanisterByCanisterId: vi
      .fn()
      .mockImplementation(() => Promise.reject(new Error('Failed to load canister'))),
    fetchExternalCanisterFilters: vi.fn().mockImplementation(() =>
      Promise.resolve({
        labels: [['production']],
        names: [],
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterSetupDialog', () => {
  it('renders the error card if load fails', async () => {
    const wrapper = mount(CanisterSetupDialog, {
      props: {
        modelValue: true,
        canisterId: Principal.anonymous(),
      },
    });
    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    // this is needed to wait for the load call to finish
    await flushPromises();

    const errorCard = document.querySelector('[data-test-id="canister-setup-error-card"]');

    expect(errorCard).not.toBeNull();

    wrapper.unmount();
  });

  it('renders default card if load is ok', async () => {
    vi.spyOn(services().station, 'getExternalCanisterByCanisterId').mockResolvedValueOnce(
      {} as ExtractOk<GetExternalCanisterResult>,
    );

    const wrapper = mount(CanisterSetupDialog, {
      props: {
        modelValue: true,
        canisterId: undefined,
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    // this is needed to wait for the load call to finish
    await flushPromises();

    const errorCard = document.querySelector('[data-test-id="canister-setup-error-card"]');
    const okCard = document.querySelector('[data-test-id="canister-setup-ok-card"]');

    expect(errorCard).toBeNull();
    expect(okCard).not.toBeNull();

    wrapper.unmount();
  });
});
