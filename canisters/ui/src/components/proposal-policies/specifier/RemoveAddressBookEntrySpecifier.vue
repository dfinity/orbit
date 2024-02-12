<template>
  {{ model }}
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { CommonSpecifier, ProposalSpecifier } from '~/generated/wallet/wallet.did';

const input = withDefaults(
  defineProps<{
    modelValue?: ProposalSpecifier & { RemoveAddressBookEntry: CommonSpecifier };
  }>(),
  {
    modelValue: () => ({ RemoveAddressBookEntry: { Any: null } }),
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalSpecifier): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});
</script>
