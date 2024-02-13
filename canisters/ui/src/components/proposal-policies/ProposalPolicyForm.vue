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

    <SpecifierSelector
      v-model="model.specifier"
      :disabled="props.disabled.value"
      @changed-variant="onChangedVariant"
    />

    <template v-if="model.specifier">
      <div class="mt-4 mb-2 text-body-2">{{ $t('terms.criteria') }}</div>
      <CriteriaBuilder
        v-model="model.criteria"
        :specifier="model.specifier"
        :disabled="props.disabled.value"
        @remove="model.criteria = undefined"
      />

      <span v-if="!model.criteria && props.disabled.value">
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
  disabled?: boolean;
  display?: {
    id?: boolean;
    specifier?: boolean;
  };
};

const form = ref<VFormValidation | null>(null);

const p = withDefaults(defineProps<ProposalPolicyFormProps>(), {
  valid: true,
  display: () => ({
    id: true,
    specifier: true,
  }),
  disabled: false,
});
const props = toRefs(p);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalPolicyFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: ProposalPolicyFormProps['modelValue']): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const onChangedVariant = (): void => {
  model.value.criteria = { AutoAdopted: null };
};

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>
