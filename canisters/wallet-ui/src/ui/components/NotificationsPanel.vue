<template>
  <VCard :width="mobile ? '100%' : '400px'">
    <VList density="compact">
      <VListItem density="compact" class="notifications-panel__title">
        {{ $t('banks.pending_operations') }}
        <VSpacer />
        <VBtn :icon="mdiClose" variant="flat" @click="emit('close')" />
      </VListItem>
    </VList>
    <VDivider />
    <VList density="compact">
      <VListItem v-if="!activeBank.hasPendingOperations" class="text-center">
        {{ $t('terms.all_done') }}
      </VListItem>
      <VListItem
        v-for="({ loading, data }, idx) in activeBank.sortedPendingOperations"
        :key="idx"
      >
        <BankOperation
          :loading="loading"
          :operation="activeBank.sortedPendingOperations[idx].data"
          @read="read => onRead(data, read)"
          @adopted="submitDecision(data, true)"
          @rejected="submitDecision(data, false)"
        />
        <VDivider v-if="activeBank.pendingOperations.items.length - 1 !== idx" class="mt-4" />
      </VListItem>
    </VList>
  </VCard>
</template>

<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { useActiveBankStore } from '~/ui/stores';
import BankOperation from './operations/BankOperation.vue';
import { Operation } from '~/generated/bank/bank.did';
import { useDisplay } from 'vuetify';

const { mobile } = useDisplay();
const activeBank = useActiveBankStore();

const emit = defineEmits<{
  (event: 'close'): void;
}>();

const onRead = (operation: Operation, read: boolean) =>
  activeBank.saveDecision(operation.id, { read });

const submitDecision = (operation: Operation, approve: boolean) =>
  activeBank.saveDecision(operation.id, { approve });
</script>

<style lang="scss">
.notifications-panel {
  &__title {
    .v-list-item__content {
      display: flex;
      flex-direction: row;
      align-items: center;
    }
  }
}
</style>
