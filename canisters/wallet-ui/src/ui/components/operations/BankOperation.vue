<template>
  <div class="operation-item">
    <div v-if="props.loading" class="operation-item__loading"></div>
    <div class="operation-item__read">
      <VBtn
        v-if="decision"
        :icon="decision.read ? mdiCheckCircle : mdiCheckCircleOutline"
        size="x-small"
        :variant="decision.read ? 'text' : 'plain'"
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
    <div v-if="props.loading" class="operation-item__action">
      <VProgressCircular indeterminate color="primary" size="small" class="mx-4" />
    </div>
    <div v-else class="operation-item__action">
      <VMenu v-if="!decisionState.decided && !props.outer" :close-on-content-click="false">
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
        v-if="decisionState.decided || props.outer"
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
  mdiCog,
  mdiHelp,
} from '@mdi/js';
import { computed, provide } from 'vue';
import { Operation } from '~/generated/bank/bank.did';
import { i18n } from '~/ui/modules';
import UnknownOperation from './UnknownOperation.vue';
import { BankOperationType } from '~/types';
import ApproveTransferOperation from './ApproveTransferOperation.vue';
import { useActiveBankStore } from '~/ui/stores';

const activeBank = useActiveBankStore();
const props = withDefaults(
  defineProps<{
    operation: Operation;
    outer?: boolean;
    loading?: boolean;
  }>(),
  {
    outer: true,
    loading: false,
  },
);

provide('bankOperationProps', { outer: props.outer });

const emit = defineEmits<{
  (event: 'update:operation', payload: Operation): void;
  (event: 'read', payload: boolean): void;
  (event: 'adopted'): void;
  (event: 'rejected'): void;
}>();

const operation = computed({
  get: () => props.operation,
  set: value => emit('update:operation', value),
});

const decision = computed({
  get: () => operation.value.decisions.find(d => d.account_id === activeBank.account.id),
  set: value => {
    operation.value.decisions.forEach(d => {
      if (d.account_id === activeBank.account.id && value) {
        d = value;
      }
    });
  },
});

const onRead = () => {
  if (decision.value) {
    emit('read', !decision.value.read);
  }
};

const onApprove = () => {
  emit('adopted');
};

const onReject = () => {
  emit('rejected');
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
  } else if ('Pending' in operation.value.status) {
    chip = {
      color: 'warning',
      text: i18n.global.t('terms.pending'),
      icon: mdiCog,
    };
  }

  return {
    isPending: 'Pending' in operation.value.status,
    chip,
  };
});

const decisionState = computed(() => {
  const state: { decided: boolean } = {
    decided: false,
  };

  if (!decision.value) {
    return { decided: true };
  }

  if (decision.value && !('Pending' in decision.value.status)) {
    state.decided = true;
  }

  return state;
});
</script>
<style lang="scss">
.operation-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--ds-bdu);
  position: relative;

  &__loading {
    position: absolute;
    width: 100%;
    height: 100%;
    background: rgb(var(--ds-background));
    opacity: 0.4;
    z-index: 1;
  }

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
