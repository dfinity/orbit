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
import { Principal } from '@dfinity/principal';
import { onMounted, ref } from 'vue';
import PageLayout from '~/components/PageLayout.vue';
import { defaultHomeRoute } from '~/configs/routes.config';
import { logger } from '~/core/logger.core';
import { router } from '~/plugins/router.plugin';
import { services } from '~/plugins/services.plugin';
import { useSessionStore } from '~/stores/session.store';
import { createUserInitialAccount, useWalletStore } from '~/stores/wallet.store';
import { wait } from '~/utils/helper.utils';

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

const waitUntilWalletIsInitialized = async (
  walletId: Principal,
  { retries, retryWaitMs }: { retries?: number; retryWaitMs?: number } = {},
): Promise<void> => {
  const walletService = services().wallet;
  let maxRetries = retries ?? 30;
  const waitBetweenTriesMs = retryWaitMs ?? 1000;

  while (maxRetries > 0) {
    if (
      await walletService
        .withWalletId(walletId)
        .isHealthy()
        .catch(e => {
          logger.error(`Failed to check wallet health, due to ${e}`);

          return false;
        })
    ) {
      return;
    }

    await wait(waitBetweenTriesMs);
    --maxRetries;
  }

  throw new Error('Wallet did not initialize in time');
};

const deployInitialWallet = async (): Promise<void> => {
  try {
    const controlPanelService = services().controlPanel;

    status.value = InitializationStatus.Deploying;
    const walletId = await controlPanelService.deployWallet();
    const controlPanelUser = await controlPanelService.getCurrentUser();

    // wait for the wallet to be initialized, this requires one round of consensus
    status.value = InitializationStatus.WaitingForCanisterInitialization;

    await waitUntilWalletIsInitialized(walletId);

    session.populateUser(controlPanelUser);

    await session.connectWallet(walletId);

    if (wallet.user) {
      status.value = InitializationStatus.CreatingInitialAccount;
      await createUserInitialAccount(wallet.user.id);
    }

    status.value = InitializationStatus.Completed;

    // this wait is here to make sure the user has a chance to see the completed status
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
