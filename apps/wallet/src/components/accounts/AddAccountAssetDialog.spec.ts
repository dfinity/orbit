import { describe, expect, it, vi } from 'vitest';
import { Account, Asset } from '~/generated/station/station.did';
import { mount } from '~/test.utils';
import AddAccountAssetDialog from './AddAccountAssetDialog.vue';
import { BlockchainStandard } from '~/types/chain.types';
import { useStationStore } from '~/stores/station.store';
import TokenAutocomplete from '../inputs/TokenAutocomplete.vue';
import { flushPromises } from '@vue/test-utils';
import { StationService } from '~/services/station.service';
import { services } from '~/plugins/services.plugin';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    editAccount: vi.fn().mockImplementation(() => Promise.resolve({} as Account)),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

const mockAssets: Asset[] = [
  {
    id: '1',
    blockchain: 'icp',
    decimals: 8,
    metadata: [],
    name: 'Test',
    symbol: 'TEST',
    standards: [BlockchainStandard.Native],
  },

  {
    id: '2',
    blockchain: 'icp',
    decimals: 8,
    metadata: [],
    name: 'Test2',
    symbol: 'TEST2',
    standards: [BlockchainStandard.ICRC1],
  },
];

const mockAccount: Account = {
  id: '1',
  assets: [
    {
      asset_id: mockAssets[0].id,
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

describe('AddAccountAssetDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(AddAccountAssetDialog, {
      props: {
        account: mockAccount,

        open: true,
        attach: true,
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('edits the account when submitted', async () => {
    const wrapper = mount(AddAccountAssetDialog, {
      props: {
        account: { ...mockAccount },
        open: true,
        attach: true,
      },
    });

    const station = useStationStore();
    station.configuration.details.supported_assets = mockAssets;

    const submitBtn = wrapper.find('button[data-test-id="add-asset-dialog-save-button"]');

    const tokenField = wrapper.findComponent(TokenAutocomplete);

    tokenField.vm.$emit('update:modelValue', [mockAssets[1].id]);

    await wrapper.vm.$nextTick();
    await flushPromises();

    await submitBtn.trigger('click');

    await wrapper.vm.$nextTick();
    await flushPromises();

    // check if editAccount was called with the correct asset
    expect(services().station.editAccount).toHaveBeenCalledWith(
      expect.objectContaining({
        change_assets: [
          {
            Change: {
              add_assets: [mockAssets[1].id],
              remove_assets: [],
            },
          },
        ],
      }),
    );

    vi.clearAllMocks();
  });
});
