<template>
  <AddRuleSelect
    v-if="model === null && !props.disabled.value"
    :specifier="props.specifier.value"
    @add="model = $event"
  />
  <template v-else-if="model !== null">
    <AllOfRule
      v-if="variantIs(model, 'AllOf')"
      v-model="model.AllOf"
      :specifier="props.specifier.value"
      :disabled="props.disabled.value"
      @remove="emit('remove')"
    />
    <AutoApprovedRule
      v-else-if="variantIs(model, 'AutoApproved')"
      :disabled="props.disabled.value"
      @remove="emit('remove')"
    />
    <AllowListedRule
      v-else-if="variantIs(model, 'AllowListed')"
      :disabled="props.disabled.value"
      @remove="emit('remove')"
    />
    <QuorumRule
      v-else-if="variantIs(model, 'Quorum')"
      v-model="model.Quorum"
      :disabled="props.disabled.value"
      @remove="emit('remove')"
    />
    <QuorumPercentageRule
      v-else-if="variantIs(model, 'QuorumPercentage')"
      v-model="model.QuorumPercentage"
      :disabled="props.disabled.value"
      @remove="emit('remove')"
    />
    <NotRule
      v-else-if="variantIs(model, 'Not')"
      v-model="model.Not"
      :specifier="props.specifier.value"
      :disabled="props.disabled.value"
      @remove="emit('remove')"
    />
    <AnyOfRule
      v-else-if="variantIs(model, 'AnyOf')"
      v-model="model.AnyOf"
      :specifier="props.specifier.value"
      :disabled="props.disabled.value"
      @remove="emit('remove')"
    />
    <AllowListedByMetadataRule
      v-else-if="variantIs(model, 'AllowListedByMetadata')"
      v-model="model.AllowListedByMetadata"
      :disabled="props.disabled.value"
      @remove="emit('remove')"
    />
  </template>
  <template v-else>
    <p class="text-medium-emphasis">{{ $t('app.request_policy_rule_builder_no_rule') }}</p>
  </template>
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { RequestPolicyRule, RequestSpecifier } from '~/generated/station/station.did';
import { variantIs } from '~/utils/helper.utils';
import AddRuleSelect from './AddRuleSelect.vue';
import AllOfRule from './AllOfRule.vue';
import AllowListedByMetadataRule from './AllowListedByMetadataRule.vue';
import AllowListedRule from './AllowListedRule.vue';
import AnyOfRule from './AnyOfRule.vue';
import AutoApprovedRule from './AutoApprovedRule.vue';
import NotRule from './NotRule.vue';
import QuorumPercentageRule from './QuorumPercentageRule.vue';
import QuorumRule from './QuorumRule.vue';

const input = withDefaults(
  defineProps<{
    modelValue?: RequestPolicyRule | null;
    specifier: RequestSpecifier;
    disabled?: boolean;
  }>(),
  {
    modelValue: null,
    disabled: false,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: RequestPolicyRule | null): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});
</script>
