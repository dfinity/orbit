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
          <VCol cols="12" md="8">
            <UserActivitiesCard />
            <WalletUpgradesCard class="mt-4" />
          </VCol>
          <VCol cols="12" md="4">
            <WalletInfoCard />
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import PageLayout from '~/ui/components/PageLayout.vue';
import WalletInfoCard from '~/ui/components/wallet-info-card/WalletInfoCard.vue';
import UserActivitiesCard from '~/ui/components/user-activities-card/UserActivitiesCard.vue';
import WalletUpgradesCard from '~/ui/components/wallet-upgrades-card/WalletUpgradesCard.vue';
</script>
