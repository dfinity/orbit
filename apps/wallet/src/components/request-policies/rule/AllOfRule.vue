<template>
  <VDivider />
  <VCard variant="text" density="comfortable" v-bind="$attrs">
    <VCardTitle class="px-2">
      {{ $t('request_policies.rule.allof') }}
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
      <RuleBuilder
        v-for="(_, idx) of model"
        :key="idx"
        v-model="model[idx]"
        :specifier="props.specifier.value"
        :disabled="props.disabled.value"
        @remove="removeEntry(idx)"
      />
    </VCardText>
    <VCardActions class="px-2">
      <AddRuleSelect
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
import { RequestPolicyRule, RequestSpecifier } from '~/generated/station/station.did';
import AddRuleSelect from './AddRuleSelect.vue';
import RuleBuilder from './RuleBuilder.vue';

const input = withDefaults(
  defineProps<{
    modelValue?: RequestPolicyRule[];
    specifier: RequestSpecifier;
    disabled?: boolean;
  }>(),
  {
    modelValue: () => [],
    disabled: false,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: RequestPolicyRule[]): void;
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
