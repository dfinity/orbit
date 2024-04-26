<template>
  <VSelect
    v-model="selectedAddCriteria"
    :label="$t('proposal_policies.add_criteria_label')"
    :items="availableCriterias"
    item-value="value"
    item-title="text"
    hide-details
    density="comfortable"
    @update:model-value="onAddCriteria"
  />
</template>

<script setup lang="ts">
import { ref, toRefs } from 'vue';
import { useProposalSpecifierCriterias } from '~/composables/proposal-policies.composable';
import { ProposalPolicyCriteria, ProposalSpecifier } from '~/generated/station/station.did';
import { ProposalCriteriaEnum } from '~/types/station.types';
import { unreachable } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    specifier: ProposalSpecifier;
  }>(),
  {},
);

const props = toRefs(input);

const selectedAddCriteria = ref<ProposalCriteriaEnum | null>(null);
const availableCriterias = useProposalSpecifierCriterias(props.specifier);

const emit = defineEmits<{
  (event: 'add', payload: ProposalPolicyCriteria): void;
}>();

const onAddCriteria = (value: ProposalCriteriaEnum | null): void => {
  if (value === null) {
    return;
  }

  selectedAddCriteria.value = null;
  switch (value) {
    case ProposalCriteriaEnum.And:
      emit('add', { And: [] });
      break;
    case ProposalCriteriaEnum.Or:
      emit('add', { Or: [] });
      break;
    case ProposalCriteriaEnum.Not:
      emit('add', { Not: {} as ProposalPolicyCriteria });
      break;
    case ProposalCriteriaEnum.AutoAdopted:
      emit('add', { AutoAdopted: null });
      break;
    case ProposalCriteriaEnum.HasAddressInAddressBook:
      emit('add', { HasAddressInAddressBook: null });
      break;
    case ProposalCriteriaEnum.MinimumVotes:
      emit('add', {
        MinimumVotes: {
          voters: { Any: null },
          minimum: 1,
        },
      });
      break;
    case ProposalCriteriaEnum.ApprovalThreshold:
      emit('add', {
        ApprovalThreshold: {
          voters: { Any: null },
          threshold: 100,
        },
      });
      break;
    case ProposalCriteriaEnum.HasAddressBookMetadata:
      emit('add', { HasAddressBookMetadata: { key: '', value: '' } });
      break;
    default:
      unreachable(value);
  }
};
</script>
