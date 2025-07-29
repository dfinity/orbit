import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import CanisterCallForm from './CanisterCallForm.vue';

describe('CanisterCallForm', () => {
  it('renders form with method_name filled', () => {
    const canisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');
    const form = mount(CanisterCallForm, {
      props: {
        modelValue: {
          methodName: 'test',
          canisterId,
        },
      },
    });

    const methodNameInput = form.find('[name="method_name"]');

    expect(methodNameInput.exists()).toBe(true);
    expect(methodNameInput.element.getAttribute('value')).toBe('test');
  });

  it('opening expansion panel shows attach cycles option', async () => {
    const canisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');
    const form = mount(CanisterCallForm, {
      props: {
        modelValue: {
          methodName: 'test',
          canisterId,
        },
      },
    });

    await form.find('button.v-expansion-panel-title').trigger('click');

    const cyclesInput = form.find('[name="cycles"]');

    expect(cyclesInput.exists()).toBe(true);
  });

  it('default is to hide advanced settings and not show cycles input', async () => {
    const canisterId = Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai');
    const form = mount(CanisterCallForm, {
      props: {
        modelValue: {
          methodName: 'test',
          canisterId,
        },
      },
    });

    const cyclesInput = form.find('[name="cycles"]');

    expect(cyclesInput.exists()).toBe(false);
  });
});
