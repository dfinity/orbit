import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import NamedRuleDialog from './NamedRuleDialog.vue';
import NamedRuleForm from './NamedRuleForm.vue';
import { VCard } from 'vuetify/components';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    getNamedRule: vi.fn().mockImplementation(() =>
      Promise.resolve({
        named_rule: {
          id: '1',
          name: 'Test Rule',
          description: ['Test Description'],
          rule: {
            AutoApproved: null,
          },
        },
      }),
    ),
    addNamedRule: vi.fn().mockImplementation(() => Promise.resolve({} as Request)),
    editNamedRule: vi.fn().mockImplementation(() => Promise.resolve({} as Request)),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('NamedRuleDialog', () => {
  it.skip('renders correctly', () => {
    const wrapper = mount(NamedRuleDialog, {
      props: {
        open: true,
      },
    });
    expect(wrapper.exists()).toBe(true);
  });

  it('loads and displays existing named rule', async () => {
    const wrapper = mount(NamedRuleDialog, {
      props: {
        open: true,
        namedRuleId: '1',
      },
    });

    await flushPromises();

    // expect getNamedRule to be called
    expect(services().station.getNamedRule).toHaveBeenCalledWith('1');

    const form = wrapper.findComponent(NamedRuleForm);

    const name = form.find('input[name="name"]').element as HTMLInputElement;
    const description = form.find('input[name="description"]').element as HTMLInputElement;

    expect(name.value).toBe('Test Rule');
    expect(description.value).toBe('Test Description');
  });

  it('creates new named rule', async () => {
    const wrapper = mount(NamedRuleDialog, {
      props: {
        open: true,
      },
    });

    await flushPromises();

    const dialogContents = wrapper.findComponent(VCard);

    const form = wrapper.findComponent(NamedRuleForm);

    // Fill out form
    await form.find('input[name="name"]').setValue('New Rule');
    await form.find('input[name="description"]').setValue('New Description');

    // Set a simple rule
    const ruleBuilder = form.findComponent({ name: 'RuleBuilder' });
    await ruleBuilder.vm.$emit('update:modelValue', {
      AutoApproved: null,
    });

    await flushPromises();

    // Find and click save button
    const saveButton = dialogContents.find('button[data-test-id="save-named-rule"]');
    expect(saveButton.exists()).toBe(true);
    await saveButton.trigger('click');

    await flushPromises();

    // Verify addNamedRule was called with correct data
    expect(services().station.addNamedRule).toHaveBeenCalledWith({
      name: 'New Rule',
      description: ['New Description'],
      rule: {
        AutoApproved: null,
      },
    });
  });

  it('edits existing named rule', async () => {
    const wrapper = mount(NamedRuleDialog, {
      props: {
        open: true,
        namedRuleId: '1',
      },
    });

    await flushPromises();

    const dialogContents = wrapper.findComponent(VCard);
    
    const form = wrapper.findComponent(NamedRuleForm);

    // Update form fields
    await form.find('input[name="name"]').setValue('Updated Rule');
    await form.find('input[name="description"]').setValue('Updated Description');

    // Update rule
    const ruleBuilder = form.findComponent({ name: 'RuleBuilder' });
    await ruleBuilder.vm.$emit('update:modelValue', {
      AutoApproved: null,
    });

    await flushPromises();

    // Find and click save button
    const saveButton = dialogContents.find('button[data-test-id="save-named-rule"]');
    await saveButton.trigger('click');

    await flushPromises();

    // Verify editNamedRule was called with correct data
    expect(services().station.editNamedRule).toHaveBeenCalledWith({
      named_rule_id: '1',
      name: ['Updated Rule'],
      description: ['Updated Description'],
      rule: [{
        AutoApproved: null,
      },]
    });
  });

  it('handles readonly mode correctly', async () => {
    const wrapper = mount(NamedRuleDialog, {
      props: {
        open: true,
        namedRuleId: '1',
        readonly: true,
      },
    });

    await flushPromises();

    const dialogContents = wrapper.findComponent(VCard);

    const form = wrapper.findComponent(NamedRuleForm);

    // Verify inputs are disabled
    const nameInput = form.find('input[name="name"]');
    expect(nameInput.attributes('disabled')).toBeDefined();

    const descriptionInput = form.find('input[name="description"]');
    expect(descriptionInput.attributes('disabled')).toBeDefined();

    // Verify save button is not present
    const saveButton = dialogContents.find('button[data-test-id="save-named-rule"]');
    expect(saveButton.exists()).toBe(false);
  });
});
