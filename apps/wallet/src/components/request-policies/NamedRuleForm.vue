<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && props.display.value.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="comfortable"
      disabled
      :prepend-icon="mdiIdentifier"
    />

    <DiffView :before-value="props.currentNamedRule.value?.name" :after-value="model.name">
      <template #default="{ value, mode }">
        <VTextField
          :name="mode === 'before' ? 'name-before' : 'name'"
          :model-value="value"
          @update:model-value="val => mode === 'after' && (model.name = val)"
          :label="$t('terms.name')"
          density="comfortable"
          :rules="mode === 'before' ? [] : [requiredRule]"
          :prepend-icon="mdiFileDocumentCheckOutline"
          :disabled="isViewMode || mode === 'before'"
        />
      </template>
    </DiffView>

    <DiffView
      :before-value="descriptionBefore"
      :after-value="description"
      :has-before="!!props.currentNamedRule.value"
      :compare-values="compareTruthy"
    >
      <template #default="{ value, mode }">
        <VTextField
          :name="mode === 'before' ? 'description-before' : 'description'"
          :model-value="value"
          @update:model-value="val => mode === 'after' && (description = val)"
          :label="$t('terms.description')"
          density="comfortable"
          :prepend-icon="mdiInformationBoxOutline"
          :disabled="isViewMode || mode === 'before'"
        />
      </template>
    </DiffView>

    <div class="mt-4 mb-2 text-body-1">{{ $t('terms.rule') }}</div>
    <DiffView :before-value="props.currentNamedRule.value?.rule" :after-value="model.rule">
      <template #default="{ value, mode }">
        <RuleBuilder
          :model-value="value"
          @update:model-value="val => mode === 'after' && (model.rule = val)"
          :disabled="isViewMode || mode === 'before'"
          @remove="mode === 'after' && (model.rule = undefined)"
        />
      </template>
    </DiffView>

    <span v-if="!model.rule && isViewMode">
      {{ $t('terms.none') }}
    </span>
  </VForm>
</template>

<script lang="ts" setup>
import { mdiFileDocumentCheckOutline, mdiIdentifier, mdiInformationBoxOutline } from '@mdi/js';
import { computed, ref, toRefs, watch } from 'vue';
import RuleBuilder from '~/components/request-policies/rule/RuleBuilder.vue';
import { NamedRule } from '~/generated/station/station.did';
import { VFormValidation } from '~/types/helper.types';
import { compareTruthy, requiredRule } from '~/utils/form.utils';
import DiffView from '~/components/requests/DiffView.vue';

export type RequestPolicyFormProps = {
  modelValue: Partial<NamedRule>;
  valid?: boolean;
  mode?: 'view' | 'edit';
  display?: {
    id?: boolean;
  };
  currentNamedRule?: NamedRule;
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<RequestPolicyFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
  }),
  mode: 'edit',
  currentNamedRule: undefined,
});
const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: RequestPolicyFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: RequestPolicyFormProps['modelValue']): void;
}>();

const model = computed(() => props.modelValue.value);

watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

const description = computed({
  get: () => (model.value.description ? model.value.description[0] : ''),
  set: value => {
    model.value.description = value ? [value] : undefined;
  },
});

const descriptionBefore = computed(() => props.currentNamedRule.value?.description[0]);

const isViewMode = computed(() => props.mode.value === 'view');

const isFormValid = computed(() => (form.value ? form.value.isValid : false));
watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
