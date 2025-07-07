import { Principal } from '@icp-sdk/core/principal';
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

  it('shows three select items for log visibility', () => {
    const form = mount(CanisterIcSettingsForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), log_visibility: { public: null } },
        display: { canisterId: true },
      },
    });

    const select = form.findComponent({ name: 'VSelect' });

    expect(select.exists()).toBe(true);
    expect(select.vm.items.length).toEqual(3);
  });
});
