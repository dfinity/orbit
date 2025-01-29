<template>
  <div class="d-flex flex-column ga-2">
    <div>
      {{ $t('request_policies.rule.not') }}
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
    <AddRuleSelect
      v-if="isEmpty && !props.disabled.value"
      :specifier="props.specifier.value"
      @add="model = $event"
    />
    <RuleBuilder
      v-else
      v-model="model"
      :specifier="props.specifier.value"
      :disabled="props.disabled.value"
      @remove="onRemove"
    />
  </div>
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { RequestPolicyRule, RequestSpecifier } from '~/generated/station/station.did';
import RuleBuilder from './RuleBuilder.vue';
import AddRuleSelect from '~/components/request-policies/rule/AddRuleSelect.vue';

const input = withDefaults(
  defineProps<{
    modelValue: RequestPolicyRule;
    specifier?: RequestSpecifier | null;
    disabled?: boolean;
  }>(),
  {
    disabled: false,
    specifier: null,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: RequestPolicyRule): void;
  (event: 'remove', payload: void): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const isEmpty = computed(() => !Object.keys(model.value).length);

const onRemove = (): void => {
  model.value = {} as RequestPolicyRule;
};
</script>
