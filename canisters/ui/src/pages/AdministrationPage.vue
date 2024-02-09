<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pa-8" fluid>
        <VRow>
          <VCol cols="12">
            <h1 class="text-h4">{{ $t(`pages.administration.title`) }}</h1>
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow>
          <AuthCheck :privileges="[Privilege.ListProposals]">
            <template #default>
              <VCol cols="12" md="8">
                <RecentProposals
                  :title="$t(`app.wallet_upgrades_card_title`)"
                  :types="[{ ChangeCanister: null }]"
                >
                  <template #top-actions>
                    <ChangeCanisterActionBtn class="mr-2" />
                    <VBtn
                      variant="tonal"
                      size="small"
                      :to="{
                        name: Routes.Proposals,
                        query: { group_by: ProposalDomains.System },
                      }"
                    >
                      {{ $t('terms.see_all') }}
                    </VBtn>
                  </template>
                </RecentProposals>
              </VCol>
              <VCol cols="12" md="4">
                <WalletInfoCard />
              </VCol>
            </template>
            <template #unauthorized>
              <VCol cols="12" md="4">
                <WalletInfoCard />
              </VCol>
            </template>
          </AuthCheck>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import AuthCheck from '~/components/AuthCheck.vue';
import PageLayout from '~/components/PageLayout.vue';
import ChangeCanisterActionBtn from '~/components/change-canister/ChangeCanisterActionBtn.vue';
import RecentProposals from '~/components/proposals/RecentProposals.vue';
import WalletInfoCard from '~/components/settings/WalletInfoCard.vue';
import { Routes } from '~/configs/routes.config';
import { Privilege } from '~/types/auth.types';
import { ProposalDomains } from '~/types/wallet.types';
</script>
