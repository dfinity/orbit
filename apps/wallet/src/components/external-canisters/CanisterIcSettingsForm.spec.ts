import { Principal } from '@dfinity/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import CanisterIcSettingsForm from './CanisterIcSettingsForm.vue';

describe('CanisterIcSettingsForm', () => {
  it('hides the canisterId when display is set to false', () => {
    const form = mount(CanisterIcSettingsForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous() },
        display: { canisterId: false },
      },
    });
    const canisterIdInput = form.find('[name="canister_id"]');

    expect(canisterIdInput.exists()).toBe(false);
  });

  it('shows the canisterId when display is set to true', () => {
    const form = mount(CanisterIcSettingsForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous() },
        display: { canisterId: true },
      },
    });
    const canisterIdInput = form.find('[name="canister_id"]');

    expect(canisterIdInput.exists()).toBe(true);
  });
});
