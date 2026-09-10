<template>
  <div class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow>
      <template #name>{{ $t('terms.canister') }}</template>
      <template #content>
        <span data-test-id="change-canister-target">{{ canisterLabel }}</span>
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>{{ $t('terms.mode') }}</template>
      <template #content>
        <span data-test-id="change-canister-mode">{{ installModeLabel }}</span>
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="showWasmMemoryPersistence">
      <template #name>{{ $t('external_canisters.wasm_memory_persistence.label') }}</template>
      <template #content>
        <span data-test-id="change-canister-wasm-memory-persistence">
          {{ wasmMemoryPersistenceLabel }}
        </span>
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="showSkipPreUpgrade">
      <template #name>{{ $t('external_canisters.skip_pre_upgrade.label') }}</template>
      <template #content>
        <span data-test-id="change-canister-skip-pre-upgrade">
          {{ skipPreUpgrade ? $t('terms.yes') : $t('terms.no') }}
        </span>
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>{{ $t('terms.wasm') }}</template>
      <template #content>
        <TextOverflow
          v-if="isListMode"
          :text="props.operation.module_checksum"
          :max-length="24"
          test-id="change-canister-module-checksum"
        />
        <span v-else data-test-id="change-canister-module-checksum">
          {{ props.operation.module_checksum }}
        </span>
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="argChecksum">
      <template #name>{{ $t('terms.arg') }}</template>
      <template #content>
        <TextOverflow
          v-if="isListMode"
          :text="argChecksum"
          :max-length="24"
          test-id="change-canister-arg-checksum"
        />
        <span v-else data-test-id="change-canister-arg-checksum">{{ argChecksum }}</span>
      </template>
    </RequestOperationListRow>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  CanisterUpgradeOptions,
  WasmMemoryPersistence,
} from '~/components/external-canisters/external-canisters.types';
import TextOverflow from '~/components/TextOverflow.vue';
import logger from '~/core/logger.core';
import { ChangeExternalCanisterOperation, Request } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { variantIs } from '~/utils/helper.utils';
import RequestOperationListRow from '../RequestOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: ChangeExternalCanisterOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const i18n = useI18n();
const station = useStationStore();

const isListMode = computed(() => props.mode === 'list');
const isUpgrade = computed(() => variantIs(props.operation.mode, 'upgrade'));

const installModeLabel = computed(() => {
  if (variantIs(props.operation.mode, 'install')) {
    return i18n.t('external_canisters.install_mode.install');
  }

  if (variantIs(props.operation.mode, 'reinstall')) {
    return i18n.t('external_canisters.install_mode.reinstall');
  }

  return i18n.t('external_canisters.install_mode.upgrade');
});

// The optional upgrade options record, only present for `upgrade` requests that
// explicitly set at least one of the options.
const upgradeOptions = computed<CanisterUpgradeOptions | undefined>(() => {
  const mode = props.operation.mode;

  return variantIs(mode, 'upgrade') ? mode.upgrade[0] : undefined;
});

const wasmMemoryPersistence = computed<WasmMemoryPersistence | undefined>(
  () => upgradeOptions.value?.wasm_memory_persistence[0],
);

// `undefined` when the request does not set the option, which differs from an
// explicit `false`.
const skipPreUpgradeOption = computed<boolean | undefined>(
  () => upgradeOptions.value?.skip_pre_upgrade[0],
);

const skipPreUpgrade = computed<boolean>(() => skipPreUpgradeOption.value ?? false);

// The compact list view only surfaces the upgrade options when they were
// explicitly set on the request, while the detail view always shows the
// effective values for an upgrade so reviewers can see what will be applied.
const showWasmMemoryPersistence = computed(
  () => isUpgrade.value && (!isListMode.value || wasmMemoryPersistence.value !== undefined),
);

const showSkipPreUpgrade = computed(
  () => isUpgrade.value && (!isListMode.value || skipPreUpgradeOption.value !== undefined),
);

const wasmMemoryPersistenceLabel = computed(() => {
  const persistence = wasmMemoryPersistence.value;

  if (persistence && variantIs(persistence, 'keep')) {
    return i18n.t('external_canisters.wasm_memory_persistence.keep');
  }

  if (persistence && variantIs(persistence, 'replace')) {
    return i18n.t('external_canisters.wasm_memory_persistence.replace');
  }

  // When the option is omitted the IC applies its default, which is `replace`.
  return i18n.t('external_canisters.wasm_memory_persistence.default');
});

const argChecksum = computed(() => props.operation.arg_checksum?.[0]);

const canisterName = ref<string | undefined>();

const canisterLabel = computed(() => {
  const canisterId = props.operation.canister_id.toText();

  return canisterName.value ? `${canisterName.value} (${canisterId})` : canisterId;
});

const loadCanisterName = async (): Promise<void> => {
  try {
    // The name is shown to approvers next to the canister id as the target of the
    // change, so it is fetched with a verified (certified) call like the request
    // itself rather than a plain query.
    const result = await station.service.getExternalCanisterByCanisterId(
      props.operation.canister_id,
      true,
    );

    canisterName.value = result.canister.name;
  } catch (err) {
    // The canister may no longer be linked to the station or the caller may not
    // have permission to read it, in which case the canister id is shown alone.
    logger.warn(`Could not resolve the name of canister ${props.operation.canister_id}`, err);
  }
};

watch(
  [isListMode, () => props.operation.canister_id.toText()],
  ([isListMode]) => {
    canisterName.value = undefined;

    if (!isListMode) {
      loadCanisterName();
    }
  },
  { immediate: true },
);
</script>
