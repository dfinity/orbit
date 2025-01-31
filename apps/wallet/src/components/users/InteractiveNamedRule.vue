<template>
  <template v-if="loading">
    <VProgressCircular indeterminate />
  </template>
  <template v-else-if="error">
    {{ id }}
  </template>
  <template v-else-if="namedRule">
    <VTooltip v-if="tooltip" location="bottom" content-class="white-space-pre-wrap" :text="tooltip">
      <template #activator="{ props: activatorProps }">
        <span v-bind="activatorProps" class="underline-dotted font-weight-bold">
          {{ namedRule.name }}
        </span>
      </template>
    </VTooltip>
  </template>
  <template v-else>
    {{ id }}
  </template>
</template>

<script setup lang="ts">
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
    id: UUID;
    name?: string;
  }>(),
  {
    name: undefined,
  },
);

const props = toRefs(input);
const namedRule = ref<NamedRule | null>(null);
const loading = ref(true);
const error = ref<boolean>(false);
const appStore = useAppStore();

const rule = computed(() => (namedRule.value ? namedRule.value.rule : null));
const tooltip = useRuleToTooltip(rule);

onMounted(async () => {
  loading.value = true;
  try {
    namedRule.value = (await services().station.getNamedRule(props.id.value)).named_rule;
  } catch (e) {
    appStore.sendErrorNotification(e);
    error.value = true;
  } finally {
    loading.value = false;
  }
});
</script>
