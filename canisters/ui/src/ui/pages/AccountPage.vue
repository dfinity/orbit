<template>
  <PageLayout v-if="wallet.hasUser" class="account">
    <template v-if="pageStore.hasLoaded" #main-header>
      <VContainer class="pt-16 pb-16 pl-8 pr-8 account__header" fluid>
        <div class="account__balance">
          <span
            v-if="pageStore.account.balance?.[0]"
            :title="pageStore.account.balance?.[0]?.last_update_timestamp"
          >
            {{ formatBalance(pageStore.account.balance[0].balance, pageStore.account.decimals) }}
          </span>
          <span v-else>-</span>
          &nbsp;{{ pageStore.account.symbol }}
        </div>
        <VRow>
          <VCol cols="12" sm="8" class="account__header__details">
            <div>
              <h1 class="text-h4">
                <VIcon :icon="mdiWallet" size="x-small" />
                {{ pageStore.account.name }}
                <!--TODO: add access control if the user can edit the account-->
                <template v-if="true">
                  <EditAccountBtn v-model="pageStore.account" />
                </template>
              </h1>
            </div>
            <div class="account__header__details__addr">
              <small class="account__header__details__addr__text">
                {{ pageStore.account.symbol }}: {{ pageStore.account.address }}
              </small>
              <VBtn
                class="account-card__subtitle__copy"
                size="x-small"
                variant="text"
                :icon="mdiContentCopy"
                @click="copyAddressToClipboard(`${pageStore.account.address}`)"
              />
            </div>
            <div>
              <VChip
                size="x-small"
                color="primary-variant"
                variant="tonal"
                :prepend-icon="pageStore.account.owners.length > 1 ? mdiAccountGroup : mdiAccount"
              >
                {{
                  pageStore.account.owners.length > 1
                    ? $t('wallets.joint_account')
                    : $t('wallets.private_account')
                }}
              </VChip>
            </div>
          </VCol>
          <VCol cols="12" sm="4" class="header-actions">
            <VSpacer v-if="!mobile" />
            <NewTransferBtn :account-id="pageStore.account.id" />
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template v-else-if="!pageStore.loading" #main-header>
      <div class="account__not-found pb-16">
        <header class="text-h3 account__not-found__title">
          {{ $t('account_page.not_found_title') }}
        </header>
        <p class="text-h6">
          {{ $t('account_page.not_found_description') }}
        </p>
        <VBtn color="primary-variant mt-8" :append-icon="_mdiLink" :to="{ name: 'AccountList' }">
          {{ $t('account_page.not_found_btn') }}
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
                value="withdrawals"
                class="account__tab__item"
              >
                {{ $t(`terms.withdrawals`) }}
              </VTab>
              <VTab
                v-if="pageStore.chainApi"
                :loading="pageStore.deposits.loading"
                value="deposits"
                class="account__tab__item"
              >
                {{ $t(`terms.deposits`) }}
              </VTab>
              <VTab
                :loading="pageStore.transfers.loading"
                value="proposals"
                class="account__tab__item"
              >
                {{ $t(`terms.withdraw_requests`) }}
              </VTab>
            </VTabs>
            <VCardText>
              <VWindow v-model="tab">
                <VWindowItem v-if="pageStore.chainApi" value="deposits">
                  <VContainer class="py-0">
                    <VCol cols="12" class="px-0 pb-0">
                      <VBtn
                        block
                        variant="tonal"
                        color="primary-variant"
                        :prepend-icon="mdiRefresh"
                        :loading="pageStore.deposits.loading"
                        @click="pageStore.loadDeposits"
                      >
                        {{ $t(`terms.search`) }}
                      </VBtn>
                    </VCol>
                    <VCol cols="12" class="px-0 pt-1">
                      <VTable v-if="pageStore.deposits.items.length" hover>
                        <tbody>
                          <tr v-for="(transfer, _idx) in pageStore.sortedDeposits" :key="_idx">
                            <td class="transfers__item__icon"><VIcon :icon="mdiTransfer" /></td>
                            <td class="transfers__item__details">
                              <div class="transfers__item__details--amount">
                                {{
                                  `${formatBalance(transfer.amount, 8)} ${pageStore.account.symbol}`
                                }}
                              </div>
                              <div class="transfers__item__details--to">
                                <small>{{ $t(`terms.from`) }}: {{ transfer.from }}</small>
                              </div>
                              <div
                                v-if="transfer.created_at"
                                class="transfers__item__details--created_at"
                              >
                                <small>{{ transfer.created_at.toISOString() }}</small>
                              </div>
                            </td>
                          </tr>
                        </tbody>
                      </VTable>
                      <p v-else class="text-h6">{{ $t(`wallets.no_deposit_found_search`) }}</p>
                    </VCol>
                  </VContainer>
                </VWindowItem>
                <VWindowItem value="withdrawals">
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
                                      pageStore.account.symbol
                                    }`
                                  }}
                                </div>
                                <div class="transfers__item__details--to">
                                  <small>{{ $t(`terms.to`) }}: {{ transfer.to }}</small>
                                </div>
                                <div class="transfers__item__details--created_at">
                                  <small>{{ transfer.created_at }}</small>
                                </div>
                              </td>
                              <td class="transfers__item__status text-right">
                                <TransferStatusChip :status="transfer.status" />
                              </td>
                            </tr>
                          </tbody>
                        </VTable>
                        <p v-else class="text-h6">{{ $t(`wallets.no_withdrawal_found_search`) }}</p>
                      </VCol>
                    </VRow>
                  </VContainer>
                </VWindowItem>
                <VWindowItem value="proposals">
                  <VContainer>
                    <VRow>
                      <VCol cols="12" md="4" class="py-0">
                        <VTextField
                          v-model="pageStore.proposals.fromDt"
                          :prepend-inner-icon="mdiCalendar"
                          density="compact"
                          type="date"
                          :label="$t(`terms.from`)"
                          variant="solo"
                          :disabled="pageStore.proposals.loading"
                          class="mb-2"
                          hide-details
                        />
                      </VCol>
                      <VCol cols="12" md="4" class="py-0">
                        <VTextField
                          v-model="pageStore.proposals.toDt"
                          :prepend-inner-icon="mdiCalendar"
                          density="compact"
                          type="date"
                          :label="$t(`terms.until`)"
                          :disabled="pageStore.proposals.loading"
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
                          :loading="pageStore.proposals.loading"
                          @click="
                            pageStore.loadProposals(
                              pageStore.proposals.fromDt
                                ? new Date(pageStore.proposals.fromDt)
                                : undefined,
                              pageStore.proposals.toDt
                                ? new Date(pageStore.proposals.toDt)
                                : undefined,
                            )
                          "
                        >
                          {{ $t(`terms.search`) }}
                        </VBtn>
                      </VCol>
                      <VCol cols="12">
                        <VTable v-if="pageStore.proposals.items.length" hover class="proposals">
                          <tbody>
                            <tr
                              v-for="(
                                { loading, data: { id: proposalId } }, _idx
                              ) in pageStore.sortedProposals"
                              :key="_idx"
                            >
                              <td class="py-4">
                                <WalletProposal
                                  :proposal="pageStore.sortedProposals[_idx].data"
                                  :outer="false"
                                  :loading="loading"
                                  @adopted="pageStore.voteOnProposal(proposalId, { approve: true })"
                                  @rejected="
                                    pageStore.voteOnProposal(proposalId, { approve: false })
                                  "
                                />
                              </td>
                            </tr>
                          </tbody>
                        </VTable>
                        <p v-else class="text-h6">
                          {{ $t(`wallets.no_withdraw_request_found_search`) }}
                        </p>
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
            <h1 class="text-h4">{{ $t('wallets.no_wallet_user') }}</h1>
            <p class="text-subtitle">{{ $t('wallets.please_register_to_continue') }}</p>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import {
  mdiAccount,
  mdiAccountGroup,
  mdiCalendar,
  mdiContentCopy,
  mdiRefresh,
  mdiTransfer,
  mdiWallet,
} from '@mdi/js';
import { onMounted, ref, watch } from 'vue';
import { useDisplay } from 'vuetify';
import { formatBalance } from '~/core';
import NewTransferBtn from '~/ui/components/NewTransferBtn.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import EditAccountBtn from '~/ui/components/accounts/EditAccountBtn.vue';
import WalletProposal from '~/ui/components/proposals/WalletProposal.vue';
import TransferStatusChip from '~/ui/components/transfers/TransferStatusChip.vue';
import { i18n, router } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import { useAccountPageStore } from '~/ui/stores/pages/account';
import { useWalletStore } from '~/ui/stores/wallet';

const { mobile } = useDisplay();

const wallet = useWalletStore();
const app = useAppStore();
const pageStore = useAccountPageStore();

const tab = ref<'withdrawals' | 'proposals' | 'deposits'>('withdrawals');

onMounted(() => {
  pageStore.load(`${router.currentRoute.value.params.id}`);
});

watch(
  wallet.accounts,
  accounts => {
    if (!pageStore.hasLoaded) {
      return;
    }

    const updatedAccount = accounts.items.find(w => w.id === pageStore.account.id);
    if (updatedAccount && pageStore._account) {
      pageStore._account.balance = updatedAccount.balance;
    }
  },
  {
    deep: true,
  },
);

const copyAddressToClipboard = (address: string) => {
  navigator.clipboard.writeText(address);

  app.sendNotification({
    type: 'success',
    message: i18n.global.t('wallets.account_address_copied_to_clipboard'),
  });
};
</script>

<style scoped lang="scss">
.header-actions {
  display: flex;
  justify-content: end;
  align-items: center;
  gap: calc(var(--ds-bdu) * 2);

  :deep(.v-btn) {
    flex-grow: 1;
  }
}

.page-layout--mobile {
  .header-actions {
    justify-content: center;
  }
}

.account {
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
