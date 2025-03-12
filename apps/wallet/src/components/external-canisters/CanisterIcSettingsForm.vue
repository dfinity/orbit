<template>
  <VForm ref="form" @submit.prevent="submit">
    <VContainer class="px-0 py-2">
      <VRow>
        <VCol cols="12" class="pb-0">
          <CanisterIdField
            v-if="props.display.canisterId || !model.canisterId"
            v-model="model.canisterId"
            name="canister_id"
            density="comfortable"
            :readonly="props.readonly"
            required
          />
        </VCol>
        <VCol cols="12" class="px-2 pt-0">
          <VAlert v-if="hasNonOrbitControllers" type="warning" variant="tonal" density="compact">
            {{ $t('external_canisters.non_orbit_controllers_warning') }}
          </VAlert>
        </VCol>
        <VCol cols="12" class="py-0">
          <p class="text-h6 mb-2">
            {{ $t('external_canisters.native_settings.controllers') }}
            <VChip rounded size="small" variant="tonal">
              {{ model.controllers?.length || 0 }}
            </VChip>
          </p>
          <VDivider />
        </VCol>
        <VCol cols="12" class="d-flex flex-column pt-0">
          <VTable v-if="model.controllers && model.controllers.length" density="compact" hover>
            <tbody>
              <tr v-for="(controller, idx) in model.controllers" :key="idx">
                <td class="pa-2 w-100">
                  <span v-if="station.canisterId === controller.toText()">
                    ({{ $t('app.name') }})
                  </span>
                  <span v-else-if="model.canisterId?.toText() === controller.toText()">
                    ({{ $t('external_canisters.self_controller') }})
                  </span>
                  <div :class="{ 'd-inline': !app.isMobile }">
                    <TextOverflow
                      :text="controller.toText()"
                      :max-length="app.isMobile ? 24 : 64"
                    />

                    <VBtn
                      size="x-small"
                      variant="text"
                      :icon="mdiContentCopy"
                      @click="
                        copyToClipboard({
                          textToCopy: controller.toText(),
                          sendNotification: true,
                        })
                      "
                    />
                  </div>
                </td>
                <td v-if="!props.readonly" class="px-1 min-height-100 text-right">
                  <VBtn
                    size="small"
                    variant="text"
                    density="comfortable"
                    :icon="mdiTrashCanOutline"
                    @click="model.controllers?.splice(idx, 1)"
                  />
                </td>
              </tr>
            </tbody>
          </VTable>
          <VTextField
            v-if="!props.readonly && !hasMaxControllers"
            v-model="newController"
            :label="$t('external_canisters.add_controller')"
            name="new_controller"
            density="comfortable"
            class="mt-2"
            :rules="[validPrincipalRule, uniqueRule(existingControllers)]"
            @keydown.enter.stop.prevent="canAddController ? addController() : undefined"
          >
            <template #append>
              <VBtn
                :disabled="!canAddController"
                color="primary"
                variant="flat"
                size="small"
                :icon="mdiPlus"
                @click="addController"
              />
            </template>
          </VTextField>
          <span v-if="props.readonly && !model.controllers?.length" class="mt-2">
            {{ $t('external_canisters.no_controllers') }}
          </span>
        </VCol>
        <VCol cols="12" class="mb-2 pt-0">
          <p class="text-h6 mb-2">{{ $t('terms.advanced') }}</p>
          <VDivider />
        </VCol>
        <VCol cols="12" class="d-flex flex-column ga-2">
          <VSlider
            v-model="model.compute_allocation"
            :label="$t('external_canisters.native_settings.compute_allocation')"
            name="compute_allocation"
            :readonly="props.readonly"
            density="comfortable"
            :min="0"
            :max="100"
            :step="1"
            thumb-label="always"
            :hint="$t('external_canisters.native_settings.compute_allocation_hint')"
          />
          <VTextField
            v-model="model.memory_allocation"
            :label="$t('external_canisters.native_settings.memory_allocation')"
            name="memory_allocation"
            :readonly="props.readonly"
            density="comfortable"
            type="number"
            :rules="[
              requiredRule,
              numberRangeRule({
                min: 0,
                max: Number.MAX_SAFE_INTEGER,
                decimals: 0,
              }),
            ]"
            :hint="$t('external_canisters.native_settings.memory_allocation_hint')"
          />
          <VTextField
            v-model="model.reserved_cycles_limit"
            :label="$t('external_canisters.native_settings.reserved_cycles_limit')"
            name="reserved_cycles_limit"
            :readonly="props.readonly"
            density="comfortable"
            type="number"
            :rules="[
              requiredRule,
              numberRangeRule({
                min: 0,
                max: Number.MAX_SAFE_INTEGER,
                decimals: 0,
              }),
            ]"
            :hint="$t('external_canisters.native_settings.reserved_cycles_limit_hint')"
          />
          <VTextField
            v-model="model.wasm_memory_limit"
            :label="$t('external_canisters.native_settings.wasm_memory_limit')"
            name="wasm_memory_limit"
            :readonly="props.readonly"
            density="comfortable"
            type="number"
            :rules="[
              requiredRule,
              numberRangeRule({
                min: 0,
                max: Number.MAX_SAFE_INTEGER,
                decimals: 0,
              }),
            ]"
            :hint="$t('external_canisters.native_settings.wasm_memory_limit_hint')"
          />
          <VTextField
            v-model="model.freezing_threshold"
            :label="$t('external_canisters.native_settings.freezing_threshold')"
            name="freezing_threshold"
            :readonly="props.readonly"
            type="number"
            density="comfortable"
            :rules="[
              requiredRule,
              numberRangeRule({
                min: 0,
                max: Number.MAX_SAFE_INTEGER,
                decimals: 0,
              }),
            ]"
            :hint="$t('external_canisters.native_settings.freezing_threshold_hint')"
          />
          <VSelect
            v-model="logVisibilitySelected"
            :items="logVisibilityItems"
            :label="$t('external_canisters.native_settings.log_visibility')"
            :hint="$t('external_canisters.native_settings.log_visibility_hint')"
            name="log_visibility"
            :readonly="props.readonly"
            density="comfortable"
            item-value="value"
            item-title="text"
            :disabled="!!model.log_visibility && variantIs(model.log_visibility, 'allowed_viewers')"
          />
        </VCol>
      </VRow>
    </VContainer>
  </VForm>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiContentCopy, mdiPlus, mdiTrashCanOutline } from '@mdi/js';
import { computed, onMounted, ref, watch } from 'vue';
import {
  VAlert,
  VBtn,
  VChip,
  VCol,
  VContainer,
  VDivider,
  VForm,
  VRow,
  VSlider,
  VTable,
  VTextField,
} from 'vuetify/components';
import logger from '~/core/logger.core';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import { SelectItem, VFormValidation } from '~/types/helper.types';
import { copyToClipboard } from '~/utils/app.utils';
import { numberRangeRule, requiredRule, uniqueRule, validPrincipalRule } from '~/utils/form.utils';
import TextOverflow from '../TextOverflow.vue';
import CanisterIdField from '../inputs/CanisterIdField.vue';
import { CanisterIcSettingsModel } from './external-canisters.types';
import { LogVisibility } from '~/generated/station/station.did';
import { useI18n } from 'vue-i18n';
import { variantIs } from '~/utils/helper.utils.ts';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterIcSettingsModel;
    triggerSubmit?: boolean;
    readonly?: boolean;
    display?: {
      canisterId: boolean;
    };
  }>(),
  {
    readonly: false,
    triggerSubmit: false,
    display: () => ({
      canisterId: true,
    }),
  },
);
const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterIcSettingsModel): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'edited', payload: boolean): void;
  (event: 'submit', payload: CanisterIcSettingsModel): void;
}>();

const form = ref<VFormValidation>();
const valid = ref(true);
const i18n = useI18n();
const fieldsWithErrors = ref<string[]>([]);
const newController = ref<string>('');
const app = useAppStore();
const station = useStationStore();
const initialModel = ref<string>('');

const takeModelSnapshot = (model: CanisterIcSettingsModel): string => {
  const snapshot: Map<string, string | undefined> = new Map();
  snapshot.set('compute_allocation', model.compute_allocation?.toString());
  snapshot.set('memory_allocation', model.memory_allocation?.toString());
  snapshot.set('reserved_cycles_limit', model.reserved_cycles_limit?.toString());
  snapshot.set('freezing_threshold', model.freezing_threshold?.toString());
  snapshot.set('canister_id', model.canisterId?.toText());
  snapshot.set('wasm_memory_limit', model.wasm_memory_limit?.toString());
  snapshot.set('log_visibility', JSON.stringify(model.log_visibility));

  const controllers = model.controllers?.map(c => c.toText()).sort() ?? [];
  controllers.forEach((controller, idx) => snapshot.set(`controllers[${idx}]`, controller));

  return JSON.stringify(Object.fromEntries(snapshot));
};

onMounted(() => {
  initialModel.value = takeModelSnapshot(props.modelValue);
});

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

watch(
  () => model.value,
  newModelValue => {
    emit('edited', takeModelSnapshot(newModelValue) !== initialModel.value);
  },
  { deep: true },
);

const triggerSubmit = computed({
  get: () => props.triggerSubmit,
  set: value => emit('update:triggerSubmit', value),
});

watch(valid, newValid => emit('valid', newValid), { immediate: true });

watch(
  () => form.value?.errors,
  _ => {
    valid.value = form.value?.isValid ?? false;
    fieldsWithErrors.value = form.value?.errors.map(error => error.id) ?? [];
  },
  { deep: true },
);

watch(triggerSubmit, shouldTrigger => {
  if (shouldTrigger) {
    emit('update:triggerSubmit', false);

    submit();
  }
});

const newControllerPrincipal = computed(() => {
  try {
    return Principal.fromText(newController.value);
  } catch {
    return undefined;
  }
});

const hasMaxControllers = computed(
  () => model.value.controllers && model.value.controllers.length >= 10,
);

const canAddController = computed(
  () =>
    form.value?.errors.find(error => error.id === 'new_controller') === undefined &&
    newControllerPrincipal.value &&
    !hasMaxControllers.value,
);

const safeControllers = computed(() => {
  const set = new Set<string>();

  if (model.value.canisterId) {
    set.add(model.value.canisterId.toText());
  }

  if (station.canisterId) {
    set.add(station.canisterId);
  }

  return set;
});

const hasNonOrbitControllers = computed(
  () =>
    model.value.controllers &&
    model.value.controllers.some(c => !safeControllers.value.has(c.toText())),
);

const addController = () => {
  if (!newControllerPrincipal.value) {
    logger.warn('Unexpected code path, newControllerPrincipal should be defined');
    return;
  }

  const currentControllers = model.value.controllers || [];

  model.value.controllers = [...currentControllers, newControllerPrincipal.value];

  newController.value = '';
};

// temporary parsing of the log_visibility field to support the new variant type
const logVisibilitySelected = computed({
  get: () => {
    if (model.value.log_visibility && variantIs(model.value.log_visibility, 'allowed_viewers')) {
      return i18n.t('terms.allowed_viewers') + ' (change via dfx)';
    }

    return model.value.log_visibility;
  },
  set: (newValue: LogVisibility) => {
    if (newValue) {
      model.value.log_visibility = newValue;
    }
  },
});

const logVisibilityItems = computed<SelectItem<LogVisibility>[]>(() => [
  { value: { controllers: null }, text: i18n.t('terms.controllers') },
  { value: { public: null }, text: i18n.t('terms.public') },
]);

const existingControllers = computed(() => model.value.controllers?.map(c => c.toText()) || []);

const revalidate = async (): Promise<boolean> => {
  const { valid: isValid, errors } = form.value
    ? await form.value.validate()
    : { valid: false, errors: [] };

  valid.value = isValid;
  fieldsWithErrors.value = errors.map(error => error.id);

  return isValid;
};

const submit = async (): Promise<void> => {
  const isValid = await revalidate();

  if (isValid) {
    emit('submit', model.value);
  }
};
</script>
