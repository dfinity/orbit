import { describe, expect, it } from 'vitest';
import NamedRuleForm from './NamedRuleForm.vue';
import { mount } from '~/test.utils';

describe('NamedRuleForm', () => {
  it('renders correctly for creation', () => {
    const wrapper = mount(NamedRuleForm, {
      props: {
        modelValue: {},
      },
    });
    expect(wrapper.exists()).toBe(true);
  });

  it('shows ID field only when id is provided and display.id is true', async () => {
    const wrapper = mount(NamedRuleForm, {
      props: {
        modelValue: {
          id: 'test-id',
        },
        display: {
          id: true,
        },
      },
    });

    expect(wrapper.find('input[name="id"]').exists()).toBe(true);

    // Update props to hide ID
    wrapper.setProps({
      display: {
        id: false,
      },
    });

    await wrapper.vm.$nextTick();

    expect(wrapper.find('input[name="id"]').exists()).toBe(false);
  });

  it('handles description updates correctly', async () => {
    const wrapper = mount(NamedRuleForm, {
      props: {
        modelValue: {
          name: 'Test Rule',
          description: ['Initial description'],
          rule: { AutoApproved: null },
        },
      },
    });

    const descriptionInput = wrapper.find('input[name="description"]');
    await descriptionInput.setValue('Updated description');

    await wrapper.vm.$nextTick();

    expect(wrapper.emitted('update:modelValue')?.[0]?.[0]).toEqual({
      name: 'Test Rule',
      description: ['Updated description'],
      rule: { AutoApproved: null },
    });

    // Test clearing description
    await descriptionInput.setValue('');

    const emitted = wrapper.emitted('update:modelValue')?.[1]?.[0] as {
      description: string[] | undefined;
    };

    expect(emitted.description).toBeUndefined();
  });

  it('emits valid form state', async () => {
    const wrapper = mount(NamedRuleForm, {
      props: {
        modelValue: {},
      },
    });

    await wrapper.vm.$nextTick();
    expect(wrapper.emitted('valid')).toEqual([[false]]);

    // Fill required name field
    await wrapper.find('input[name="name"]').setValue('Test Rule');

    await wrapper.vm.$nextTick();
    expect(wrapper.emitted('valid')).toEqual([[false], [true]]);
  });

  it('handles view mode correctly', async () => {
    const wrapper = mount(NamedRuleForm, {
      props: {
        modelValue: {
          id: 'test-id',
          name: 'Test Rule',
          description: ['Test Description'],
        },
        mode: 'view',
      },
    });

    // In view mode, inputs should be disabled
    const nameInput = wrapper.find('input[name="name"]');

    // Test that the input is disabled in view mode
    expect(nameInput.attributes('readonly')).toBeDefined();
  });

  it('handles form submission', async () => {
    const wrapper = mount(NamedRuleForm, {
      props: {
        modelValue: {
          name: 'Test Rule',
          rule: { AutoApproved: null },
        },
      },
    });

    await wrapper.find('form').trigger('submit');

    await wrapper.vm.$nextTick();

    expect(wrapper.emitted('submit')?.[0]?.[0]).toEqual({
      name: 'Test Rule',
      rule: { AutoApproved: null },
    });
  });
});
