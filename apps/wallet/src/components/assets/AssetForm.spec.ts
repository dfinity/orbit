import { describe, expect, it } from 'vitest';
import AssetForm from './AssetForm.vue';
import { mount } from '~/test.utils';

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
          standards: ['native'],
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
          standards: ['native'],
          metadata: [],
          decimals: 8,
          name: 'Test',
          symbol: 'TEST',
        },
      },
    });

    // has not emitted "valid" event
    expect(wrapper.emitted('valid')).toBeUndefined();

    // fill out metadata for ICP
    await wrapper
      .find('input[name="metadata_ledger_canister_id"]')
      .setValue('ryjl3-tyaaa-aaaaa-aaaba-cai');

    expect(wrapper.emitted('valid')).toEqual([[false]]);

    await wrapper
      .find('input[name="metadata_index_canister_id"]')
      .setValue('qhbym-qaaaa-aaaaa-aaafq-cai');

    await wrapper.vm.$nextTick();

    // has emitted "valid" event with value true
    expect(wrapper.emitted('valid')).toEqual([[false], [true]]);
  });
});
