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
      <div class="mt-4 mb-2 text-body-2">{{ $t('terms.criteria') }}</div>
      <CriteriaBuilder
        v-model="model.criteria"
        :specifier="model.specifier"
        :disabled="isViewMode"
        @remove="model.criteria = undefined"
      />

      <span v-if="!model.criteria && isViewMode">
        {{ $t('terms.none') }}
      </span>
    </template>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, toRefs } from 'vue';
import CriteriaBuilder from '~/components/proposal-policies/criteria/CriteriaBuilder.vue';
import SpecifierSelector from '~/components/proposal-policies/specifier/SpecifierSelector.vue';
import { ProposalPolicy } from '~/generated/wallet/wallet.did';
import { VFormValidation } from '~/types/helper.types';

export type ProposalPolicyFormProps = {
  modelValue: Partial<ProposalPolicy>;
  valid?: boolean;
  mode?: 'view' | 'edit';
  display?: {
    id?: boolean;
    specifier?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const input = withDefaults(defineProps<ProposalPolicyFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
    specifier: true,
  }),
  mode: 'edit',
});
const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalPolicyFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: ProposalPolicyFormProps['modelValue']): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const isViewMode = computed(() => props.mode.value === 'view');

const onChangedVariant = (): void => {
  model.value.criteria = undefined;
};

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
