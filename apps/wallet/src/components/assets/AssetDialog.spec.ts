import { describe, expect, it, vi } from 'vitest';
import AssetDialog from './AssetDialog.vue';
import { mount } from '~/test.utils';
import { StationService } from '~/services/station.service';
import { Capabilities, GetAssetResult } from '~/generated/station/station.did';
import { ExtractOk } from '~/types/helper.types';
import { services } from '~/plugins/services.plugin';
import AssetForm from './AssetForm.vue';
import { flushPromises } from '@vue/test-utils';
import { VCard } from 'vuetify/components';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    capabilities: vi.fn().mockImplementation(() =>
      Promise.resolve({
        supported_blockchains: [
          {
            blockchain: 'icp',
            supported_standards: [{ standard: 'native' }],
          },
        ],
      } as Capabilities),
    ),
    addAsset: vi.fn().mockImplementation(() => Promise.resolve({} as Request)),
    getAsset: vi.fn().mockImplementation(() =>
      Promise.resolve({
        asset: {
          id: '1',
          blockchain: 'icp',
          decimals: 8,
          metadata: [
            {
              key: 'ledger_canister_id',
              value: 'ryjl3-tyaaa-aaaaa-aaaba-cai',
            },
            {
              key: 'index_canister_id',
              value: 'qhbym-qaaaa-aaaaa-aaafq-cai',
            },
          ],
          standards: ['native'],
          name: 'Test',
          symbol: 'TEST',
        },
        privileges: {},
      } as ExtractOk<GetAssetResult>),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('AssetDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(AssetDialog, {
      props: {
        open: true,
      },
    });
    expect(wrapper.exists()).toBe(true);
  });

  it('loads and displays existing asset', async () => {
    const wrapper = mount(AssetDialog, {
      props: {
        open: true,
        assetId: '1',
      },
    });

    await flushPromises();

    // expect getAsset to be called
    expect(services().station.getAsset).toHaveBeenCalled();

    const form = wrapper.findComponent(AssetForm);

    const name = form.find('input[name="name"]').element as HTMLInputElement;
    const symbol = form.find('input[name="symbol"]').element as HTMLInputElement;
    const decimals = form.find('input[name="decimals"]').element as HTMLInputElement;
    const ledger = form.find('input[name="metadata_ledger_canister_id"]')
      .element as HTMLInputElement;
    const index = form.find('input[name="metadata_index_canister_id"]').element as HTMLInputElement;

    expect(name.value).toBe('Test');
    expect(symbol.value).toBe('TEST');
    expect(decimals.value).toBe('8');
    expect(ledger.value).toBe('ryjl3-tyaaa-aaaaa-aaaba-cai');
    expect(index.value).toBe('qhbym-qaaaa-aaaaa-aaafq-cai');
  });

  it('creates new asset', async () => {
    const wrapper = mount(AssetDialog, {
      props: {
        open: true,
      },
    });

    await flushPromises();

    const dialogContents = wrapper.findComponent(VCard);

    const form = wrapper.findComponent(AssetForm);

    await form
      .findComponent({ name: 'BlockchainAutocomplete' })
      .vm.$emit('update:modelValue', 'icp');

    await form
      .findComponent({ name: 'StandardsAutocomplete' })
      .vm.$emit('update:modelValue', ['native']);

    // fill out form
    await form.find('input[name="name"]').setValue('Test');
    await form.find('input[name="symbol"]').setValue('TEST');
    await form.find('input[name="decimals"]').setValue('8');
    await form
      .find('input[name="metadata_ledger_canister_id"]')
      .setValue('ryjl3-tyaaa-aaaaa-aaaba-cai');
    await form
      .find('input[name="metadata_index_canister_id"]')
      .setValue('qhbym-qaaaa-aaaaa-aaafq-cai');

    await flushPromises();

    const saveButton = dialogContents.find('[data-test-id="save-asset"]');

    await saveButton.trigger('click');

    await flushPromises();

    expect(services().station.addAsset).toHaveBeenCalled();
  });
});
