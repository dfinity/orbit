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

    <VTextField
      v-model="model.name"
      name="name"
      :label="$t('terms.name')"
      density="comfortable"
      :rules="[requiredRule]"
      :prepend-icon="mdiFileDocumentCheckOutline"
      :disabled="isViewMode"
    />

    <VTextField
      v-model="description"
      name="description"
      :label="$t('terms.description')"
      density="comfortable"
      :prepend-icon="mdiInformationBoxOutline"
      :disabled="isViewMode"
    />

    <div class="mt-4 mb-2 text-body-1">{{ $t('terms.rule') }}</div>
    <RuleBuilder v-model="model.rule" :disabled="isViewMode" @remove="model.rule = undefined" />

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
import { requiredRule } from '~/utils/form.utils';

export type RequestPolicyFormProps = {
  modelValue: Partial<NamedRule>;
  valid?: boolean;
  mode?: 'view' | 'edit';
  display?: {
    id?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<RequestPolicyFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
  }),
  mode: 'edit',
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
