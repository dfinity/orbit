import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils.ts';
import CanisterMonitorForm from '~/components/external-canisters/monitor/CanisterMonitorForm.vue';

describe('CanisterMonitorForm', () => {
  it('hides the canisterId when display is set to false', () => {
    const form = mount(CanisterMonitorForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), fundingStrategy: undefined },
        display: { canisterId: false },
      },
    });
    const canisterIdInput = form.find('[name="canisterId"]');

    expect(canisterIdInput.exists()).toBe(false);
  });
});
