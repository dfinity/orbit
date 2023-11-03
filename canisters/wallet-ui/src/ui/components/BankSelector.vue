<template>
  <VSelect
    v-model="selectedBank"
    :loading="activeBankStore.loading"
    class="bank-selector"
    variant="filled"
    hide-details
    item-value="canisterId"
    :no-data-text="$t('banks.no_banks')"
    :items="bankStore.banks"
  >
    <template #item="{ props, item }">
      <VListItem
        v-bind="props"
        :title="bankStore.computedBankName(Principal.fromText(item.raw.canisterId))"
        :subtitle="item.raw.canisterId"
      />
    </template>
    <template #selection="{ item }">
      <VListItem
        v-if="bankStore.hasBanks"
        :title="bankStore.computedBankName(Principal.fromText(item.raw.canisterId))"
        :subtitle="item.raw.canisterId"
        :prepend-icon="mdiBank"
      />
      <VListItem v-else :title="$t('banks.no_banks')" :prepend-icon="mdiBank" />
    </template>
  </VSelect>
</template>
<script lang="ts" setup>
import { computed } from 'vue';
import { Principal } from '@dfinity/principal';
import { mdiBank } from '@mdi/js';
import { useActiveBankStore, useBankStore } from '~/ui/stores';

const bankStore = useBankStore();
const activeBankStore = useActiveBankStore();

const selectedBank = computed({
  get(): string | null {
    return activeBankStore.hasUser ? activeBankStore.bankId.toString() : null;
  },
  set(newBankId: string | null) {
    if (!newBankId) {
      bankStore._main = null;
      activeBankStore.reset();
      return;
    }

    activeBankStore.load(Principal.fromText(newBankId)).then(() => {
      bankStore._main = activeBankStore.bankId.toString();
    });
  },
});
</script>

<style lang="scss">
.bank-selector {
  .v-field__input {
    padding-top: calc(var(--ds-bdu) / 2);
    padding-bottom: calc(var(--ds-bdu) / 2);
  }

  .v-select__selection {
    .v-list-item__prepend {
      .v-list-item__spacer {
        width: calc(var(--ds-bdu) * 2);
      }
    }
    > .v-list-item {
      padding-left: 0;
    }

    .v-list-item__content {
      text-overflow: ellipsis;
      max-width: 260px;
    }
  }
}
</style>
