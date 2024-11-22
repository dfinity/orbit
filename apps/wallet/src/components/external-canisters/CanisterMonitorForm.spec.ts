import { Principal } from '@dfinity/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import { flushPromises } from '@vue/test-utils';
import CanisterMonitorForm from '~/components/external-canisters/CanisterMonitorForm.vue';

describe('CanisterMonitorForm', () => {
  it('hides the canisterId when display is set to false', () => {
    const form = mount(CanisterMonitorForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), strategy: undefined },
        display: { canisterId: false },
      },
    });
    const canisterIdInput = form.find('[name="canisterId"]');

    expect(canisterIdInput.exists()).toBe(false);
  });

  it('triggers submit when Always strategy input triggers keydown.enter', async () => {
    const form = mount(CanisterMonitorForm, {
      props: {
        modelValue: {
          canisterId: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
          strategy: { Always: BigInt(1_000_000_000_000) },
        },
      },
    });

    const cyclesInput = form.find('[name="always_fund_cycles"]');

    await cyclesInput.trigger('keydown.enter');
    await flushPromises();

    const isValidEvents = form.emitted('valid') ?? [];
    const submitEvents = form.emitted('submitting') ?? [];

    expect(isValidEvents.pop()).toEqual([true]);
    expect(submitEvents.pop()).toEqual([true]);
  });
});
