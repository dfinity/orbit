<template>
  <VDivider />
  <VCard variant="text" density="comfortable" v-bind="$attrs">
    <VCardTitle class="px-2">
      {{ $t('proposal_policies.criteria.or') }}
      <VBtn
        v-if="!props.disabled.value"
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
        :disabled="props.disabled.value"
        @remove="removeEntry(idx)"
      />
    </VCardText>
    <VCardActions class="px-2">
      <AddCriteriaSelect
        v-if="!props.disabled.value"
        :specifier="props.specifier.value"
        @add="model.push($event)"
      />
    </VCardActions>
  </VCard>
  <VDivider />
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { ProposalPolicyCriteria, ProposalSpecifier } from '~/generated/station/station.did';
import AddCriteriaSelect from './AddCriteriaSelect.vue';
import CriteriaBuilder from './CriteriaBuilder.vue';

const input = withDefaults(
  defineProps<{
    modelValue?: ProposalPolicyCriteria[];
    specifier: ProposalSpecifier;
    disabled?: boolean;
  }>(),
  {
    modelValue: () => [],
    disabled: false,
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
</script>
