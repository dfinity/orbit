<template>
  {{ model }}
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { ProposalPolicyCriteria } from '~/generated/wallet/wallet.did';

const input = withDefaults(
  defineProps<{
    modelValue?: ProposalPolicyCriteria;
  }>(),
  {
    modelValue: () => ({
      And: [
        {
          AutoAdopted: null,
        },
      ],
    }),
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalPolicyCriteria): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});
</script>
