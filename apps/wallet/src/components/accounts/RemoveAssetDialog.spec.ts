import { afterAll, describe, expect, it, vi } from 'vitest';
import { Account } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import RemoveAssetDialog from './RemoveAssetDialog.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    editAccount: vi.fn().mockImplementation(() => Promise.resolve({} as Account)),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

const mockAccount: Account = {
  id: '1',
  assets: [
    {
      asset_id: '1',
      balance: [],
    },
  ],
  addresses: [],
  configs_request_policy: [],
  metadata: [],
  last_modification_timestamp: '2021-09-01T00:00:00Z',
  name: 'Test',
  transfer_request_policy: [],
};

describe('RemoveAssetDialog', () => {
  afterAll(() => {});

  it('renders correctly', () => {
    const wrapper = mount(RemoveAssetDialog, {
      props: {
        account: mockAccount,
        asset: '1',

        open: true,
        attach: true,
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('removes the asset when confirmed', async () => {
    const wrapper = mount(RemoveAssetDialog, {
      props: {
        account: mockAccount,
        asset: '1',

        open: true,
        attach: true,
      },
    });

    const saveButton = wrapper.find('button[data-test-id="remove-asset-dialog-confirm-button');

    await saveButton.trigger('click');

    expect(services().station.editAccount).toHaveBeenCalledWith(
      expect.objectContaining({
        change_assets: [
          {
            Change: {
              add_assets: [],
              remove_assets: ['1'],
            },
          },
        ],
      }),
    );

    vi.clearAllMocks();
  });

  it('does not remove the asset when canceled', async () => {
    const wrapper = mount(RemoveAssetDialog, {
      props: {
        account: mockAccount,
        asset: '1',

        open: true,
        attach: true,
      },
    });

    const cancelButton = wrapper.find('button[data-test-id="remove-asset-dialog-cancel-button');

    await cancelButton.trigger('click');

    expect(services().station.editAccount).not.toHaveBeenCalled();
  });
});
