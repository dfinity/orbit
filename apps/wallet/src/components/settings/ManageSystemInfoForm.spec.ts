import { flushPromises } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import ManageSystemInfoForm from './ManageSystemInfoForm.vue';

describe('ManageSystemInfoForm', () => {
  it('renders properly', () => {
    const wrapper = mount(ManageSystemInfoForm, {
      props: {
        modelValue: {
          name: ['Test'],
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('submits the form and emits it', async () => {
    const wrapper = mount(ManageSystemInfoForm, {
      props: {
        modelValue: {
          name: ['Test'],
        },
      },
    });

    const nameInput = wrapper.find('input[name="name"]');
    expect(nameInput.exists()).toBe(true);

    await nameInput.setValue('Personal');

    const form = wrapper.findComponent({ ref: 'form' });
    await form.trigger('submit');

    await flushPromises();

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')).toEqual([
      [
        {
          name: ['Personal'],
        },
      ],
    ]);
  });
});
