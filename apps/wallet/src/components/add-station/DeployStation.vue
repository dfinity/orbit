<template>
  <div v-if="waitlistStatus !== WaitlistStatus.Approved" data-test-id="deploy-station-screen">
    <VBtn variant="flat" :disabled="working" data-test-id="back-button" @click="emit('back')">
      <VIcon :icon="mdiChevronLeft" size="x-large"></VIcon>
      {{ $t('terms.back') }}</VBtn
    >

    <div v-if="waitlistStatus === WaitlistStatus.CheckPermissions" class="text-center mt-12">
      <h2 class="text-h4">
        {{ $t('pages.add_station.check_permissions_title') }}
      </h2>
      <VProgressCircular class="mt-10" color="primary" indeterminate size="90" width="8" />
    </div>

    <VForm
      v-else-if="waitlistStatus === WaitlistStatus.Join"
      ref="form"
      class="mt-12"
      data-test-id="join-waitlist-form"
      @submit.prevent="joinWaitlist"
    >
      <h2 class="mb-6 text-h4">
        {{ $t('pages.add_station.join_waitlist_title') }}
      </h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.join_waitlist_body') }}
      </p>

      <VTextField
        v-model="email"
        type="email"
        :rules="[requiredRule]"
        :label="$t('pages.add_station.join_waitlist_email_field')"
        :variant="'outlined'"
        hide-details="auto"
        :disabled="working"
        data-test-id="join-waitlist-form-email"
      />
      <div class="d-flex align-center ga-4 mt-6">
        <VBtn
          color="primary"
          class=""
          type="submit"
          :loading="working"
          :disabled="working || !isFormValid"
        >
          {{ $t('pages.add_station.join_waitlist') }}
        </VBtn>
      </div>
    </VForm>

    <div
      v-else-if="waitlistStatus === WaitlistStatus.Pending"
      class="mt-12"
      data-test-id="join-waitlist-pending"
    >
      <h2 class="mb-6 text-h4">
        {{ $t('pages.add_station.waitlist_pending_title') }}
      </h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.waitlist_pending_body') }}
      </p>
    </div>
    <div
      v-else-if="waitlistStatus === WaitlistStatus.Denied"
      class="mt-12"
      data-test-id="join-waitlist-denied"
    >
      <h2 class="mb-6 text-h4">
        {{ $t('pages.add_station.waitlist_denied_title') }}
      </h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.waitlist_denied_body') }}
      </p>
    </div>

    <div
      v-else-if="waitlistStatus === WaitlistStatus.CheckError"
      class="mt-12"
      data-test-id="join-waitlist-check-error"
    >
      <h2 class="mb-6 text-h4">
        {{ $t('pages.add_station.waitlist_check_error_title') }}
      </h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.waitlist_check_error_body') }}
      </p>
    </div>

    <template v-else>{{ unreachable(waitlistStatus) }}</template>
  </div>

  <VContainer v-else class="pl-8 pr-8 mt-12" fluid data-test-id="deploying-station">
    <VRow>
      <VCol cols="12" class="text-center">
        <VProgressCircular class="my-16" color="primary" indeterminate size="90" width="8" />
        <header class="text-h4">
          {{ $t(`pages.add_station.status_${deploymentStatus}`) }}
        </header>
      </VCol>
    </VRow>
  </VContainer>
</template>

<script setup lang="ts">
import { Principal } from '@dfinity/principal';
import { mdiChevronLeft } from '@mdi/js';
import { computed } from 'vue';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { VBtn, VForm, VIcon, VTextField } from 'vuetify/components';
import { defaultHomeRoute } from '~/configs/routes.config';
import { logger } from '~/core/logger.core';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { createUserInitialAccount, useStationStore } from '~/stores/station.store';
import { VFormValidation } from '~/types/helper.types';
import { requiredRule } from '~/utils/form.utils';
import { unreachable, variantIs, wait } from '~/utils/helper.utils';

enum WaitlistStatus {
  CheckPermissions = 'check_permissions',
  CheckError = 'error',
  Join = 'join_waitlist',
  Pending = 'waitlist_pending',
  Denied = 'waitlist_denied',
  Approved = 'approved',
}
const waitlistStatus = ref<WaitlistStatus>(WaitlistStatus.CheckPermissions);

enum DeployStationStatus {
  Idle = 'idle',
  Starting = 'starting',
  Deploying = 'deploying',
  WaitingForCanisterInitialization = 'waiting_for_canister_initialization',
  CreatingInitialAccount = 'creating_initial_account',
  Completed = 'completed',
  Failed = 'failed',
}
const deploymentStatus = ref<DeployStationStatus>(DeployStationStatus.Idle);

const emit = defineEmits<{
  (event: 'back', payload: void): void;
}>();

const router = useRouter();
const session = useSessionStore();
const station = useStationStore();
const app = useAppStore();
const controlPanelService = services().controlPanel;

const email = ref('');
const working = ref(false);
const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const waitUntilStationIsInitialized = async (
  stationId: Principal,
  { retries, retryWaitMs }: { retries?: number; retryWaitMs?: number } = {},
): Promise<void> => {
  const stationService = services().station;
  let maxRetries = retries ?? 30;
  const waitBetweenTriesMs = retryWaitMs ?? 1000;

  while (maxRetries > 0) {
    if (
      await stationService
        .withStationId(stationId)
        .isHealthy()
        .catch(e => {
          logger.error(`Failed to check station health, due to ${e}`);

          return false;
        })
    ) {
      return;
    }

    await wait(waitBetweenTriesMs);
    --maxRetries;
  }

  throw new Error('Station did not initialize in time');
};

const deployInitialStation = async (): Promise<void> => {
  try {
    deploymentStatus.value = DeployStationStatus.Deploying;
    const stationId = await controlPanelService.deployStation();
    const controlPanelUser = await controlPanelService.getCurrentUser();

    // wait for the station to be initialized, this requires one round of consensus
    deploymentStatus.value = DeployStationStatus.WaitingForCanisterInitialization;

    await waitUntilStationIsInitialized(stationId);

    session.populateUser(controlPanelUser);

    await session.connectStation(stationId, false);

    if (station.user) {
      deploymentStatus.value = DeployStationStatus.CreatingInitialAccount;
      await createUserInitialAccount(station.user.id);
    }

    deploymentStatus.value = DeployStationStatus.Completed;

    // this wait is here to make sure the user has a chance to see the completed status
    await wait(2000);

    router.push({ name: defaultHomeRoute, query: { stationId: stationId.toText() } });
  } catch (err) {
    logger.error('Failed initialization', { err });
    deploymentStatus.value = DeployStationStatus.Failed;
  }
};

async function joinWaitlist() {
  try {
    working.value = true;
    await controlPanelService.subscribeToWaitlist(email.value);
    waitlistStatus.value = WaitlistStatus.Pending;
    working.value = false;
  } catch (e: unknown) {
    app.sendErrorNotification(e);
    working.value = false;
  }
}

onMounted(async () => {
  try {
    const user = await controlPanelService.getCurrentUser();

    if (variantIs(user.subscription_status, 'Approved')) {
      deploymentStatus.value = DeployStationStatus.Starting;
      waitlistStatus.value = WaitlistStatus.Approved;
      await deployInitialStation();
    } else if (variantIs(user.subscription_status, 'Denylisted')) {
      waitlistStatus.value = WaitlistStatus.Denied;
    } else if (variantIs(user.subscription_status, 'Pending')) {
      waitlistStatus.value = WaitlistStatus.Pending;
    } else if (variantIs(user.subscription_status, 'Unsubscribed')) {
      waitlistStatus.value = WaitlistStatus.Join;
    } else {
      unreachable(user.subscription_status);
    }
  } catch (e: unknown) {
    app.sendErrorNotification(e);

    waitlistStatus.value = WaitlistStatus.CheckError;
  }
});
</script>