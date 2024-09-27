import { flushPromises } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import { VTextarea } from 'vuetify/components';
import { mount } from '~/test.utils';
import { hexStringToArrayBuffer } from '~/utils/crypto.utils';
import CanisterArgumentField from './CanisterArgumentField.vue';

describe('CanisterArgumentField', () => {
  it('renders with empty input', () => {
    const input = mount(CanisterArgumentField, {
      props: { modelValue: undefined },
    });

    expect(input.exists()).toBe(true);
    expect(input.findComponent(VTextarea).exists()).toBe(true);
  });

  it('emits an update as Uint8Array when text is added', async () => {
    const wrapper = mount(CanisterArgumentField);
    const content = '1234';
    const hexString = new Uint8Array(hexStringToArrayBuffer(content));

    const wrapperVm = wrapper.vm as typeof wrapper.vm & { argument?: string };
    wrapperVm.argument = '1234';

    await flushPromises();

    expect(wrapper.emitted('update:modelValue')?.[1]).toEqual([hexString]);
  });
});
