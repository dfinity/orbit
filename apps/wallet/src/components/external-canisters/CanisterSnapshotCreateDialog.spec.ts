import { Principal } from '@dfinity/principal';
import { describe, expect, it, vi } from 'vitest';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import CanisterSnapshotCreateDialog from './CanisterSnapshotCreateDialog.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    createExternalCanisterSnapshot: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterSnapshotCreateDialog', () => {
  it('renders default card open is true', () => {
    const wrapper = mount(CanisterSnapshotCreateDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    const container = dialog.find('[data-test-id="canister-snapshot-create-card"]');

    expect(container).not.toBeNull();

    wrapper.unmount();
  });

  it('triggers submit when on click of save button', async () => {
    const createSnapshotMethod = vi
      .spyOn(services().station, 'createExternalCanisterSnapshot')
      .mockImplementation(() => Promise.reject());

    const wrapper = mount(CanisterSnapshotCreateDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const saveBtn = wrapper.find('[data-test-id="submit-btn"]');
    await saveBtn.trigger('click');

    expect(createSnapshotMethod).toHaveBeenCalled();
  });
});
