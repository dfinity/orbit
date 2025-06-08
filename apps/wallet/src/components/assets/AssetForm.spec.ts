import { describe, expect, it } from 'vitest';
import AssetForm from './AssetForm.vue';
import { mount } from '~/test.utils';
import { BlockchainStandard } from '~/types/chain.types';

describe('AssetForm', () => {
  it('renders correctly for creation', () => {
    const wrapper = mount(AssetForm, {
      props: {
        modelValue: {},
      },
    });
    expect(wrapper.exists()).toBe(true);
  });

  it('shows custom metadata form for ICP', async () => {
    const wrapper = mount(AssetForm, {
      props: {
        modelValue: {
          blockchain: 'icp',
          standards: [BlockchainStandard.Native],
          metadata: [],
        },
      },
    });

    expect(wrapper.find('input[name="metadata_ledger_canister_id"]').exists()).toBe(true);
    expect(wrapper.find('input[name="metadata_index_canister_id"]').exists()).toBe(true);
  });

  it('emits valid form', async () => {
    const wrapper = mount(AssetForm, {
      props: {
        modelValue: {
          blockchain: 'icp',
          standards: [BlockchainStandard.Native],
          metadata: [],
          decimals: 8,
          name: 'Test',
          symbol: 'TEST',
        },
      },
    });

    await wrapper.vm.$nextTick();
    expect(wrapper.emitted('valid')).toEqual([[false]]);

    // fill out metadata for ICP
    await wrapper
      .find('input[name="metadata_ledger_canister_id"]')
      .setValue('ryjl3-tyaaa-aaaaa-aaaba-cai');

    await wrapper
      .find('input[name="metadata_index_canister_id"]')
      .setValue('qhbym-qaaaa-aaaaa-aaafq-cai');

    await wrapper.vm.$nextTick();
    expect(wrapper.emitted('valid')).toEqual([[false], [true]]);
  });
});
