<template>
  <div class="d-flex flex-column ga-2">
    <div>
      {{ $t('proposal_policies.criteria.not') }}
      <VBtn
        :icon="mdiTrashCanOutline"
        variant="flat"
        size="small"
        color="transparent"
        density="compact"
        class="ml-1"
        @click="emit('remove')"
      />
    </div>
    <CriteriaBuilder v-model="model" :specifier="props.specifier.value" @remove="emit('remove')" />
  </div>
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { ProposalPolicyCriteria, ProposalSpecifier } from '~/generated/wallet/wallet.did';
import CriteriaBuilder from './CriteriaBuilder.vue';

const input = withDefaults(
  defineProps<{
    modelValue: ProposalPolicyCriteria;
    specifier: ProposalSpecifier;
  }>(),
  {},
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalPolicyCriteria): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});
</script>
