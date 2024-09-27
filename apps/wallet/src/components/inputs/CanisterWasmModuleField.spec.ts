import { flushPromises } from '@vue/test-utils';
import { Mock, beforeEach, describe, expect, it, vi } from 'vitest';
import { VFileInput } from 'vuetify/components';
import logger from '~/core/logger.core';
import { mount } from '~/test.utils';
import { arrayBufferToHashHex } from '~/utils/crypto.utils';
import { readFileAsArrayBuffer } from '~/utils/file.utils';
import CanisterWasmModuleField from './CanisterWasmModuleField.vue';

// Mock utility functions
vi.mock('~/utils/file.utils', () => ({
  readFileAsArrayBuffer: vi.fn(),
}));
vi.mock('~/utils/crypto.utils', () => ({
  arrayBufferToHashHex: vi.fn(),
}));

describe('CanisterWasmModuleField', () => {
  const fileContent = new Uint8Array([1, 2, 3, 4, 5]);
  const file = new File([fileContent.buffer], 'test.wasm', { type: 'application/wasm' });

  beforeEach(() => {
    vi.clearAllMocks();
    (readFileAsArrayBuffer as Mock).mockResolvedValue(fileContent.buffer);
    (arrayBufferToHashHex as Mock).mockResolvedValue('mockedChecksum');
  });

  it('renders with empty input', () => {
    const input = mount(CanisterWasmModuleField, {
      props: { modelValue: undefined },
    });

    expect(input.exists()).toBe(true);
    expect(input.findComponent(VFileInput).exists()).toBe(true);
  });

  it('emits an update when file is added', async () => {
    const wrapper = mount(CanisterWasmModuleField);

    const wrapperVm = wrapper.vm as typeof wrapper.vm & { wasmModuleFile: File[] };
    wrapperVm.wasmModuleFile = [file];

    await flushPromises();

    expect(wrapper.emitted('update:modelValue')?.[0]).toEqual([new Uint8Array(fileContent.buffer)]);
  });

  it('computes the checksum when file is added', async () => {
    const wrapper = mount(CanisterWasmModuleField);

    const wrapperVm = wrapper.vm as typeof wrapper.vm & {
      wasmModuleFile: File[];
      wasmModuleChecksum: string;
    };
    wrapperVm.wasmModuleFile = [file];

    await flushPromises();

    expect(wrapperVm.wasmModuleChecksum).toEqual('mockedChecksum');
  });

  it('emits an update when file is removed', async () => {
    const wrapper = mount(CanisterWasmModuleField);

    const wrapperVm = wrapper.vm as typeof wrapper.vm & {
      wasmModuleFile: File[];
      wasmModuleChecksum: string;
    };
    wrapperVm.wasmModuleFile = [file];

    await flushPromises();

    wrapperVm.wasmModuleFile = [];

    await flushPromises();

    expect(wrapper.emitted('update:modelValue')?.[1]).toEqual([undefined]);
    expect(wrapperVm.wasmModuleChecksum).toBeUndefined();
  });

  it('handles errors when reading the file', async () => {
    (readFileAsArrayBuffer as Mock).mockRejectedValue(new Error('mocked error'));
    vi.spyOn(logger, 'error');

    const wrapper = mount(CanisterWasmModuleField);

    const wrapperVm = wrapper.vm as typeof wrapper.vm & { wasmModuleFile: File[] };
    wrapperVm.wasmModuleFile = [file];

    await flushPromises();

    expect(wrapper.emitted('update:modelValue')?.[0]).toEqual([undefined]);
    expect(logger.error).toHaveBeenCalled();
  });
});
