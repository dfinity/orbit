<template>
  <VSelect
    v-model="selectedWallet"
    :loading="session.loading"
    class="wallet-selector"
    variant="filled"
    hide-details
    item-value="canisterId"
    :no-data-text="$t('wallets.no_wallets')"
    :items="session.user?.wallets || []"
  >
    <template #item="{ props, item }">
      <VListItem
        v-bind="props"
        :title="session.computedWalletName(Principal.fromText(item.raw.canisterId))"
        :subtitle="item.raw.canisterId"
      />
    </template>
    <template #selection="{ item }">
      <VListItem
        v-if="session.hasWallets"
        :title="session.computedWalletName(Principal.fromText(item.raw.canisterId))"
        :subtitle="item.raw.canisterId"
        :prepend-icon="mdiWallet"
      />
      <VListItem v-else :title="$t('wallets.no_wallets')" :prepend-icon="mdiWallet" />
    </template>
  </VSelect>
</template>
<script lang="ts" setup>
import { computed } from 'vue';
import { Principal } from '@dfinity/principal';
import { mdiWallet } from '@mdi/js';
import { useSessionStore } from '~/ui/stores';

const session = useSessionStore();

const selectedWallet = computed({
  get(): string | null {
    return session.selectedWallet ? session.selectedWallet.toText() : null;
  },
  set(newWalletId: string | null) {
    if (!newWalletId) {
      session.unloadWallet();
      return;
    }

    session.loadWallet(Principal.fromText(newWalletId));
  },
});
</script>

<style lang="scss">
.wallet-selector {
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
