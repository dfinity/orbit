<template>
  <div class="d-flex flex-column ga-2">
    <div>
      {{ $t('proposal_policies.criteria.minimumvotes') }}
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
    <div class="d-flex flex-column flex-md-row ga-4 align-center">
      <div class="w-md-50 w-100">
        <VTextField
          v-model="model[1]"
          :label="$t('terms.min')"
          type="number"
          :rules="rules.min"
          :disabled="disabledInput"
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
          variant="underlined"
          density="comfortable"
        />
        <UserGroupsAutocomplete
          v-if="variantIs(model[0], 'Group')"
          v-model="model[0].Group"
          :label="$t('proposal_policies.criteria_user_specifier.group')"
          multiple
        />
        <UsersAutocomplete
          v-else-if="variantIs(model[0], 'Id')"
          v-model="model[0].Id"
          :label="$t('proposal_policies.criteria_user_specifier.id')"
          multiple
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { computed, ref, toRefs, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import UserGroupsAutocomplete from '~/components/inputs/UserGroupsAutocomplete.vue';
import UsersAutocomplete from '~/components/inputs/UsersAutocomplete.vue';
import { useUserSpecifierSelectorItems } from '~/composables/proposal-policies.composable';
import { UserSpecifier } from '~/generated/wallet/wallet.did';
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
    modelValue: [UserSpecifier, number];
  }>(),
  {},
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: [UserSpecifier, number]): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const userTypeModel = computed({
  get: () => mapProposalCriteriaUserSpecifierToEnum(model.value[0]),
  set: value => {
    model.value[0] = mapProposalCriteriaUserSpecifierEnumToVariant(value);
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
      case ProposalCriteriaUserSpecifierEnum.Owner:
        model.value[1] = 1;
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
