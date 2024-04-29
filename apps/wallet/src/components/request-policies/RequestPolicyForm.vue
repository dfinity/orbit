<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id && props.display.value.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      disabled
    />

    <SpecifierSelector
      v-model="model.specifier"
      :disabled="isViewMode"
      @changed-variant="onChangedVariant"
    />

    <template v-if="model.specifier">
      <div class="mt-4 mb-2 text-body-2">{{ $t('terms.rule') }}</div>
      <RuleBuilder
        v-model="model.rule"
        :specifier="model.specifier"
        :disabled="isViewMode"
        @remove="model.rule = undefined"
      />

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
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<RequestPolicyFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
    specifier: true,
  }),
  mode: 'edit',
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
