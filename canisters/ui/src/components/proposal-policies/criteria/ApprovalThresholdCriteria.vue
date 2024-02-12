<template>
  <div class="d-flex flex-column ga-2">
    <div>
      {{ $t('proposal_policies.criteria.approvalthreshold') }}
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
    <div class="d-flex flex-row ga-4 align-center">
      <VSlider
        v-model="model[1]"
        :min="0"
        :max="100"
        :step="1"
        class="w-25"
        thumb-label="always"
        thumb-size="12"
        hide-details
      />
      <span class="text-body-1">{{ $t('terms.of') }}</span>
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
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { computed, toRefs } from 'vue';
import UserGroupsAutocomplete from '~/components/inputs/UserGroupsAutocomplete.vue';
import UsersAutocomplete from '~/components/inputs/UsersAutocomplete.vue';
import { useUserSpecifierSelectorItems } from '~/composables/proposal-policies.composable';
import { UserSpecifier } from '~/generated/wallet/wallet.did';
import {
  mapProposalCriteriaUserSpecifierEnumToVariant,
  mapProposalCriteriaUserSpecifierToEnum,
} from '~/mappers/specifiers.mapper';
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
</script>
