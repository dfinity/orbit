import { Principal } from '@dfinity/principal';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import CanisterIcSettingsForm from './CanisterIcSettingsForm.vue';
import { flushPromises } from '@vue/test-utils';

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

  it('add controllers btn is clickable', async () => {
    const form = mount(CanisterIcSettingsForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), controllers: [Principal.anonymous()] },
        display: { canisterId: true },
      },
    });

    const btn = form.find("[data-test-id='add-controller-button']");

    expect(btn.exists()).toBe(true);
    expect(btn.attributes().disabled).toBeDefined();

    const controllerInput = form.find("[name='new_controller']");

    expect(controllerInput.exists()).toBe(true);
    controllerInput.setValue('ryjl3-tyaaa-aaaaa-aaaba-cai');

    await flushPromises();

    expect(btn.attributes().disabled).toBeUndefined();
  });
});
