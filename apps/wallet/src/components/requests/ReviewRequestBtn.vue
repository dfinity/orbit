<template>
  <VBtn
    v-bind="$attrs"
    data-test-id="review-request-btn"
    density="comfortable"
    :size="props.size"
    :variant="props.variant"
    :icon="props.icon && !btnText"
    :color="props.color"
    @click="openDialog"
  >
    <slot name="default">
      {{ btnText }}
    </slot>
  </VBtn>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn } from 'vuetify/components';
import { useRequestOverlay } from '~/composables/request.composable';
import { UUID } from '~/generated/station/station.did';

const requestOverlay = useRequestOverlay();

const props = withDefaults(
  defineProps<{
    requestId: UUID;
    canApprove: boolean;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined' | 'elevated';
    color?: string;
    readonly?: boolean;
  }>(),
  {
    request: undefined,
    canApprove: false,
    icon: undefined,
    text: undefined,
    size: 'default',
    variant: 'elevated',
    color: 'secondary',
    readonly: false,
  },
);

defineEmits<{
  (event: 'approved'): void;
  (event: 'closed'): void;
  (event: 'opened'): void;
}>();

const i18n = useI18n();
const btnText = computed(
  () => props.text || (props.canApprove ? i18n.t('terms.review') : i18n.t('terms.view')),
);

function openDialog() {
  requestOverlay.open(props.requestId);
}
</script>
