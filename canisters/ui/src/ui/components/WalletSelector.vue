<template>
  <VSelect
    v-model="selectedWallet"
    :loading="session.loading"
    class="wallet-selector"
    eager
    :variant="app.isMobile ? 'filled' : 'solo'"
    density="compact"
    hide-details
    bg-color="background"
    item-value="canisterId"
    :no-data-text="$t('wallets.no_wallets')"
    :items="allWallets"
    :key="allWallets.length"
  >
    <template #item="{ props, item }">
      <VListItem
        v-bind="props"
        :title="computedWalletName({ canisterId: Principal.fromText(item.raw.canisterId) })"
        :subtitle="item.raw.canisterId"
      />
    </template>
    <template #selection="{ item }">
      <VListItem
        v-if="allWallets.length"
        :title="computedWalletName({ canisterId: Principal.fromText(item.raw.canisterId) })"
        :prepend-icon="mdiWallet"
      />
      <VListItem v-else :title="$t('wallets.no_wallets')" :prepend-icon="mdiWallet" />
    </template>

    <template #append-item>
      <AddWalletListItem />
    </template>
  </VSelect>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { computed } from 'vue';
import { useAppStore } from '~/ui/stores/app';
import { useSessionStore } from '~/ui/stores/session';
import { computedWalletName } from '~/ui/utils';
import AddWalletListItem from './add-wallet/AddWalletListItem.vue';
import { mdiWallet } from '@mdi/js';

const session = useSessionStore();
const app = useAppStore();

const allWallets = computed(()=>session.data.wallets);

const selectedWallet = computed({
  get(): string | null {
    return session.data.selectedWallet.canisterId ? session.data.selectedWallet.canisterId : null;
  },
  set(newWalletId: string | null) {
    if (!newWalletId) {
      session.disconnectWallet();
      return;
    }

    session.connectWallet(Principal.fromText(newWalletId));
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
    }
  }
}
</style>
