import { Principal } from '@dfinity/principal';
import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import CanisterSnapshotList from './CanisterSnapshotList.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    getExternalCanisterSnapshots: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterSnapshotList', () => {
  it('shows load error when loading fails', async () => {
    const wrapper = mount(CanisterSnapshotList, {
      props: {
        hasInstalledWasm: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
      },
    });

    await flushPromises();

    const container = wrapper.find('[data-test-id="load-error"]');
    const emptyContainer = wrapper.find('[data-test-id="empty-snapshots-list"]');
    expect(container.exists()).toBe(true);
    expect(emptyContainer.exists()).toBe(false);

    wrapper.unmount();
  });

  it('shows empty snapshot list when there is no snapshot', async () => {
    const fetchMethod = vi
      .spyOn(services().station, 'getExternalCanisterSnapshots')
      .mockImplementation(() => Promise.resolve([]));

    const wrapper = mount(CanisterSnapshotList, {
      props: {
        hasInstalledWasm: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
      },
    });

    await flushPromises();

    expect(fetchMethod).toHaveBeenCalled();

    const container = wrapper.find('[data-test-id="empty-snapshots-list"]');
    const errorContainer = wrapper.find('[data-test-id="load-error"]');

    expect(container.exists()).toBe(true);
    expect(errorContainer.exists()).toBe(false);

    wrapper.unmount();
  });

  it('shows snapshot list when there are snapshots', async () => {
    const fetchMethod = vi
      .spyOn(services().station, 'getExternalCanisterSnapshots')
      .mockImplementation(() =>
        Promise.resolve([
          {
            snapshot_id: '1',
            taken_at_timestamp: '2021-09-01T00:00:00Z',
            total_size: BigInt(100),
          },
        ]),
      );

    const wrapper = mount(CanisterSnapshotList, {
      props: {
        hasInstalledWasm: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
      },
    });

    await flushPromises();

    expect(fetchMethod).toHaveBeenCalled();

    const snapshotListContainer = wrapper.find('[data-test-id="snapshots-list"]');
    const emptyContainer = wrapper.find('[data-test-id="empty-snapshots-list"]');

    expect(snapshotListContainer.exists()).toBe(true);
    expect(emptyContainer.exists()).toBe(false);

    wrapper.unmount();
  });
});
