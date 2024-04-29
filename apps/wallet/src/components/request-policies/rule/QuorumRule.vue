<template>
  <div class="d-flex flex-column ga-2">
    <div>
      {{ $t('request_policies.rule.quorum') }}
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
          v-model="quorum"
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
          :label="$t('request_policies.user_type_select')"
          :items="userSelectorItems"
          item-value="value"
          item-title="text"
          :disabled="props.disabled.value"
          variant="underlined"
          density="comfortable"
        />
        <UserGroupAutocomplete
          v-if="variantIs(model.approvers, 'Group')"
          v-model="model.approvers.Group"
          :label="$t('request_policies.rule_user_specifier.group')"
          multiple
          :disabled="props.disabled.value"
        />
        <UserAutocomplete
          v-else-if="variantIs(model.approvers, 'Id')"
          v-model="model.approvers.Id"
          :label="$t('request_policies.rule_user_specifier.id')"
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
import { useUserSpecifierSelectorItems } from '~/composables/request-policies.composable';
import { Quorum } from '~/generated/station/station.did';
import {
  mapRequestPolicyRuleUserSpecifierEnumToVariant,
  mapRequestPolicyRuleUserSpecifierToEnum,
} from '~/mappers/request-specifiers.mapper';
import { FormValidationRules } from '~/types/helper.types';
import { intNumberRangeRule, requiredRule } from '~/utils/form.utils';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue: Quorum;
    disabled?: boolean;
  }>(),
  {
    disabled: false,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Quorum): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const userTypeModel = computed({
  get: () => mapRequestPolicyRuleUserSpecifierToEnum(model.value.approvers),
  set: value => {
    model.value.approvers = mapRequestPolicyRuleUserSpecifierEnumToVariant(value);
  },
});

const quorum = computed({
  get: () => model.value.min_approved,
  set: min => {
    if (min && typeof min !== 'number') {
      min = parseInt(min, 10);
    }

    // needs a reassignment to trigger the reactivity of the model
    model.value = {
      ...model.value,
      min_approved: min,
    };
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
  _ => {
    disabledInput.value = false;
  },
  { immediate: true },
);
</script>
