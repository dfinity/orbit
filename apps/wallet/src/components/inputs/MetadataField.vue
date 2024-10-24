<template>
  <div class="d-flex flex-column">
    <label v-if="props.label.value" class="text-body-1 font-weight-bold">
      {{ props.label.value }}
    </label>
    <VTable :density="props.density.value">
      <thead>
        <tr>
          <th class="w-50">{{ $t('terms.key') }}</th>
          <th class="w-50">{{ $t('terms.value') }}</th>
          <th v-if="!props.disabled.value && !props.readonly.value">&nbsp;</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(item, idx) in model" :key="idx">
          <template v-if="!hideKeys.includes(item.key)">
            <td class="px-0 py-2">
              <VTextField
                v-model="item.key"
                :readonly="props.readonly.value"
                :disabled="props.disabled.value"
                variant="filled"
                density="compact"
                :rules="[requiredRule]"
                hide-details
              />
            </td>
            <td class="px-1">
              <VTextField
                v-model="item.value"
                :readonly="props.readonly.value"
                :disabled="props.disabled.value"
                variant="filled"
                density="compact"
                hide-details
              />
            </td>
            <td v-if="!props.disabled.value && !props.readonly.value">
              <div class="d-flex align-center justify-end">
                <VBtn
                  size="small"
                  variant="text"
                  :disabled="props.disabled.value"
                  :icon="mdiTrashCanOutline"
                  @click="remove(idx)"
                />
              </div>
            </td>
          </template>
        </tr>
        <tr>
          <td class="px-0" colspan="3">
            <VBtn
              size="small"
              class="my-2 w-100"
              color="background"
              variant="flat"
              :disabled="props.disabled.value"
              :prepend-icon="mdiPlus"
              @click="model.push({ key: '', value: '' })"
            >
              {{ $t('terms.add') }}
            </VBtn>
          </td>
        </tr>
      </tbody>
    </VTable>
  </div>
</template>

<script setup lang="ts">
import { mdiPlus, mdiTrashCanOutline } from '@mdi/js';
import { computed, toRefs, watch } from 'vue';
import { VBtn, VTable, VTextField } from 'vuetify/components';
import { MetadataItem } from '~/types/station.types';
import { requiredRule } from '~/utils/form.utils';

const input = withDefaults(
  defineProps<{
    modelValue?: MetadataItem[];
    label?: string;
    density?: 'comfortable' | 'compact';
    readonly?: boolean;
    disabled?: boolean;
    hideKeys?: string[];
  }>(),
  {
    modelValue: () => [],
    label: undefined,
    density: 'comfortable',
    readonly: false,
    disabled: false,
    hideKeys: () => [],
  },
);

const props = toRefs(input);

const model = computed(() => props.modelValue.value);
watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

const emit = defineEmits<{
  (event: 'update:modelValue', payload: MetadataItem[]): void;
}>();

const remove = (idx: number) => {
  model.value.splice(idx, 1);
};
</script>
