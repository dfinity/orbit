import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import CanisterIdField from './CanisterIdField.vue';

describe('CanisterIdField', () => {
  it('renders with no canisterId in the input', () => {
    const input = mount(CanisterIdField);
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.exists()).toBe(true);
    expect(displayInput.props('modelValue')).toEqual(undefined);
  });

  it('renders with the provided cansiterId', async () => {
    const input = mount(CanisterIdField, {
      props: {
        modelValue: Principal.anonymous(),
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.props('modelValue')).toEqual(Principal.anonymous().toText());
    expect(input.props('modelValue')).toEqual(Principal.anonymous());
  });

  it('on change canisterId the emitted change is with the principal format', async () => {
    const input = mount(CanisterIdField, {
      props: {
        modelValue: Principal.anonymous(),
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });
    const updatedPrincipal = Principal.fromUint8Array(Uint8Array.from([1, 2, 3, 4, 5]));

    await displayInput.setValue(updatedPrincipal.toText());

    expect(displayInput.props('modelValue')).toEqual(updatedPrincipal.toText());
    expect(input.emitted('update:modelValue')).toEqual([[updatedPrincipal]]);
  });

  it('on invalid change sets the canister id to undefined', async () => {
    const input = mount(CanisterIdField, {
      props: {
        modelValue: Principal.anonymous(),
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });

    await displayInput.setValue('invalid');

    expect(displayInput.props('modelValue')).toEqual('invalid');
    expect(input.emitted('update:modelValue')).toEqual([[undefined]]);
  });
});
