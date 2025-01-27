<template>
  <VBtn
    :size="props.size.value"
    :variant="props.variant.value"
    :icon="props.icon.value && !props.text.value"
    :color="props.color.value"
    @click="open = true"
  >
    <VIcon v-if="props.icon.value" :icon="props.icon.value" />
    <span v-if="props.text">{{ props.text.value }}</span>
  </VBtn>

  <NamedRuleDialog
    :open="open"
    :named-rule-id="props.namedRuleId.value"
    :readonly="props.readonly.value"
    @update:open="
      openEvent => {
        open = openEvent;

        emit('opened', openEvent);
      }
    "
  />
</template>
<script lang="ts" setup>
import { ref, toRefs } from 'vue';
import { VBtn, VIcon } from 'vuetify/components';
import { UUID } from '~/generated/station/station.did';
import NamedRuleDialog from './NamedRuleDialog.vue';

const input = withDefaults(
  defineProps<{
    namedRuleId?: UUID;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined' | 'elevated';
    color?: string;
    readonly?: boolean;
  }>(),
  {
    namedRuleId: undefined,
    icon: undefined,
    text: undefined,
    size: 'default',
    variant: 'elevated',
    color: 'primary',
    readonly: false,
  },
);

const open = ref(false);

const emit = defineEmits<{
  (event: 'opened', payload: boolean): void;
}>();

const props = toRefs(input);
</script>
