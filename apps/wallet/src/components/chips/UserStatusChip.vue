<template>
  <VChip size="small" :color="chipColor" :class="$props.class">
    <slot>
      {{ $t(`app.user_status_${$props.status.toLowerCase()}`) }}
    </slot>
  </VChip>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { UserStatusType } from '~/types/wallet.types';

const props = withDefaults(
  defineProps<{
    status: UserStatusType;
    class?: string;
  }>(),
  {
    class: 'text-lowercase',
  },
);

const chipColor = computed(() => {
  switch (props.status) {
    case UserStatusType.Active:
      return 'success';
    case UserStatusType.Inactive:
      return 'error';
    default:
      return 'default';
  }
});
</script>
