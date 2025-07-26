import { Principal } from '@icp-sdk/core/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import CanisterInstallForm from './CanisterInstallForm.vue';

describe('CanisterInstallForm', () => {
  it('hides the canisterId when display is set to false', () => {
    const form = mount(CanisterInstallForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous() },
        display: { canisterId: false },
      },
    });
    const canisterIdInput = form.find('[name="canister_id"]');

    expect(canisterIdInput.exists()).toBe(false);
  });

  it('shows the canisterId when display is set to true', () => {
    const form = mount(CanisterInstallForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous() },
        display: { canisterId: true },
      },
    });
    const canisterIdInput = form.find('[name="canister_id"]');

    expect(canisterIdInput.exists()).toBe(true);
  });
});
