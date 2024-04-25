<template>
  <VBtn
    v-bind="$attrs"
    :size="props.size.value"
    :variant="props.variant.value"
    :icon="props.icon.value && !props.text.value"
    :color="props.color.value"
    @click="open = true"
  >
    <VIcon v-if="props.icon.value" class="mr-1" :icon="props.icon.value" />
    <slot name="default">
      <span v-if="props.text">{{ props.text.value }}</span>
    </slot>
    <VIcon v-if="props.appendIcon.value" class="ml-1" :icon="props.appendIcon.value" />
  </VBtn>

  <UserDialog
    :open="open"
    :user-id="props.userId.value"
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
import UserDialog from './UserDialog.vue';

const input = withDefaults(
  defineProps<{
    userId?: UUID;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined' | 'elevated';
    color?: string;
    readonly?: boolean;
    appendIcon?: string;
  }>(),
  {
    userId: undefined,
    icon: undefined,
    text: undefined,
    size: 'default',
    variant: 'elevated',
    color: 'primary',
    readonly: false,
    appendIcon: undefined,
  },
);

const open = ref(false);

const emit = defineEmits<{
  (event: 'opened', payload: boolean): void;
}>();

const props = toRefs(input);
</script>
