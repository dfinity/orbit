import { Principal } from '@dfinity/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import CanisterTopUpForm from './CanisterTopUpForm.vue';
import { flushPromises } from '@vue/test-utils';

describe('CanisterTopUpForm', () => {
  it('hides the canisterId when display is set to false', () => {
    const form = mount(CanisterTopUpForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), cycles: undefined },
        display: { canisterId: false },
      },
    });
    const canisterIdInput = form.find('[name="canisterId"]');

    expect(canisterIdInput.exists()).toBe(false);
  });

  it('triggers submit when cycles input triggers keydown.enter', async () => {
    const form = mount(CanisterTopUpForm, {
      props: {
        modelValue: {
          canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
          cycles: BigInt(1_000_000_000_000),
        },
      },
    });

    const cyclesInput = form.find('[name="cycles"]');

    await cyclesInput.trigger('keydown.enter');
    await flushPromises();

    const isValidEvents = form.emitted('valid') ?? [];
    const submitEvents = form.emitted('submit') ?? [];

    expect(isValidEvents.pop()).toEqual([true]);
    expect(submitEvents.pop()).toEqual([
      {
        canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
        cycles: BigInt(1_000_000_000_000),
      },
    ]);
  });
});
