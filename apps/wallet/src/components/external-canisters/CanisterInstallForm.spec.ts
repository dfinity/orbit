import { Principal } from '@dfinity/principal';
import { describe, expect, it } from 'vitest';
import WasmMemoryPersistenceSelect from '~/components/inputs/WasmMemoryPersistenceSelect.vue';
import { mount } from '~/test.utils';
import CanisterInstallForm from './CanisterInstallForm.vue';
import { CanisterInstallModel } from './external-canisters.types';

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

  it('shows the upgrade options only for the upgrade mode', () => {
    const upgradeForm = mount(CanisterInstallForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), mode: { upgrade: [] } },
      },
    });
    expect(upgradeForm.find('[name="wasm_memory_persistence"]').exists()).toBe(true);
    expect(upgradeForm.find('[name="skip_pre_upgrade"]').exists()).toBe(true);

    const installForm = mount(CanisterInstallForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), mode: { install: null } },
      },
    });
    expect(installForm.find('[name="wasm_memory_persistence"]').exists()).toBe(false);
    expect(installForm.find('[name="skip_pre_upgrade"]').exists()).toBe(false);
  });

  it('writes the selected wasm memory persistence into the upgrade mode', async () => {
    const form = mount(CanisterInstallForm, {
      props: {
        modelValue: { canisterId: Principal.anonymous(), mode: { upgrade: [] } },
      },
    });

    const select = form.findComponent(WasmMemoryPersistenceSelect);
    expect(select.exists()).toBe(true);

    select.vm.$emit('update:modelValue', { keep: null });
    await form.vm.$nextTick();

    const emitted = form.emitted('update:modelValue') as CanisterInstallModel[][] | undefined;
    expect(emitted).toBeTruthy();
    const latest = emitted![emitted!.length - 1][0];
    expect(latest.mode).toEqual({
      upgrade: [{ wasm_memory_persistence: [{ keep: null }], skip_pre_upgrade: [] }],
    });
  });

  it('collapses the upgrade options back to an empty upgrade when cleared', async () => {
    const form = mount(CanisterInstallForm, {
      props: {
        modelValue: {
          canisterId: Principal.anonymous(),
          mode: { upgrade: [{ wasm_memory_persistence: [{ keep: null }], skip_pre_upgrade: [] }] },
        },
      },
    });

    const select = form.findComponent(WasmMemoryPersistenceSelect);
    // Vuetify's `clearable` emits `undefined`/`null`.
    select.vm.$emit('update:modelValue', undefined);
    await form.vm.$nextTick();

    const emitted = form.emitted('update:modelValue') as CanisterInstallModel[][] | undefined;
    expect(emitted).toBeTruthy();
    const latest = emitted![emitted!.length - 1][0];
    expect(latest.mode).toEqual({ upgrade: [] });
  });
});
