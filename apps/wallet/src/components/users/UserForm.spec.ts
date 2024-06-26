import { Principal } from '@dfinity/principal';
import { flushPromises } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import UserForm from './UserForm.vue';

describe('UserForm', () => {
  it('renders properly', () => {
    const wrapper = mount(UserForm, {
      props: {
        modelValue: {},
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('changes the model on changes', async () => {
    const wrapper = mount(UserForm, {
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
    const wrapper = mount(UserForm, {
      props: {
        modelValue: {
          groups: [{ id: '1', name: 'test' }],
          identities: [Principal.anonymous()],
          status: { Active: null },
        },
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
          groups: [{ id: '1', name: 'test' }],
          identities: [Principal.anonymous()],
          status: { Active: null },
        },
      ],
    ]);
  });
});
