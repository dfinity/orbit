<template>
  <AddCriteriaSelect
    v-if="model === null"
    :specifier="props.specifier.value"
    @add="model = $event"
  />
  <template v-else>
    <AndCriteria
      v-if="variantIs(model, 'And')"
      v-model="model.And"
      :specifier="props.specifier.value"
      @remove="emit('remove')"
    />
    <AutoAdoptedCriteria v-else-if="variantIs(model, 'AutoAdopted')" @remove="emit('remove')" />
    <MinimumVotesCriteria
      v-else-if="variantIs(model, 'MinimumVotes')"
      v-model="model.MinimumVotes"
      @remove="emit('remove')"
    />
    <ApprovalThresholdCriteria
      v-else-if="variantIs(model, 'ApprovalThreshold')"
      v-model="model.ApprovalThreshold"
      @remove="emit('remove')"
    />
    <NotCriteria
      v-else-if="variantIs(model, 'Not')"
      v-model="model.Not"
      :specifier="props.specifier.value"
      @remove="emit('remove')"
    />
    <OrCriteria
      v-else-if="variantIs(model, 'Or')"
      v-model="model.Or"
      :specifier="props.specifier.value"
      @remove="emit('remove')"
    />
    <HasAddressBookMetadataCriteria
      v-else-if="variantIs(model, 'HasAddressBookMetadata')"
      v-model="model.HasAddressBookMetadata"
      @remove="emit('remove')"
    />
  </template>
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { ProposalPolicyCriteria, ProposalSpecifier } from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';
import AddCriteriaSelect from './AddCriteriaSelect.vue';
import AndCriteria from './AndCriteria.vue';
import ApprovalThresholdCriteria from './ApprovalThresholdCriteria.vue';
import AutoAdoptedCriteria from './AutoAdoptedCriteria.vue';
import MinimumVotesCriteria from './MinimumVotesCriteria.vue';
import NotCriteria from './NotCriteria.vue';
import OrCriteria from './OrCriteria.vue';
import HasAddressBookMetadataCriteria from '~/components/proposal-policies/criteria/HasAddressBookMetadataCriteria.vue';

const input = withDefaults(
  defineProps<{
    modelValue?: ProposalPolicyCriteria | null;
    specifier: ProposalSpecifier;
  }>(),
  {
    modelValue: null,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalPolicyCriteria | null): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});
</script>
