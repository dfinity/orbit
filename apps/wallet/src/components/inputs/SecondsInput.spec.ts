import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import { TimeUnit } from '~/types/app.types';
import SecondsInput from '~/components/inputs/SecondsInput.vue';

describe('SecondsInput', () => {
  it('renders with no seconds in the input', () => {
    const input = mount(SecondsInput);
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.exists()).toBe(true);
    expect(displayInput.props('modelValue')).toEqual(undefined);
  });

  it('default with time in seconds format', async () => {
    const input = mount(SecondsInput, {
      props: {
        modelValue: BigInt(1_000_000),
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.props('modelValue')).toEqual(1_000_000);
    expect(input.props('modelValue')).toEqual(BigInt(1_000_000));
  });

  it('renders with time in the given unit', async () => {
    const input = mount(SecondsInput, {
      props: {
        modelValue: BigInt(6_000_000),
        unit: TimeUnit.Minutes,
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.props('modelValue')).toEqual(100_000);
    expect(input.props('modelValue')).toEqual(BigInt(6_000_000));
  });

  it('on change cycles the emitted change is always in the seconds format', async () => {
    const input = mount(SecondsInput, {
      props: {
        modelValue: BigInt(6_000_000_000),
        unit: TimeUnit.Minutes,
      },
    });
    const displayInput = input.findComponent({ name: 'VTextField' });

    expect(displayInput.props('modelValue')).toEqual(100_000_000);
    expect(input.props('modelValue')).toEqual(BigInt(6_000_000_000));

    await displayInput.setValue(10);

    expect(displayInput.props('modelValue')).toEqual(10);
    expect(input.emitted('update:modelValue')).toEqual([[BigInt(600)], [BigInt(600)]]);
  });
});
