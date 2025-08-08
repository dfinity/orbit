import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import CanisterCallDialog from './CanisterCallDialog.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    changeExternalCanister: vi.fn().mockImplementation(() => Promise.reject()),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('CanisterCallDialog', () => {
  it('renders default card open is true', () => {
    const wrapper = mount(CanisterCallDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const dialog = wrapper.findComponent({ name: 'VDialog' });
    expect(dialog.exists()).toBe(true);

    const container = dialog.find('[data-test-id="canister-call-card"]');

    expect(container).not.toBeNull();

    wrapper.unmount();
  });

  it('renders with form and submit button', async () => {
    const wrapper = mount(CanisterCallDialog, {
      props: {
        open: true,
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        attach: true, // disables teleport in VDialog
      },
    });

    const form = wrapper.findComponent({ name: 'VForm' });
    expect(form.exists()).toBe(true);

    const submitBtn = wrapper.find('[data-test-id="submit-btn"]');
    expect(submitBtn.exists()).toBe(true);
  });
});
