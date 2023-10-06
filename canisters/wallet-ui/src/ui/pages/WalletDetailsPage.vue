<template>
  <PageLayout v-if="activeBank.hasAccount" class="wallet">
    <template v-if="pageStore.hasLoaded" #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8 wallet__header" fluid>
        <div class="wallet__balance">
          <span
            v-if="pageStore.wallet.balance?.[0]"
            :title="pageStore.wallet.balance?.[0]?.last_update_timestamp"
          >
            {{ formatBalance(pageStore.wallet.balance[0].balance, pageStore.wallet.decimals) }}
          </span>
          <span v-else>-</span>
          &nbsp;{{ pageStore.wallet.symbol }}
        </div>
        <VRow>
          <VCol cols="12" sm="8" class="wallet__header__details">
            <div>
              <h1 class="text-h4">
                <VIcon :icon="mdiWallet" size="x-small" />
                {{ pageStore.wallet.name?.[0] ?? $t('terms.wallet') }}
              </h1>
            </div>
            <div class="wallet__header__details__addr">
              <small class="wallet__header__details__addr__text">
                {{ pageStore.wallet.symbol }}: {{ pageStore.wallet.address }}
              </small>
              <VBtn
                class="wallet-card__subtitle__copy"
                size="x-small"
                variant="text"
                :icon="mdiContentCopy"
                @click="copyAddressToClipboard(`${pageStore.wallet.address}`)"
              />
            </div>
            <div>
              <VChip
                size="x-small"
                color="primary-variant"
                variant="tonal"
                :prepend-icon="pageStore.wallet.owners.length > 1 ? mdiAccountGroup : mdiAccount"
              >
                {{
                  pageStore.wallet.owners.length > 1
                    ? $t('banks.joint_wallet')
                    : $t('banks.private_wallet')
                }}
              </VChip>
            </div>
          </VCol>
          <VCol cols="12" sm="4" class="d-flex">
            <VSpacer />
            <NewTransferBtn :wallet-id="pageStore.wallet.id" />
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template v-else-if="!pageStore.loading" #main-header>
      <div class="wallet__not-found pb-16">
        <header class="text-h3 wallet__not-found__title">
          {{ $t('wallet_page.not_found_title') }}
        </header>
        <p class="text-h6">
          {{ $t('wallet_page.not_found_description') }}
        </p>
        <VBtn color="primary-variant mt-8" :append-icon="_mdiLink" :to="{ name: 'Wallets' }">
          {{ $t('wallet_page.not_found_btn') }}
        </VBtn>
      </div>
    </template>
    <template v-else #main-header>
      <VProgressLinear indeterminate color="primary" />
    </template>
    <template v-if="pageStore.hasLoaded" #main-body>
      <VRow>
        <VCol cols="12">
          <VCard color="background" variant="flat">
            <VTabs v-model="tab" center-active class="px-8">
              <VTab
                :loading="pageStore.transfers.loading"
                value="transfers"
                class="wallet__tab__item"
              >
                {{ $t(`terms.transfers`) }}
              </VTab>
              <VTab
                :loading="pageStore.transfers.loading"
                value="operations"
                class="wallet__tab__item"
              >
                {{ $t(`terms.operations`) }}
              </VTab>
            </VTabs>
            <VCardText>
              <VWindow v-model="tab">
                <VWindowItem value="transfers">
                  <VContainer>
                    <VRow>
                      <VCol cols="12" md="4" class="py-0">
                        <VTextField
                          v-model="pageStore.transfers.fromDt"
                          :prepend-inner-icon="mdiCalendar"
                          density="compact"
                          type="date"
                          :label="$t(`terms.from`)"
                          variant="solo"
                          :disabled="pageStore.transfers.loading"
                          class="mb-2"
                          hide-details
                        />
                      </VCol>
                      <VCol cols="12" md="4" class="py-0">
                        <VTextField
                          v-model="pageStore.transfers.toDt"
                          :prepend-inner-icon="mdiCalendar"
                          density="compact"
                          type="date"
                          :label="$t(`terms.until`)"
                          :disabled="pageStore.transfers.loading"
                          variant="solo"
                          class="mb-2"
                          hide-details
                        />
                      </VCol>
                      <VCol cols="12" md="4">
                        <VBtn
                          block
                          variant="tonal"
                          color="primary-variant"
                          :prepend-icon="mdiRefresh"
                          :loading="pageStore.transfers.loading"
                          @click="
                            pageStore.loadSentTransfers(
                              pageStore.transfers.fromDt
                                ? new Date(pageStore.transfers.fromDt)
                                : undefined,
                              pageStore.transfers.toDt
                                ? new Date(pageStore.transfers.toDt)
                                : undefined,
                            )
                          "
                        >
                          {{ $t(`terms.search`) }}
                        </VBtn>
                      </VCol>
                      <VCol cols="12">
                        <VTable v-if="pageStore.transfers.items.length" hover class="transfers">
                          <tbody>
                            <tr v-for="(transfer, _idx) in pageStore.sortedTransfers" :key="_idx">
                              <td class="transfers__item__icon"><VIcon :icon="mdiTransfer" /></td>
                              <td class="transfers__item__details">
                                <div class="transfers__item__details--amount">
                                  {{
                                    `${formatBalance(transfer.amount, 8)} ${
                                      pageStore.wallet.symbol
                                    }`
                                  }}
                                </div>
                                <div class="transfers__item__details--created_at">
                                  <small>{{ transfer.created_at }}</small>
                                </div>
                              </td>
                              <td class="transfers__item__status text-right">
                                <VChip size="small">{{
                                  extractTransferStatus(transfer.status)
                                }}</VChip>
                              </td>
                            </tr>
                          </tbody>
                        </VTable>
                        <p v-else class="text-h6">{{ $t(`banks.no_transfers_found_search`) }}</p>
                      </VCol>
                    </VRow>
                  </VContainer>
                </VWindowItem>
                <VWindowItem value="operations">
                  <VContainer>
                    <VRow>
                      <VCol cols="12" md="4" class="py-0">
                        <VTextField
                          v-model="pageStore.operations.fromDt"
                          :prepend-inner-icon="mdiCalendar"
                          density="compact"
                          type="date"
                          :label="$t(`terms.from`)"
                          variant="solo"
                          :disabled="pageStore.operations.loading"
                          class="mb-2"
                          hide-details
                        />
                      </VCol>
                      <VCol cols="12" md="4" class="py-0">
                        <VTextField
                          v-model="pageStore.operations.toDt"
                          :prepend-inner-icon="mdiCalendar"
                          density="compact"
                          type="date"
                          :label="$t(`terms.until`)"
                          :disabled="pageStore.operations.loading"
                          variant="solo"
                          class="mb-2"
                          hide-details
                        />
                      </VCol>
                      <VCol cols="12" md="4">
                        <VBtn
                          block
                          variant="tonal"
                          color="primary-variant"
                          :prepend-icon="mdiRefresh"
                          :loading="pageStore.operations.loading"
                          @click="
                            pageStore.loadOperations(
                              pageStore.operations.fromDt
                                ? new Date(pageStore.operations.fromDt)
                                : undefined,
                              pageStore.operations.toDt
                                ? new Date(pageStore.operations.toDt)
                                : undefined,
                            )
                          "
                        >
                          {{ $t(`terms.search`) }}
                        </VBtn>
                      </VCol>
                      <VCol cols="12">
                        <VTable v-if="pageStore.operations.items.length" hover class="operations">
                          <tbody>
                            <tr v-for="(operation, _idx) in pageStore.sortedOperations" :key="_idx">
                              <td class="py-4">
                                <BankOperation
                                  v-model="pageStore.sortedOperations[_idx]"
                                  :outer="false"
                                  :details="getOperationDetails(operation)"
                                  @updated="() => saveOperation(operation)"
                                />
                              </td>
                            </tr>
                          </tbody>
                        </VTable>
                        <p v-else class="text-h6">{{ $t(`banks.no_operations_found_search`) }}</p>
                      </VCol>
                    </VRow>
                  </VContainer>
                </VWindowItem>
              </VWindow>
            </VCardText>
          </VCard>
        </VCol>
      </VRow>
    </template>
  </PageLayout>
  <PageLayout v-else>
    <template #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8" fluid>
        <VRow>
          <VCol sm="12">
            <h1 class="text-h4">{{ $t('banks.no_bank_account') }}</h1>
            <p class="text-subtitle">{{ $t('banks.please_register_to_continue') }}</p>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import {
  mdiContentCopy,
  mdiWallet,
  mdiAccountGroup,
  mdiAccount,
  mdiLink as _mdiLink,
  mdiTransfer,
  mdiRefresh,
  mdiCalendar,
} from '@mdi/js';
import { onMounted, ref } from 'vue';
import { formatBalance, extractTransferStatus } from '~/core';
import NewTransferBtn from '~/ui/components/NewTransferBtn.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import { i18n, router } from '~/ui/modules';
import { useActiveBankStore, useSettingsStore, useWalletDetailsStore } from '~/ui/stores';
import BankOperation from '~/ui/components/operations/BankOperation.vue';
import { BankOperationType } from '~/types';
import { Operation } from '~/generated/bank/bank.did';

const activeBank = useActiveBankStore();
const settings = useSettingsStore();
const pageStore = useWalletDetailsStore();

const tab = ref<'transfers' | 'operations'>('transfers');

const saveOperation = async (operation: Operation) => {
  await activeBank.saveOperation(operation);

  pageStore.loadSentTransfers(
    pageStore.transfers.fromDt ? new Date(pageStore.transfers.fromDt) : undefined,
    pageStore.transfers.toDt ? new Date(pageStore.transfers.toDt) : undefined,
  );
};

const getOperationDetails = (operation: Operation): Record<string, string> => {
  const details: Record<string, string> = {};

  if (operation.code === BankOperationType.ApproveTransfer) {
    const found = operation.metadata.find(([k]) => k === 'transfer_id');
    if (found) {
      const [_, transferId] = found;
      const transfer = pageStore.transfers.items.find(t => t.transfer_id === transferId);

      if (transfer) {
        details[i18n.global.t(`terms.amount`).toLowerCase()] = formatBalance(
          transfer.amount,
          pageStore.wallet.decimals,
        );
        details[i18n.global.t(`terms.to`).toLowerCase()] = transfer.to;
      }
    }
  }

  return details;
};

onMounted(() => {
  pageStore.load(`${router.currentRoute.value.params.id}`);
});

const copyAddressToClipboard = (address: string) => {
  navigator.clipboard.writeText(address);

  settings.setNotification({
    show: true,
    type: 'success',
    message: i18n.global.t('banks.wallet_address_copied_to_clipboard'),
  });
};
</script>

<style scoped lang="scss">
.header-actions {
  display: flex;
  justify-content: end;
  align-items: center;
  gap: calc(var(--ds-bdu) * 2);
}

.page-layout--mobile {
  .header-actions {
    justify-content: center;
  }
}

.wallet {
  &__not-found {
    text-align: center;
    margin-top: calc(var(--ds-bdu) * 10);

    &__title {
      color: rgb(var(--ds-primary-variant));
    }
  }

  &__tab {
    &__item {
      background: rgb(var(--ds-surface));
    }

    &__item.v-slide-group-item--active {
      font-weight: 600;
    }
  }

  &__header {
    position: relative;

    &__details {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      height: 100%;

      &__addr {
        white-space: nowrap;
        align-items: center;
        display: flex;
        flex-wrap: nowrap;

        &__text {
          overflow: hidden;
          text-overflow: ellipsis;
          max-width: calc(100% - calc(var(--ds-bdu) * 4));
        }
      }
    }
  }

  &__balance {
    font-size: var(--ds-font-size-md);
    position: absolute;
    display: flex;
    z-index: 2;
    background: rgb(var(--ds-surface));
    width: auto;
    min-width: calc(var(--ds-bdu) * 20);
    border-radius: 10px;
    justify-content: center;
    align-items: center;
    padding: calc(var(--ds-bdu) * 2) calc(var(--ds-bdu) * 2);
    height: calc(var(--ds-bdu) * 6);
    right: calc(var(--ds-bdu) * 4);
    bottom: 0;
  }

  .transfers {
    &__item {
      &__details {
        &--created_at {
          white-space: nowrap;
        }

        &--amount {
          white-space: nowrap;
        }
      }

      &__icon {
        min-width: calc(var(--ds-bdu) * 3);
        width: calc(var(--ds-bdu) * 3);
        max-width: calc(var(--ds-bdu) * 3);
      }
    }
  }
}
</style>
