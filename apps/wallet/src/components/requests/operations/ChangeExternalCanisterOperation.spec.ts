import { Principal } from '@dfinity/principal';
import { flushPromises } from '@vue/test-utils';
import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  CanisterInstallMode,
  ChangeExternalCanisterOperation as ChangeExternalCanisterOperationDTO,
  Request,
} from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { mount } from '~/test.utils';
import ChangeExternalCanisterOperation from './ChangeExternalCanisterOperation.vue';

type ExternalCanisterLookup = Awaited<
  ReturnType<ReturnType<typeof services>['station']['getExternalCanisterByCanisterId']>
>;

const canisterId = Principal.fromText('rrkah-fqaaa-aaaaa-aaaaq-cai');
const moduleChecksum = 'a'.repeat(64);
const argChecksum = 'b'.repeat(64);

const operationWithMode = (mode: CanisterInstallMode): ChangeExternalCanisterOperationDTO => ({
  mode,
  canister_id: canisterId,
  module_checksum: moduleChecksum,
  arg_checksum: [argChecksum],
});

const requestWith = (operation: ChangeExternalCanisterOperationDTO): Request => ({
  id: 'request-id',
  title: 'Upgrade canister',
  summary: [],
  status: { Created: null },
  approvals: [],
  created_at: '',
  execution_plan: { Immediate: null },
  expiration_dt: '',
  requested_by: 'requester-id',
  tags: [],
  deduplication_key: [],
  operation: { ChangeExternalCanister: operation },
});

const mountOperation = (operation: ChangeExternalCanisterOperationDTO, mode: 'list' | 'detail') =>
  mount(ChangeExternalCanisterOperation, {
    props: {
      request: requestWith(operation),
      operation,
      mode,
    },
  });

const keepUpgrade: CanisterInstallMode = {
  upgrade: [{ wasm_memory_persistence: [{ keep: null }], skip_pre_upgrade: [] }],
};

describe('ChangeExternalCanisterOperation', () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('shows the canister id, install mode and checksums', () => {
    const wrapper = mountOperation(operationWithMode({ install: null }), 'list');

    expect(wrapper.find('[data-test-id="change-canister-target"]').text()).toBe(
      canisterId.toText(),
    );
    expect(wrapper.find('[data-test-id="change-canister-mode"]').text()).toBe('Install');
    expect(wrapper.find('[data-test-id="change-canister-module-checksum"]').exists()).toBe(true);
    expect(wrapper.find('[data-test-id="change-canister-arg-checksum"]').exists()).toBe(true);
  });

  it('shows the full checksums in detail mode', async () => {
    vi.spyOn(services().station, 'getExternalCanisterByCanisterId').mockRejectedValueOnce(
      new Error('not found'),
    );

    const wrapper = mountOperation(operationWithMode({ reinstall: null }), 'detail');
    await flushPromises();

    expect(wrapper.find('[data-test-id="change-canister-mode"]').text()).toBe('Reinstall');
    expect(wrapper.find('[data-test-id="change-canister-module-checksum"]').text()).toBe(
      moduleChecksum,
    );
    expect(wrapper.find('[data-test-id="change-canister-arg-checksum"]').text()).toBe(argChecksum);
  });

  it('omits the argument checksum row when the request has no argument', () => {
    const wrapper = mountOperation(
      { ...operationWithMode({ install: null }), arg_checksum: [] },
      'list',
    );

    expect(wrapper.find('[data-test-id="change-canister-arg-checksum"]').exists()).toBe(false);
  });

  it('shows the wasm memory persistence of an upgrade in list mode when it is set', () => {
    const wrapper = mountOperation(operationWithMode(keepUpgrade), 'list');

    expect(wrapper.find('[data-test-id="change-canister-mode"]').text()).toBe('Upgrade');
    expect(wrapper.find('[data-test-id="change-canister-wasm-memory-persistence"]').text()).toBe(
      'Keep',
    );
    expect(wrapper.text()).toContain('Wasm Memory Persistence');
  });

  it('shows the skip pre-upgrade flag of an upgrade when it is set', () => {
    const wrapper = mountOperation(
      operationWithMode({
        upgrade: [{ wasm_memory_persistence: [{ replace: null }], skip_pre_upgrade: [true] }],
      }),
      'list',
    );

    expect(wrapper.find('[data-test-id="change-canister-wasm-memory-persistence"]').text()).toBe(
      'Replace',
    );
    expect(wrapper.find('[data-test-id="change-canister-skip-pre-upgrade"]').text()).toBe('Yes');
  });

  it('hides the upgrade options in list mode when the upgrade does not set them', () => {
    const wrapper = mountOperation(operationWithMode({ upgrade: [] }), 'list');

    expect(wrapper.find('[data-test-id="change-canister-wasm-memory-persistence"]').exists()).toBe(
      false,
    );
    expect(wrapper.find('[data-test-id="change-canister-skip-pre-upgrade"]').exists()).toBe(false);
  });

  it('shows the effective upgrade options in detail mode for a plain upgrade', async () => {
    vi.spyOn(services().station, 'getExternalCanisterByCanisterId').mockRejectedValueOnce(
      new Error('not found'),
    );

    const wrapper = mountOperation(operationWithMode({ upgrade: [] }), 'detail');
    await flushPromises();

    expect(wrapper.find('[data-test-id="change-canister-wasm-memory-persistence"]').text()).toBe(
      'Default (replace)',
    );
    expect(wrapper.find('[data-test-id="change-canister-skip-pre-upgrade"]').text()).toBe('No');
  });

  it('does not show upgrade options for install and reinstall requests', async () => {
    vi.spyOn(services().station, 'getExternalCanisterByCanisterId').mockRejectedValue(
      new Error('not found'),
    );

    for (const mode of [{ install: null }, { reinstall: null }] as CanisterInstallMode[]) {
      const wrapper = mountOperation(operationWithMode(mode), 'detail');
      await flushPromises();

      expect(
        wrapper.find('[data-test-id="change-canister-wasm-memory-persistence"]').exists(),
      ).toBe(false);
      expect(wrapper.find('[data-test-id="change-canister-skip-pre-upgrade"]').exists()).toBe(
        false,
      );
    }
  });

  it('resolves the canister name in detail mode', async () => {
    const lookup = vi
      .spyOn(services().station, 'getExternalCanisterByCanisterId')
      .mockResolvedValueOnce({ canister: { name: 'Backend' } } as ExternalCanisterLookup);

    const wrapper = mountOperation(operationWithMode(keepUpgrade), 'detail');
    await flushPromises();

    expect(lookup).toHaveBeenCalledWith(canisterId);
    expect(wrapper.find('[data-test-id="change-canister-target"]').text()).toBe(
      `Backend (${canisterId.toText()})`,
    );
  });

  it('falls back to the canister id when the canister cannot be resolved', async () => {
    vi.spyOn(services().station, 'getExternalCanisterByCanisterId').mockRejectedValueOnce(
      new Error('not found'),
    );

    const wrapper = mountOperation(operationWithMode(keepUpgrade), 'detail');
    await flushPromises();

    expect(wrapper.find('[data-test-id="change-canister-target"]').text()).toBe(
      canisterId.toText(),
    );
  });

  it('does not look up the canister in list mode', async () => {
    const lookup = vi.spyOn(services().station, 'getExternalCanisterByCanisterId');

    mountOperation(operationWithMode(keepUpgrade), 'list');
    await flushPromises();

    expect(lookup).not.toHaveBeenCalled();
  });
});
