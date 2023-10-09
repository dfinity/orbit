<template>
  <div class="operation-item">
    <div class="operation-item__read">
      <VBtn
        :icon="operation.read ? mdiCheckCircle : mdiCheckCircleOutline"
        size="x-small"
        :variant="operation.read ? 'text' : 'plain'"
        @click="onRead"
      />
    </div>
    <div class="operation-item__code">
      <ApproveTransferOperation
        v-if="operation.code === BankOperationType.ApproveTransfer"
        v-model="operation"
      />
      <UnknownOperation v-else v-model="operation" />
    </div>
    <div class="operation-item__action">
      <VMenu v-if="operationState.isPending" :close-on-content-click="false">
        <template #activator="{ props: actionProps }">
          <VBtn v-bind="actionProps" :prepend-icon="mdiCogs" size="small" variant="text" block>
            {{ $t(`terms.edit`) }}
          </VBtn>
        </template>
        <VList density="compact" :lines="false" class="py-0">
          <VListItem density="compact" class="px-1">
            <VBtn
              :prepend-icon="mdiClose"
              size="small"
              color="error"
              variant="tonal"
              block
              @click="onReject"
            >
              {{ $t(`terms.reject`) }}
            </VBtn>
          </VListItem>
          <VListItem density="compact" class="px-1">
            <VBtn
              :prepend-icon="mdiCheck"
              size="small"
              color="success"
              variant="tonal"
              block
              @click="onApprove"
            >
              {{ $t(`terms.approve`) }}
            </VBtn>
          </VListItem>
        </VList>
      </VMenu>
      <VChip
        v-else
        :prepend-icon="operationState.chip.icon"
        size="x-small"
        :color="operationState.chip.color"
        variant="tonal"
      >
        {{ operationState.chip.text }}
      </VChip>
    </div>
  </div>
</template>
<script lang="ts" setup>
import {
  mdiCheckCircleOutline,
  mdiCheckCircle,
  mdiCheck,
  mdiClose,
  mdiCogs,
  mdiHelp,
} from '@mdi/js';
import { computed, provide } from 'vue';
import { Operation } from '~/generated/bank/bank.did';
import { i18n } from '~/ui/modules';
import UnknownOperation from './UnknownOperation.vue';
import { BankOperationType } from '~/types';
import ApproveTransferOperation from './ApproveTransferOperation.vue';

const props = withDefaults(
  defineProps<{
    modelValue: Operation;
    outer?: boolean;
  }>(),
  {
    outer: true,
  },
);

provide('bankOperationProps', { outer: props.outer });

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Operation): void;
  (event: 'updated'): void;
}>();

const operation = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const onRead = () => {
  operation.value.read = !operation.value.read;
  emit('updated');
};

const onApprove = () => {
  operation.value.status = { Adopted: null };
  emit('updated');
};

const onReject = () => {
  operation.value.status = { Rejected: null };
  emit('updated');
};

const operationState = computed(() => {
  let chip: { color: string; text: string; icon: string } = {
    color: 'info',
    text: i18n.global.t('terms.abstained'),
    icon: mdiHelp,
  };
  if ('Adopted' in operation.value.status) {
    chip = {
      color: 'success',
      text: i18n.global.t('terms.approved'),
      icon: mdiCheck,
    };
  } else if ('Rejected' in operation.value.status) {
    chip = {
      color: 'error',
      text: i18n.global.t('terms.rejected'),
      icon: mdiClose,
    };
  }

  return {
    isPending: 'Pending' in operation.value.status,
    chip,
  };
});
</script>
<style lang="scss">
.operation-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--ds-bdu);

  &__read,
  &__action {
    flex: 0 0 auto;
  }

  &__action {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--ds-bdu);
  }

  &__code {
    flex: 1 1 auto;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: calc(var(--ds-bdu) / 2);
    border-right: var(--ds-border-width) var(--ds-border-style) rgb(var(--ds-background));

    &__title {
      font-weight: 500;
      text-transform: capitalize;
      font-size: var(--ds-font-size-xs);
      line-height: 20px;
      color: var(--ds-text-primary);
      white-space: nowrap;
    }

    &__time {
      display: flex;
      white-space: nowrap;
      font-weight: 400;
      font-size: var(--ds-font-size-xxs);
      line-height: var(--ds-font-size-xxs);
      color: var(--ds-text-secondary);
      gap: calc(var(--ds-bdu) / 2);
    }
  }
}
</style>
