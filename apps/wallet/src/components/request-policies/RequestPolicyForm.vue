<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && props.display.value.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />

    <DiffView
      :before-value="props.currentRequestPolicy.value?.specifier"
      :after-value="model.specifier"
    >
      <template #default="{ value, diffMode }">
        <SpecifierSelector
          :model-value="value"
          :disabled="isViewMode || diffMode === 'before'"
          @update:model-value="val => diffMode === 'after' && (model.specifier = val)"
          @changed-variant="() => diffMode === 'after' && onChangedVariant()"
        />
      </template>
    </DiffView>

    <template v-if="model.specifier">
      <div class="mt-4 mb-2 text-body-2">{{ $t('terms.rule') }}</div>
      <DiffView :before-value="props.currentRequestPolicy.value?.rule" :after-value="model.rule">
        <template #default="{ value, diffMode }">
          <RuleBuilder
            :model-value="value"
            :specifier="value ? props.modelValue.value.specifier! : undefined"
            :disabled="isViewMode || diffMode === 'before'"
            @update:model-value="val => diffMode === 'after' && (model.rule = val)"
            @remove="diffMode === 'after' && (model.rule = undefined)"
          />
        </template>
      </DiffView>

      <span v-if="!model.rule && isViewMode">
        {{ $t('terms.none') }}
      </span>
    </template>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, toRefs } from 'vue';
import RuleBuilder from '~/components/request-policies/rule/RuleBuilder.vue';
import SpecifierSelector from '~/components/request-policies/specifier/SpecifierSelector.vue';
import DiffView from '~/components/requests/DiffView.vue';
import { RequestPolicy } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';

export type RequestPolicyFormProps = {
  modelValue: Partial<RequestPolicy>;
  valid?: boolean;
  mode?: 'view' | 'edit';
  display?: {
    id?: boolean;
    specifier?: boolean;
  };
  currentRequestPolicy?: RequestPolicy;
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<RequestPolicyFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
    specifier: true,
  }),
  mode: 'edit',
  currentRequestPolicy: undefined,
});
const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: RequestPolicyFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: RequestPolicyFormProps['modelValue']): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const isViewMode = computed(() => props.mode.value === 'view');

const onChangedVariant = (): void => {
  model.value.rule = undefined;
};

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
