<template>
  <VDivider />
  <VCard variant="text" density="comfortable" v-bind="$attrs">
    <VCardTitle class="px-2">
      {{ $t('proposal_policies.criteria.or') }}
      <VBtn
        :icon="mdiTrashCanOutline"
        variant="flat"
        size="small"
        dark
        density="compact"
        @click="emit('remove')"
      />
    </VCardTitle>
    <VCardText class="d-flex flex-column ga-2 px-2">
      <CriteriaBuilder
        v-for="(_, idx) of model"
        :key="idx"
        v-model="model[idx]"
        :specifier="props.specifier.value"
        @remove="removeEntry(idx)"
      />
    </VCardText>
    <VCardActions class="px-2">
      <VSelect
        v-model="selectedAddCriteria"
        :label="$t('proposal_policies.add_criteria_label')"
        :items="availableCriterias"
        item-value="value"
        item-title="text"
        density="comfortable"
        @update:model-value="onAddCriteria"
      />
    </VCardActions>
  </VCard>
  <VDivider />
</template>

<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { ProposalPolicyCriteria, ProposalSpecifier } from '~/generated/wallet/wallet.did';
import CriteriaBuilder from './CriteriaBuilder.vue';
import { useProposalSpecifierCriterias } from '~/composables/proposal-policies.composable';
import { ProposalCriteriaEnum } from '~/types/wallet.types';
import { unreachable } from '~/utils/helper.utils';
import { ref } from 'vue';
import { mdiTrashCanOutline } from '@mdi/js';

const input = withDefaults(
  defineProps<{
    modelValue?: ProposalPolicyCriteria[];
    specifier: ProposalSpecifier;
  }>(),
  {
    modelValue: () => [],
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ProposalPolicyCriteria[]): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const removeEntry = (idx: number): void => {
  model.value.splice(idx, 1);
};

const selectedAddCriteria = ref<ProposalCriteriaEnum | null>(null);

const onAddCriteria = (value: ProposalCriteriaEnum | null): void => {
  if (value === null) {
    return;
  }

  selectedAddCriteria.value = null;
  switch (value) {
    case ProposalCriteriaEnum.And:
      model.value.push({ And: [] });
      break;
    case ProposalCriteriaEnum.Or:
      model.value.push({ Or: [] });
      break;
    case ProposalCriteriaEnum.Not:
      model.value.push({ Not: { And: [] } });
      break;
    case ProposalCriteriaEnum.AutoAdopted:
      model.value.push({ AutoAdopted: null });
      break;
    case ProposalCriteriaEnum.MinimumVotes:
      model.value.push({ MinimumVotes: [{ Any: null }, 0] });
      break;
    case ProposalCriteriaEnum.ApprovalThreshold:
      model.value.push({ ApprovalThreshold: [{ Any: null }, 0] });
      break;
    case ProposalCriteriaEnum.HasAddressBookMetadata:
      model.value.push({ HasAddressBookMetadata: { key: '', value: '' } });
      break;
    default:
      unreachable(value);
  }
};

const availableCriterias = useProposalSpecifierCriterias(props.specifier.value);
</script>
