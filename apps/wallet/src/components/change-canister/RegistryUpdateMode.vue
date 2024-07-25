<template>
  <VAlert type="info" density="compact" variant="tonal" class="mb-4">
    {{ $t('app.update_recommended_latest') }}
  </VAlert>

  <p v-if="checkingForUpdates" class="text-h6 py-3" data-test-id="loading-screen">
    <VProgressCircular indeterminate color="primary" class="mb-1 mr-2" :size="20" :width="2" />
    {{ $t('app.checking_for_updates') }}
  </p>

  <p v-else-if="!isUpdateAvailable" class="text-h6 py-3" data-test-id="latest-screen">
    {{ $t('app.update_already_latest_version') }}
  </p>

  <p v-else class="text-h6 py-3" data-test-id="update-available-screen">
    {{ $t('app.update_available') }}
  </p>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VAlert, VProgressCircular } from 'vuetify/components';
import { useDefaultUpgradeFormValue } from '~/composables/change-canister.composable';
import logger from '~/core/logger.core';
import { services } from '~/plugins/services.plugin';
import { useStationStore } from '~/stores/station.store';
import { toArrayBuffer } from '~/utils/helper.utils';
import { ChangeCanisterFormValue } from './change-canister.types';

const i18n = useI18n();
const props = defineProps<{
  modelValue: ChangeCanisterFormValue;
}>();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ChangeCanisterFormValue): void;
  (event: 'valid', payload: boolean): void;
  (event: 'loading', payload: boolean): void;
}>();

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const station = useStationStore();
const preparingArtifacts = ref(false);

onMounted(() => {
  emit('valid', false);
  if (!station.versionManagement.loading) {
    station.checkVersionUpdates();
  }
});

const checkingForUpdates = computed(
  () => station.versionManagement.loading || preparingArtifacts.value,
);

const isUpdateAvailable = computed(
  () =>
    station.versionManagement.nextStationVersion || station.versionManagement.nextUpgraderVersion,
);

const createAutomatedComment = (input: {
  name: string;
  version: string;
  repositoryUrl: string;
}): string => {
  const summary = i18n.t('app.update_automated_comment.summary', {
    name: input.name,
    version: input.version,
  });

  const verifyInstructions = i18n.t('app.update_automated_comment.verify_instructions');

  return `${summary}\n\n${verifyInstructions}\n
  1. git clone ${input.repositoryUrl}
  2. cd orbit
  3. git checkout @orbit/${input.name}-v${input.version}
  4. ./scripts/docker-build.sh --${input.name}
  5. cat ./artifacts/${input.name}/${input.name}.wasm.gz.sha256\n`;
};

const prepareUpdateWithWasmModule = async (): Promise<void> => {
  try {
    preparingArtifacts.value = true;
    emit('valid', false);

    modelValue.value = useDefaultUpgradeFormValue();
    const controlPanel = services().controlPanel;
    const { nextStationVersion, nextUpgraderVersion } = station.versionManagement;

    if (!nextStationVersion && !nextUpgraderVersion) {
      return;
    }

    // If there is a new upgrader version, it needs to be updated first
    const registryEntry = nextUpgraderVersion
      ? await controlPanel.findModuleVersionRegistryEntry('@orbit/upgrader', nextUpgraderVersion)
      : await controlPanel.findModuleVersionRegistryEntry('@orbit/station', nextStationVersion!);

    if (!registryEntry) {
      logger.warn('Failed to find registry entry');
      return;
    }

    const repositoryUrl =
      registryEntry.metadata.find(metadata => metadata.key === 'url')?.value ??
      'https://github.com/dfinity/orbit';

    if (nextUpgraderVersion) {
      modelValue.value.target = { UpgradeUpgrader: null };
      modelValue.value.comment = createAutomatedComment({
        name: 'upgrader',
        version: nextUpgraderVersion,
        repositoryUrl,
      });
    } else {
      modelValue.value.target = { UpgradeStation: null };
      modelValue.value.comment = createAutomatedComment({
        name: 'station',
        version: nextStationVersion!,
        repositoryUrl,
      });
    }

    // Fetch the artifact with an update call to make sure it's verified
    const { artifact } = await controlPanel.getArtifact(
      { artifact_id: registryEntry.value.WasmModule.wasm_artifact_id },
      true,
    );

    modelValue.value = {
      ...modelValue.value,
      wasmModule: toArrayBuffer(artifact.artifact),
      wasmInitArg: undefined,
    };

    emit('valid', true);
  } catch (err) {
    logger.error('Failed to prepare update with wasm module', err);
  } finally {
    preparingArtifacts.value = false;
  }
};

watch(
  () => station.versionManagement.loading,
  checking => {
    if (!checking) {
      prepareUpdateWithWasmModule();
    }
  },
);

watch(
  () => preparingArtifacts.value,
  loadingArtifacts => emit('loading', loadingArtifacts),
);
</script>
