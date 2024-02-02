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
          <AuthCheck :privileges="[Privilege.ListUsers]">
            <template #default>
              <VCol cols="12" md="8">
                <AuthCheck :privileges="[Privilege.ListUsers]">
                  <RecentProposals
                    :title="$t(`app.user_activities_card_title`)"
                    :see-all-link="{
                      name: Routes.Proposals,
                      query: { group_by: ProposalDomains.Users },
                    }"
                    :types="[{ AddUser: null }, { EditUser: null }]"
                  />
                </AuthCheck>

                <RecentProposals
                  :title="$t(`app.wallet_upgrades_card_title`)"
                  :see-all-link="{
                    name: Routes.Proposals,
                    query: { group_by: ProposalDomains.System },
                  }"
                  :types="[{ ChangeCanister: null }]"
                  class="mt-4"
                />
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
import { Privilege, ProposalDomains } from '~/types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import RecentProposals from '~/ui/components/proposals/RecentProposals.vue';
import WalletInfoCard from '~/ui/components/settings/WalletInfoCard.vue';
import { Routes } from '~/ui/config/routes';
</script>
