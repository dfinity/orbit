import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import ChangeCanisterConfirmationScreen from './ChangeCanisterConfirmationScreen.vue';

describe('ChangeCanisterConfirmationScreen', () => {
  it('confirmation screen has checksum and comment fields', () => {
    const wrapper = mount(ChangeCanisterConfirmationScreen, {
      props: {
        wasmModuleChecksum: 'checksum',
        comment: 'My test comment',
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[name="wasm_checksum"]').exists()).toBe(true);
    expect(wrapper.find('[name="comment"]').exists()).toBe(true);
  });

  it('confirmation screen shows the checksum in the field in readonly mode', () => {
    const wrapper = mount(ChangeCanisterConfirmationScreen, {
      props: {
        wasmModuleChecksum: 'checksum',
      },
    });

    expect(wrapper.exists()).toBe(true);

    const field = wrapper.find('[name="wasm_checksum"]').element as HTMLInputElement;
    expect(field.value).toEqual('checksum');
    expect(field.readOnly).toBe(true);
  });

  it('confirmation screen shows the comment in the field in edit mode', () => {
    const wrapper = mount(ChangeCanisterConfirmationScreen, {
      props: {
        wasmModuleChecksum: 'checksum',
        comment: 'My test comment',
      },
    });

    expect(wrapper.exists()).toBe(true);

    const field = wrapper.find('[name="comment"]').element as HTMLInputElement;
    expect(field.value).toEqual('My test comment');
    expect(field.readOnly).toBe(false);
  });
});
