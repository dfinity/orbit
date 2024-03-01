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

  <UserGroupDialog
    :open="open"
    :user-group-id="props.userGroupId.value"
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
import { UUID } from '~/generated/wallet/wallet.did';
import UserGroupDialog from './UserGroupDialog.vue';

const input = withDefaults(
  defineProps<{
    userGroupId?: UUID;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined';
    color?: string;
    readonly?: boolean;
    appendIcon?: string;
  }>(),
  {
    userGroupId: undefined,
    icon: undefined,
    text: undefined,
    size: 'default',
    variant: 'flat',
    color: 'primary-variant',
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
