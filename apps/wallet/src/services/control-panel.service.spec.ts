import { HttpAgent } from '@dfinity/agent';
import { describe, expect, it, vi } from 'vitest';
import { RegistryEntry } from '~/generated/control-panel/control_panel.did';
import { ControlPanelService } from '~/services/control-panel.service';

const mockWasmModuleEntry = (name: string, version: string): RegistryEntry => ({
  id: '',
  name,
  categories: [],
  tags: [],
  description: 'This is a test description',
  created_at: '2024-01-01T00:00:00Z',
  updated_at: [],
  metadata: [],
  value: {
    WasmModule: {
      dependencies: [],
      version,
      wasm_artifact_id: '',
    },
  },
});

describe('ControlPanelService', () => {
  it('should find module of a given version', async () => {
    const controlPanel = new ControlPanelService(new HttpAgent());
    const firstCallEntry = mockWasmModuleEntry('test', '1.0.0');
    const secondCallEntry = mockWasmModuleEntry('test', '1.0.1');

    vi.spyOn(controlPanel, 'findRegistryEntries')
      .mockReturnValue(Promise.resolve({ entries: [], next_offset: [], total: BigInt(0) }))
      .mockReturnValueOnce(
        Promise.resolve({ entries: [firstCallEntry], next_offset: [BigInt(1)], total: BigInt(2) }),
      )
      .mockReturnValueOnce(
        Promise.resolve({ entries: [secondCallEntry], next_offset: [], total: BigInt(2) }),
      );

    vi.spyOn(controlPanel, 'getRegistryEntry').mockReturnValue(
      Promise.resolve({ entry: secondCallEntry }),
    );

    const entry = await controlPanel.findModuleVersionRegistryEntry('test', '1.0.1');

    expect(controlPanel.findRegistryEntries).toHaveBeenCalledTimes(2);
    expect(controlPanel.getRegistryEntry).toHaveBeenCalledTimes(1);
    expect(entry).toEqual(mockWasmModuleEntry('test', '1.0.1'));
  });

  it('should return null if module version is not found', async () => {
    const controlPanel = new ControlPanelService(new HttpAgent());

    vi.spyOn(controlPanel, 'findRegistryEntries').mockReturnValue(
      Promise.resolve({ entries: [], next_offset: [], total: BigInt(0) }),
    );

    const entry = await controlPanel.findModuleVersionRegistryEntry('test', '1.0.1');

    expect(entry).toBeNull();
  });
});
