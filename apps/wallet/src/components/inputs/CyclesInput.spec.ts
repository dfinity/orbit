import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import { CyclesUnit } from '~/types/app.types';
import CyclesInput from './CyclesInput.vue';

describe('CyclesInput', () => {
  it('renders with no cycles in the input', () => {
    const input = mount(CyclesInput);
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.exists()).toBe(true);
    expect(displayInput.props('modelValue')).toEqual(undefined);
  });

  it('default with cycles in the e8s format', async () => {
    const input = mount(CyclesInput, {
      props: {
        modelValue: BigInt(1_000_000),
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.props('modelValue')).toEqual(1_000_000);
    expect(input.props('modelValue')).toEqual(BigInt(1_000_000));
  });

  it('renders with cycles in the given unit', async () => {
    const input = mount(CyclesInput, {
      props: {
        modelValue: BigInt(1_000_000),
        unit: CyclesUnit.Million,
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.props('modelValue')).toEqual(1);
    expect(input.props('modelValue')).toEqual(BigInt(1_000_000));
  });

  it('on change cycles the emitted change is always in the e8s format', async () => {
    const input = mount(CyclesInput, {
      props: {
        modelValue: BigInt(1_000_000_000),
        unit: CyclesUnit.Billion,
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.props('modelValue')).toEqual(1);
    expect(input.props('modelValue')).toEqual(BigInt(1_000_000_000));

    await displayInput.setValue(2);

    expect(displayInput.props('modelValue')).toEqual(2);
    expect(input.emitted('update:modelValue')).toEqual([
      [BigInt(2_000_000_000)],
      [BigInt(2_000_000_000)],
    ]);
  });
});
