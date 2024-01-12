<template>
  <PageLayout hide-sidebar>
    <template #main-header>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow>
          <VCol cols="12" class="text-center">
            <VProgressCircular class="my-16" color="primary" indeterminate size="90" width="8" />
            <header class="text-h4">
              {{ $t(`pages.initialization.status_${status}`) }}
            </header>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { logger, wait } from '~/core';
import PageLayout from '~/ui/components/PageLayout.vue';
import { defaultHomeRoute, router, services } from '~/ui/modules';
import { useSessionStore } from '~/ui/stores/session';
import { createUserInitialAccount, useWalletStore } from '~/ui/stores/wallet';

enum InitializationStatus {
  Starting = 'starting',
  Deploying = 'deploying',
  WaitingForCanisterInitialization = 'waiting_for_canister_initialization',
  CreatingInitialAccount = 'creating_initial_account',
  Completed = 'completed',
  Failed = 'failed',
}

const session = useSessionStore();
const wallet = useWalletStore();
const status = ref<InitializationStatus>(InitializationStatus.Starting);

const deployInitialWallet = async (): Promise<void> => {
  try {
    const controlPanelService = services().controlPanel;

    status.value = InitializationStatus.Deploying;
    const walletId = await controlPanelService.deployWallet();
    const controlPanelUser = await controlPanelService.getCurrentUser();

    // wait for the wallet to be initialized, this requires one round of consensus
    status.value = InitializationStatus.WaitingForCanisterInitialization;
    await wait(6000);

    session.populateUser(controlPanelUser);

    await session.connectWallet(walletId);

    if (wallet.user) {
      status.value = InitializationStatus.CreatingInitialAccount;
      await createUserInitialAccount(wallet.user.id);
    }

    status.value = InitializationStatus.Completed;

    await wait(2000);

    router.push({ name: defaultHomeRoute });
  } catch (err) {
    logger.error('Failed initialization', { err });
    status.value = InitializationStatus.Failed;
  }
};

onMounted(async () => {
  await deployInitialWallet();
});
</script>
