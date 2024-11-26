import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import InternetComputerNativeStandardForm from './InternetComputerNativeStandardForm.vue';

describe('InternetComputerNativeStandardForm', () => {
  it('renders correctly for creation', () => {
    const wrapper = mount(InternetComputerNativeStandardForm, {
      props: {
        modelValue: [],
        readonly: false,
      },
    });
    expect(wrapper.exists()).toBe(true);
  });

  it('initializes custom form from metadata', async () => {
    const wrapper = mount(InternetComputerNativeStandardForm, {
      props: {
        modelValue: [
          {
            key: 'ledger_canister_id',
            value: 'ryjl3-tyaaa-aaaaa-aaaba-cai',
          },
          {
            key: 'index_canister_id',
            value: 'qhbym-qaaaa-aaaaa-aaafq-cai',
          },
        ],
        readonly: false,
      },
    });

    await wrapper.vm.$nextTick();

    const ledgerInput = wrapper.find('input[name="metadata_ledger_canister_id"]')
      .element as HTMLInputElement;
    const indexInput = wrapper.find('input[name="metadata_index_canister_id"]')
      .element as HTMLInputElement;

    expect(ledgerInput.value).toBe('ryjl3-tyaaa-aaaaa-aaaba-cai');
    expect(indexInput.value).toBe('qhbym-qaaaa-aaaaa-aaafq-cai');
  });
});
