<template>
  <VSelect
    v-model="selectedAddRule"
    :label="$t('request_policies.add_rule_label')"
    :items="availableRules"
    item-value="value"
    item-title="text"
    hide-details
    density="comfortable"
    @update:model-value="onAddRule"
  />
</template>

<script setup lang="ts">
import { ref, toRefs } from 'vue';
import { useRequestSpecifierRules } from '~/composables/request-policies.composable';
import { RequestPolicyRule, RequestSpecifier } from '~/generated/station/station.did';
import { RequestPolicyRuleEnum } from '~/types/station.types';
import { unreachable } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    specifier: RequestSpecifier;
  }>(),
  {},
);

const props = toRefs(input);

const selectedAddRule = ref<RequestPolicyRuleEnum | null>(null);
const availableRules = useRequestSpecifierRules(props.specifier);

const emit = defineEmits<{
  (event: 'add', payload: RequestPolicyRule): void;
}>();

const onAddRule = (value: RequestPolicyRuleEnum | null): void => {
  if (value === null) {
    return;
  }

  selectedAddRule.value = null;
  switch (value) {
    case RequestPolicyRuleEnum.AllOf:
      emit('add', { AllOf: [] });
      break;
    case RequestPolicyRuleEnum.AnyOf:
      emit('add', { AnyOf: [] });
      break;
    case RequestPolicyRuleEnum.Not:
      emit('add', { Not: {} as RequestPolicyRule });
      break;
    case RequestPolicyRuleEnum.AutoApproved:
      emit('add', { AutoApproved: null });
      break;
    case RequestPolicyRuleEnum.AllowListed:
      emit('add', { AllowListed: null });
      break;
    case RequestPolicyRuleEnum.Quorum:
      emit('add', {
        Quorum: {
          approvers: { Any: null },
          min_approved: 1,
        },
      });
      break;
    case RequestPolicyRuleEnum.QuorumPercentage:
      emit('add', {
        QuorumPercentage: {
          approvers: { Any: null },
          min_approved: 100,
        },
      });
      break;
    case RequestPolicyRuleEnum.AllowListedByMetadata:
      emit('add', { AllowListedByMetadata: { key: '', value: '' } });
      break;
    default:
      unreachable(value);
  }
};
</script>
