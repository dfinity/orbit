<template>
  <div class="d-flex flex-column ga-2">
    <div>
      {{ $t('request_policies.rule.quorumpercentage') }}
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
    <div class="d-flex flex-column flex-md-row ga-4 align-md-center">
      <VSlider
        v-model="model.min_approved"
        :min="0"
        :max="100"
        :step="1"
        class="flex-1-1"
        thumb-label="always"
        thumb-size="12"
        hide-details
        :readonly="disabledSlider || props.disabled.value"
      />
      <span class="text-body-1">{{ $t('terms.of') }}</span>
      <div class="d-flex flex-row ga-4 flex-1-1">
        <VAutocomplete
          v-model="userTypeModel"
          :label="$t('request_policies.user_type_select')"
          :items="userSelectorItems"
          item-value="value"
          item-title="text"
          variant="underlined"
          density="comfortable"
          :readonly="props.disabled.value"
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
import UserAutocomplete from '~/components/inputs/UserAutocomplete.vue';
import UserGroupAutocomplete from '~/components/inputs/UserGroupAutocomplete.vue';
import { useUserSpecifierSelectorItems } from '~/composables/request-policies.composable';
import { QuorumPercentage } from '~/generated/station/station.did';
import {
  mapRequestPolicyRuleUserSpecifierEnumToVariant,
  mapRequestPolicyRuleUserSpecifierToEnum,
} from '~/mappers/request-specifiers.mapper';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue: QuorumPercentage;
    disabled?: boolean;
  }>(),
  {
    disabled: false,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: QuorumPercentage): void;
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

const userSelectorItems = useUserSpecifierSelectorItems();
const disabledSlider = ref(false);

watch(
  () => userTypeModel.value,
  _ => {
    disabledSlider.value = false;
  },
  { immediate: true },
);
</script>
