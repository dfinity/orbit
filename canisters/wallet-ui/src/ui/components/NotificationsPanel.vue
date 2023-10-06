<template>
  <VCard width="400">
    <VList density="compact">
      <VListItem density="compact" :title="$t('banks.pending_operations')" />
    </VList>
    <VDivider />
    <VList density="compact">
      <VListItem v-if="!activeBank.hasPendingOperations" class="text-center">
        {{ $t('terms.all_done') }}
      </VListItem>
      <VListItem v-for="(operation, idx) in activeBank.pendingOperations.items" :key="idx">
        <BankOperation
          v-model="activeBank.pendingOperations.items[idx]"
          @updated="() => save(operation)"
        />
        <VDivider v-if="activeBank.pendingOperations.items.length - 1 !== idx" class="mt-4" />
      </VListItem>
    </VList>
  </VCard>
</template>

<script lang="ts" setup>
import { useActiveBankStore } from '~/ui/stores';
import BankOperation from './operations/BankOperation.vue';
import { Operation } from '~/generated/bank/bank.did';

const activeBank = useActiveBankStore();

const save = (operation: Operation) => activeBank.saveOperation(operation);
</script>

<style lang="scss"></style>
