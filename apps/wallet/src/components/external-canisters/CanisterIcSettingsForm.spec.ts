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

  it('shows disabled select value for allowed_viewers enum type', () => {
    const form = mount(CanisterIcSettingsForm, {
      props: {
        modelValue: {
          canisterId: Principal.anonymous(),
          log_visibility: { allowed_viewers: [Principal.anonymous()] },
        },
        display: { canisterId: true },
      },
    });
    const select = form.findComponent({ name: 'VSelect' });

    expect(select.exists()).toBe(true);
    expect(select.vm.disabled).toBe(true);
    expect(select.vm.modelValue).toContain('unsupported');
    expect(form.vm.modelValue.log_visibility).toStrictEqual({
      allowed_viewers: [Principal.anonymous()],
    });
  });

  it('shows two select items for log visibility', () => {
    const form = mount(CanisterIcSettingsForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), log_visibility: { public: null } },
        display: { canisterId: true },
      },
    });

    const select = form.findComponent({ name: 'VSelect' });

    expect(select.exists()).toBe(true);
    expect(select.vm.items.length).toEqual(2);
  });
});
