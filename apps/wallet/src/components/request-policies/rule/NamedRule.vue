<template>
  <div class="d-flex align-center justify-start">
    {{ $t('request_policies.rule.named_rule') }}
    <template v-if="loading">
      <VProgressCircular indeterminate />
    </template>
    <template v-else-if="error">
      {{ $t('request_policies.rule.named_rule') }}
    </template>
    <template v-else-if="namedRule">
      <VTooltip
        v-if="tooltip"
        location="bottom"
        content-class="white-space-pre-wrap"
        :text="tooltip"
      >
        <template #activator="{ props: activatorProps }">
          <span v-bind="activatorProps" class="underline-dotted ml-2 font-weight-bold">
            {{ namedRule.name }}
          </span>
        </template>
      </VTooltip>
    </template>
    <template v-else>
      {{ namedRuleId }}
    </template>
    <VBtn
      v-if="!props.disabled.value"
      :icon="mdiTrashCanOutline"
      variant="flat"
      size="small"
      color="transparent"
      density="compact"
      class="ml-2"
      @click="emit('remove')"
    />
  </div>
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { ref } from 'vue';
import { onMounted } from 'vue';
import { computed } from 'vue';
import { toRefs } from 'vue';
import { useRuleToTooltip } from '~/composables/request-policies.composable';
import { NamedRule, UUID } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';

const input = withDefaults(
  defineProps<{
    namedRuleId: UUID;
    disabled?: boolean;
  }>(),
  {
    disabled: false,
  },
);

const props = toRefs(input);

const namedRule = ref<NamedRule | null>(null);
const loading = ref(true);
const error = ref<boolean>(false);
const appStore = useAppStore();

const rule = computed(() => (namedRule.value ? namedRule.value.rule : null));

const tooltip = useRuleToTooltip(rule);

const emit = defineEmits<{
  (event: 'remove', payload: void): void;
}>();

onMounted(async () => {
  loading.value = true;
  try {
    namedRule.value = (await services().station.getNamedRule(props.namedRuleId.value)).named_rule;
  } catch (e) {
    appStore.sendErrorNotification(e);
    error.value = true;
  } finally {
    loading.value = false;
  }
});
</script>
