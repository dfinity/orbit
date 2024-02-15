<template>
  <div class="d-flex flex-column ga-2">
    <div>
      {{ $t('proposal_policies.criteria.minimumvotes') }}
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
      <div class="w-md-50 w-100">
        <VTextField
          v-model="model.minimum"
          :label="$t('terms.min')"
          type="number"
          :rules="rules.min"
          :disabled="disabledInput || props.disabled.value"
          density="comfortable"
          variant="underlined"
        />
      </div>
      <span class="text-body-1">{{ $t('terms.of') }}</span>
      <div class="d-flex flex-row ga-4 w-md-50 w-100">
        <VAutocomplete
          v-model="userTypeModel"
          :label="$t('proposal_policies.user_type_select')"
          :items="userSelectorItems"
          item-value="value"
          item-title="text"
          :disabled="props.disabled.value"
          variant="underlined"
          density="comfortable"
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
import { useI18n } from 'vue-i18n';
import UserAutocomplete from '~/components/inputs/UserAutocomplete.vue';
import UserGroupAutocomplete from '~/components/inputs/UserGroupAutocomplete.vue';
import { useUserSpecifierSelectorItems } from '~/composables/proposal-policies.composable';
import { MinimumVotes } from '~/generated/wallet/wallet.did';
import {
  mapProposalCriteriaUserSpecifierEnumToVariant,
  mapProposalCriteriaUserSpecifierToEnum,
} from '~/mappers/specifiers.mapper';
import { FormValidationRules } from '~/types/helper.types';
import { ProposalCriteriaUserSpecifierEnum } from '~/types/wallet.types';
import { intNumberRangeRule, requiredRule } from '~/utils/form.utils';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue: MinimumVotes;
    disabled?: boolean;
  }>(),
  {
    disabled: false,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: MinimumVotes): void;
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
const i18n = useI18n();

const rules: {
  min: FormValidationRules;
} = {
  min: [requiredRule, intNumberRangeRule(i18n.t('terms.min'), 1)],
};

const disabledInput = ref(false);

watch(
  () => userTypeModel.value,
  userType => {
    switch (userType) {
      case ProposalCriteriaUserSpecifierEnum.Proposer:
        model.value.minimum = 1;
        disabledInput.value = true;
        break;
      default:
        disabledInput.value = false;
        break;
    }
  },
  { immediate: true },
);
</script>
