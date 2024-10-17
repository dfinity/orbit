import { Principal } from '@dfinity/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import CanisterConfigureMethodCallForm from './CanisterConfigureMethodCallForm.vue';

describe('CanisterConfigureMethodCallForm', () => {
  it('renders form with method_name filled', () => {
    const form = mount(CanisterConfigureMethodCallForm, {
      props: {
        modelValue: {
          methodName: 'test',
          canisterId: Principal.anonymous(),
          alreadyConfiguredMethods: [],
          permission: {
            auth_scope: { Restricted: null },
            user_groups: [],
            users: [],
          },
          requestPolicies: [],
        },
      },
    });

    const methodNameInput = form.find('[name="method_name"]');

    expect(methodNameInput.exists()).toBe(true);
    expect(methodNameInput.element.getAttribute('value')).toBe('test');
  });

  it('renders form with option to add advanced validation', async () => {
    const canisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');
    const form = mount(CanisterConfigureMethodCallForm, {
      props: {
        modelValue: {
          methodName: 'test',
          canisterId,
          alreadyConfiguredMethods: [],
          permission: {
            auth_scope: { Restricted: null },
            user_groups: [],
            users: [],
          },
          requestPolicies: [],
        },
      },
    });

    await form.find('button.v-expansion-panel-title').trigger('click');

    const addAdvancedValidationBtn = form.find('[data-test-id="add-advanced-validation"]');
    const removeAdvancedValidationBtn = form.find('[data-test-id="remove-advanced-validation"]');

    expect(addAdvancedValidationBtn.exists()).toBe(true);
    expect(removeAdvancedValidationBtn.exists()).toBe(false);
  });

  it('renders form with option to remove advanced validation', async () => {
    const canisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');
    const form = mount(CanisterConfigureMethodCallForm, {
      props: {
        modelValue: {
          methodName: 'test',
          canisterId,
          alreadyConfiguredMethods: [],
          permission: {
            auth_scope: { Restricted: null },
            user_groups: [],
            users: [],
          },
          requestPolicies: [],
          validationMethodName: 'validate_test',
          validationCanisterId: canisterId,
        },
      },
    });

    await form.find('button.v-expansion-panel-title').trigger('click');

    const addAdvancedValidationBtn = form.find('[data-test-id="add-advanced-validation"]');
    const removeAdvancedValidationBtn = form.find('[data-test-id="remove-advanced-validation"]');

    expect(addAdvancedValidationBtn.exists()).toBe(false);
    expect(removeAdvancedValidationBtn.exists()).toBe(true);
  });
});
