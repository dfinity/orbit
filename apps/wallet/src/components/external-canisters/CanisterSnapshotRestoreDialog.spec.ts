import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it, vi } from 'vitest';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import CanisterSnapshotRestoreDialog from './CanisterSnapshotRestoreDialog.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    restoreExternalCanisterSnapshot: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterSnapshotRestoreDialog', () => {
  it('renders default card open is true', () => {
    const wrapper = mount(CanisterSnapshotRestoreDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        snapshot: { snapshotId: '1', takenAtTimestamp: '2024-12-01T10:00:00Z', totalSize: 100 },
        attach: true, // disables teleport in VDialog
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    const container = dialog.find('[data-test-id="canister-snapshot-delete-card"]');

    expect(container).not.toBeNull();

    wrapper.unmount();
  });

  it('triggers submit when on click of save button', async () => {
    const restoreSnapshotMethod = vi
      .spyOn(services().station, 'restoreExternalCanisterSnapshot')
      .mockImplementation(() => Promise.reject());

    const wrapper = mount(CanisterSnapshotRestoreDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        snapshot: { snapshotId: '1', takenAtTimestamp: '2024-12-01T10:00:00Z', totalSize: 100 },
        attach: true, // disables teleport in VDialog
      },
    });

    const saveBtn = wrapper.find('[data-test-id="submit-btn"]');
    await saveBtn.trigger('click');

    expect(restoreSnapshotMethod).toHaveBeenCalled();
  });
});
