import { Principal } from '@dfinity/principal';
import { describe, expect, it } from 'vitest';
import CanisterWasmMemoryPersistenceSelect from '~/components/inputs/CanisterWasmMemoryPersistenceSelect.vue';
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

  it('hides the upgrade options unless the mode is upgrade', () => {
    const form = mount(CanisterInstallForm, {
      props: {
        modelValue: { mode: { install: null } },
      },
    });

    expect(form.findComponent(CanisterWasmMemoryPersistenceSelect).exists()).toBe(false);
  });

  it('shows the upgrade options when the mode is upgrade', () => {
    const form = mount(CanisterInstallForm, {
      props: {
        modelValue: { mode: { upgrade: [] } },
      },
    });

    expect(form.findComponent(CanisterWasmMemoryPersistenceSelect).exists()).toBe(true);
  });

  it('folds the selected wasm_memory_persistence into the upgrade mode', async () => {
    const form = mount(CanisterInstallForm, {
      props: {
        modelValue: { mode: { upgrade: [] } },
      },
    });

    form.findComponent(CanisterWasmMemoryPersistenceSelect).vm.$emit('update:modelValue', {
      keep: null,
    });
    await form.vm.$nextTick();

    const updates = form.emitted('update:modelValue');
    expect(updates).toBeTruthy();
    expect(updates?.at(-1)?.[0]).toEqual({
      mode: { upgrade: [{ wasm_memory_persistence: [{ keep: null }], skip_pre_upgrade: [] }] },
    });
  });

  it('collapses back to a plain upgrade when the options are cleared', async () => {
    const form = mount(CanisterInstallForm, {
      props: {
        modelValue: {
          mode: { upgrade: [{ wasm_memory_persistence: [{ keep: null }], skip_pre_upgrade: [] }] },
        },
      },
    });

    form.findComponent(CanisterWasmMemoryPersistenceSelect).vm.$emit('update:modelValue', undefined);
    await form.vm.$nextTick();

    const updates = form.emitted('update:modelValue');
    expect(updates?.at(-1)?.[0]).toEqual({ mode: { upgrade: [] } });
  });
});
