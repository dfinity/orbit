<template>
  <VRow>
    <VCol v-if="model.id" cols="12" class="pt-0 pb-0">
      <VTextField
        v-model="model.id"
        name="id"
        :prepend-icon="mdiIdentifier"
        :label="$t('terms.id')"
        variant="plain"
        density="comfortable"
        hide-details
        disabled
      />
    </VCol>
    <VCol cols="12" class="pt-2 pb-0">
      <VTextField
        v-model="model.name"
        name="name"
        :label="$t('terms.name')"
        variant="filled"
        density="comfortable"
        :rules="[requiredRule]"
        :disabled="isViewMode"
        :prepend-icon="mdiDatabase"
      />
    </VCol>
    <VCol v-if="isCreationMode" cols="12" class="pt-0 pb-0">
      <VRadioGroup v-model="creationModeKind" name="creation_kind" inline class="ml-8">
        <VRadio :label="$t('external_canisters.create_new')" value="new" class="mr-4" />
        <VRadio :label="$t('external_canisters.use_existing')" value="existing" />
      </VRadioGroup>
      <div v-if="creationModeKind === 'existing'">
        <VTextField
          v-model="model.canisterId"
          name="canister_id"
          :label="$t('terms.canister_id')"
          variant="filled"
          density="comfortable"
          :rules="[requiredRule, validCanisterId]"
          :prepend-icon="mdiIdentifier"
        />
        <VTextField
          v-model="model.maybe_with_initial_cycles"
          name="initial_cycles"
          :label="$t('external_canisters.initial_cycles')"
          variant="filled"
          density="comfortable"
          type="number"
          :rules="[
            intNumberRangeRule($t('external_canisters.initial_cycles'), 1, Number.MAX_SAFE_INTEGER),
          ]"
          :prepend-icon="mdiDatabaseRefresh"
        />
      </div>
    </VCol>
    <VCol cols="12" class="pt-2 pb-0">
      <VTextarea
        v-model="model.description"
        name="description"
        :label="$t('terms.description')"
        variant="filled"
        density="comfortable"
        :disabled="isViewMode"
        :prepend-icon="mdiText"
      />
    </VCol>
    <VCol cols="12" class="pt-2 pb-0 d-flex flex-row flex-nowrap ga-4">
      <div><VIcon :icon="mdiLabel" class="text-medium-emphasis" /></div>
      <div class="d-flex flex-column flex-grow-1 ga-2">
        <span class="text-medium-emphasis">{{ $t('terms.labels') }}</span>
        <div class="d-flex ga-2 flex-row flex-wrap">
          <VChip v-for="(label, idx) in model.labels" :key="idx" size="small">
            {{ label }}
            <template #append>
              <VBtn
                v-if="!isViewMode"
                :icon="mdiCloseCircle"
                size="small"
                variant="text"
                density="compact"
                class="ml-2"
                @click.stop="
                  () => {
                    if (model.labels) {
                      model.labels = model.labels.filter((_, i) => i !== idx);
                    }
                  }
                "
              />
            </template>
          </VChip>
        </div>
        <div>
          <VTextField
            v-if="!isViewMode"
            v-model="newLabel"
            name="new_label"
            :label="$t('external_canisters.add_new_label')"
            variant="filled"
            density="compact"
          >
            <template #append>
              <VBtn
                :disabled="!newLabel?.length"
                color="primary"
                :icon="mdiPlus"
                variant="tonal"
                size="small"
                @click.stop="
                  () => {
                    if (!model.labels) {
                      model.labels = [];
                    }

                    if (
                      newLabel?.length &&
                      model.labels &&
                      model.labels.findIndex(
                        curr => curr.toLowerCase() === newLabel.toLowerCase(),
                      ) === -1
                    ) {
                      model.labels = [...model.labels, newLabel];
                    }

                    newLabel = '';
                  }
                "
              />
            </template>
          </VTextField>
        </div>
      </div>
    </VCol>
    <VCol cols="12" class="pt-2 pb-0 d-flex">
      <div><VIcon :icon="mdiCogs" class="text-medium-emphasis" /></div>
      <div>
        <VRadioGroup v-model="model.state" name="state" inline :label="$t('terms.status')">
          <VRadio
            v-for="(state, idx) in availableStates"
            :key="idx"
            :label="state.text"
            :value="state.key"
            :readonly="isViewMode"
            class="mr-4"
          />
        </VRadioGroup>
      </div>
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import {
  mdiCloseCircle,
  mdiCogs,
  mdiDatabase,
  mdiDatabaseRefresh,
  mdiIdentifier,
  mdiLabel,
  mdiPlus,
  mdiText,
} from '@mdi/js';
import { computed, ref, watch } from 'vue';
import {
  VBtn,
  VChip,
  VCol,
  VIcon,
  VRadio,
  VRadioGroup,
  VRow,
  VTextField,
  VTextarea,
} from 'vuetify/components';
import { useExternalCanistersStates } from '~/composables/external-canisters.composable';
import { intNumberRangeRule, requiredRule, validCanisterId } from '~/utils/form.utils';
import { CanisterConfigurationModel } from './wizard.types';

const props = withDefaults(
  defineProps<{
    modelValue: Partial<CanisterConfigurationModel>;
    mode: 'view' | 'edit';
  }>(),
  {
    mode: 'edit',
    triggerSubmit: false,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<CanisterConfigurationModel>): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const isViewMode = computed(() => props.mode === 'view');
const isCreationMode = computed(() => !model.value.id);
const availableStates = useExternalCanistersStates();
const newLabel = ref<string>('');
const creationModeKind = ref<'existing' | 'new'>(model.value.canisterId ? 'existing' : 'new');

watch(
  () => creationModeKind.value,
  () => {
    if (creationModeKind.value === 'new') {
      model.value.canisterId = undefined;
      model.value.maybe_with_initial_cycles = undefined;
    }
  },
  { immediate: true },
);
</script>
