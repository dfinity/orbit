<template>
  <div class="d-flex flex-column ga-2">
    <div>
      {{ $t('proposal_policies.criteria.approvalthreshold') }}
      <VBtn
        v-if="!props.disabled.value"
        :icon="mdiTrashCanOutline"
        variant="flat"
        size="small"
        color="transparent"
        density="compact"
        class="ml-1"
        @click="emit('remove')"
      />
    </div>
    <div class="d-flex flex-column flex-md-row ga-4 align-center">
      <VSlider
        v-model="model.threshold"
        :min="0"
        :max="100"
        :step="1"
        class="w-md-50 w-100"
        thumb-label="always"
        thumb-size="12"
        hide-details
        :disabled="disabledSlider || props.disabled.value"
      />
      <span class="text-body-1">{{ $t('terms.of') }}</span>
      <div class="d-flex flex-row ga-4 w-md-50 w-100">
        <VAutocomplete
          v-model="userTypeModel"
          :label="$t('proposal_policies.user_type_select')"
          :items="userSelectorItems"
          item-value="value"
          item-title="text"
          variant="underlined"
          density="comfortable"
          :disabled="props.disabled.value"
        />
        <UserGroupAutocomplete
          v-if="variantIs(model.voters, 'Group')"
          v-model="model.voters.Group"
          :label="$t('proposal_policies.criteria_user_specifier.group')"
          multiple
          :disabled="props.disabled.value"
        />
        <UserAutocomplete
          v-else-if="variantIs(model.voters, 'Id')"
          v-model="model.voters.Id"
          :label="$t('proposal_policies.criteria_user_specifier.id')"
          multiple
          :disabled="props.disabled.value"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { computed, ref, toRefs, watch } from 'vue';
import UserAutocomplete from '~/components/inputs/UserAutocomplete.vue';
import UserGroupAutocomplete from '~/components/inputs/UserGroupAutocomplete.vue';
import { useUserSpecifierSelectorItems } from '~/composables/proposal-policies.composable';
import { ApprovalThreshold } from '~/generated/wallet/wallet.did';
import {
  mapProposalCriteriaUserSpecifierEnumToVariant,
  mapProposalCriteriaUserSpecifierToEnum,
} from '~/mappers/specifiers.mapper';
import { ProposalCriteriaUserSpecifierEnum } from '~/types/wallet.types';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue: ApprovalThreshold;
    disabled?: boolean;
  }>(),
  {
    disabled: false,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: ApprovalThreshold): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const userTypeModel = computed({
  get: () => mapProposalCriteriaUserSpecifierToEnum(model.value.voters),
  set: value => {
    model.value.voters = mapProposalCriteriaUserSpecifierEnumToVariant(value);
  },
});

const userSelectorItems = useUserSpecifierSelectorItems();
const disabledSlider = ref(false);

watch(
  () => userTypeModel.value,
  userType => {
    switch (userType) {
      case ProposalCriteriaUserSpecifierEnum.Proposer:
      case ProposalCriteriaUserSpecifierEnum.Owner:
        model.value.threshold = 100;
        disabledSlider.value = true;
        break;
      default:
        disabledSlider.value = false;
        break;
    }
  },
  { immediate: true },
);
</script>
