<template>
  <div v-if="canDeployStatus !== CanDeployStatus.Approved" data-test-id="deploy-station-screen">
    <VBtn variant="flat" :disabled="working" data-test-id="back-button" @click="emit('back')">
      <VIcon :icon="mdiChevronLeft" size="x-large"></VIcon>
      {{ $t('terms.back') }}</VBtn
    >

    <div v-if="canDeployStatus === CanDeployStatus.CheckPermissions" class="text-center mt-12">
      <h2 class="text-h4">
        {{ $t('pages.add_station.check_permissions_title') }}
      </h2>
      <VProgressCircular class="mt-10" color="primary" indeterminate size="90" width="8" />
    </div>

    <VForm
      v-else-if="canDeployStatus === CanDeployStatus.PickName"
      ref="stationForm"
      class="mt-12"
      data-test-id="deploy-station-form"
      @submit.prevent="stationFormSubmit"
    >
      <h2 class="mb-6 text-h4">
        {{ $t('pages.add_station.station_title') }}
      </h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.station_body') }}
      </p>

      <VTextField
        v-model.trim="stationName"
        type="text"
        name="station_name"
        :rules="[requiredRule, maxLengthRule(40, $t('pages.add_station.station_name_field'))]"
        :label="$t('pages.add_station.station_name_field')"
        variant="filled"
        hide-details="auto"
        :disabled="working"
        data-test-id="deploy-station-form-name-field"
      />

      <VTextField
        v-model.trim="adminName"
        type="text"
        name="admin_name"
        class="mt-4"
        :rules="[requiredRule, maxLengthRule(50, $t('pages.add_station.admin_name_field'))]"
        :label="$t('pages.add_station.admin_name_field')"
        variant="filled"
        hide-details="auto"
        :disabled="working"
        data-test-id="deploy-station-form-admin-name-field"
      />

      <div class="d-flex align-center ga-4 mt-6">
        <VBtn
          color="primary"
          type="submit"
          data-test-id="deploy-station-form-continue-button"
          :loading="working"
          :disabled="working || !isStationFormValid"
        >
          {{ $t('terms.create') }}
        </VBtn>
      </div>
    </VForm>

    <div
      v-else-if="canDeployStatus === CanDeployStatus.NotAllowed"
      class="mt-12"
      data-test-id="deploy-not-allowed"
    >
      <h2 class="mb-6 text-h4">
        {{ $t('pages.add_station.waitlist_denied_title') }}
      </h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.waitlist_denied_body') }}
      </p>
    </div>

    <div
      v-else-if="canDeployStatus === CanDeployStatus.CheckError"
      class="mt-12"
      data-test-id="deploy-check-error"
    >
      <h2 class="mb-6 text-h4">
        {{ $t('pages.add_station.deployment_check_error_title') }}
      </h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.deployment_check_error_body') }}
      </p>
    </div>

    <div
      v-else-if="canDeployStatus === CanDeployStatus.QuotaExceeded"
      class="mt-12"
      data-test-id="deploy-quota-exceeded-error"
    >
      <h2 class="mb-6 text-h4">
        {{ $t('pages.add_station.quota_exceed_error_title') }}
      </h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.quota_exceed_error_body') }}
      </p>
    </div>

    <template v-else>{{ unreachable(canDeployStatus) }}</template>
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
import { maxLengthRule } from '~/utils/form.utils';
import { requiredRule } from '~/utils/form.utils';
import { unreachable, variantIs, wait } from '~/utils/helper.utils';
import { CONTROL_PANEL_USER_STATION_LABEL } from '~/core/constants.core';

enum CanDeployStatus {
  CheckPermissions = 'check_permissions',
  CheckError = 'error',
  NotAllowed = 'not_allowed',
  Approved = 'approved',
  QuotaExceeded = 'quota_exceeded',
  PickName = 'pick_name',
}
const canDeployStatus = ref<CanDeployStatus>(CanDeployStatus.CheckPermissions);

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

const working = ref(false);

const stationName = ref('');
const adminName = ref('');
const stationForm = ref<VFormValidation | null>(null);
const isStationFormValid = computed(() => (stationForm.value ? stationForm.value.isValid : false));

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
    const stationId = await controlPanelService.deployStation({
      name: stationName.value,
      admins: [
        {
          identity: Principal.fromText(session.principal),
          username: adminName.value,
        },
      ],
      associate_with_caller: [
        {
          labels: [CONTROL_PANEL_USER_STATION_LABEL],
        },
      ],
      // TODO: Make use of the subnet selection feature
      subnet_selection: [],
    });
    const [controlPanelUser, userStations] = await Promise.all([
      controlPanelService.getCurrentUser(),
      controlPanelService.listUserStations({
        filter_by_labels: [[CONTROL_PANEL_USER_STATION_LABEL]],
      }),
    ]);

    // wait for the station to be initialized, this requires one round of consensus
    deploymentStatus.value = DeployStationStatus.WaitingForCanisterInitialization;

    await waitUntilStationIsInitialized(stationId);

    session.populateUser({
      user: controlPanelUser,
      stations: userStations,
    });

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

async function stationFormSubmit() {
  deploymentStatus.value = DeployStationStatus.Starting;
  canDeployStatus.value = CanDeployStatus.Approved;

  try {
    await deployInitialStation();
  } catch (e: unknown) {
    app.sendErrorNotification(e);
    deploymentStatus.value = DeployStationStatus.Failed;
  }
}

onMounted(async () => {
  try {
    const canDeploy = await controlPanelService.canDeployStation();

    if (variantIs(canDeploy, 'NotAllowed')) {
      if (variantIs(canDeploy.NotAllowed, 'Approved')) {
        canDeployStatus.value = CanDeployStatus.PickName;
        await deployInitialStation();
      } else if (variantIs(canDeploy.NotAllowed, 'Denylisted')) {
        canDeployStatus.value = CanDeployStatus.NotAllowed;
      } else if (variantIs(canDeploy.NotAllowed, 'Pending')) {
        canDeployStatus.value = CanDeployStatus.NotAllowed;
      } else if (variantIs(canDeploy.NotAllowed, 'Unsubscribed')) {
        canDeployStatus.value = CanDeployStatus.NotAllowed;
      } else {
        unreachable(canDeploy.NotAllowed);
      }

      return;
    }

    if (variantIs(canDeploy, 'Allowed')) {
      canDeployStatus.value = CanDeployStatus.PickName;

      return;
    }

    if (variantIs(canDeploy, 'QuotaExceeded')) {
      canDeployStatus.value = CanDeployStatus.QuotaExceeded;
      return;
    }
  } catch (e: unknown) {
    app.sendErrorNotification(e);
    canDeployStatus.value = CanDeployStatus.CheckError;
  }
});
</script>
