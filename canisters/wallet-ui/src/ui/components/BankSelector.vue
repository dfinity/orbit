<template>
  <VSelect
    v-model="bankStore.main"
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
        :title="bankStore.computedBankName(item.raw.canisterId as Principal)"
        :subtitle="item.raw.canisterId?.toText()"
      />
    </template>
    <template #selection="{ item }">
      <VListItem
        v-if="bankStore.hasBanks"
        :title="bankStore.computedBankName(item.raw.canisterId as Principal)"
        :subtitle="item.raw.canisterId?.toText()"
        :prepend-icon="mdiBank"
      />
      <VListItem v-else :title="$t('banks.no_banks')" :prepend-icon="mdiBank" />
    </template>
  </VSelect>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiBank } from '@mdi/js';
import { useBankStore } from '~/ui/stores';

const bankStore = useBankStore();
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
