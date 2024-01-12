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
                  <UserActivitiesCard />
                </AuthCheck>

                <WalletUpgradesCard class="mt-4" />
              </VCol>
              <VCol cols="12" md="4">
                <WalletInfo />
              </VCol>
            </template>
            <template #unauthorized>
              <VCol cols="12" md="4">
                <WalletInfo />
              </VCol>
            </template>
          </AuthCheck>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { Privilege } from '~/types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import UserActivitiesCard from '~/ui/components/user-activities-card/UserActivitiesCard.vue';
import WalletInfo from '~/ui/components/wallet-info/WalletInfo.vue';
import WalletUpgradesCard from '~/ui/components/wallet-upgrades-card/WalletUpgradesCard.vue';
</script>
