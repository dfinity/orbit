import { flushPromises } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import { mount } from '~/ui/test.utils';
import UserGroupForm from './UserGroupForm.vue';

describe('AddPrincipalForm', () => {
  it('renders properly', () => {
    const wrapper = mount(UserGroupForm, {
      props: {
        modelValue: {},
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('changes the model on changes', async () => {
    const wrapper = mount(UserGroupForm, {
      props: {
        modelValue: {},
      },
    });

    expect(wrapper.exists()).toBe(true);

    const nameInput = wrapper.find('input');
    expect(nameInput.exists()).toBe(true);

    await nameInput.setValue('Test');

    expect(wrapper.emitted('update:modelValue')).toBeTruthy();
    expect(wrapper.emitted('update:modelValue')).toEqual([[{ name: 'Test' }]]);
  });

  it('submits the form and emits it', async () => {
    const wrapper = mount(UserGroupForm, {
      props: {
        modelValue: {},
      },
    });

    const nameInput = wrapper.find('input');
    expect(nameInput.exists()).toBe(true);

    await nameInput.setValue('Test');

    const form = wrapper.findComponent({ ref: 'form' });
    await form.trigger('submit');

    await flushPromises();

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')).toEqual([
      [
        {
          name: 'Test',
        },
      ],
    ]);
  });
});
