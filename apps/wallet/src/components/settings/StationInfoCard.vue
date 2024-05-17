<template>
  <VCard>
    <VCardTitle data-test-id="user-selected-station-name">
      {{ $t(`app.station_info_card_title`, { name: station.name }) }}
    </VCardTitle>
    <VCardText class="pb-0">
      <VList lines="two" class="bg-transparent">
        <VListItem v-if="station.canisterId" class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.station_id`) }}</VListItemTitle>
          <VListItemSubtitle>
            <span>
              {{ station.canisterId }}
              <VBtn
                size="x-small"
                variant="text"
                :icon="mdiContentCopy"
                @click="
                  copyToClipboard({
                    textToCopy: station.canisterId,
                    sendNotification: true,
                  })
                "
              />
            </span>
          </VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0" :lines="hasOverridenStationName ? 'two' : 'one'">
          <VListItemTitle class="font-weight-bold">
            {{ $t(`terms.station_name`) }}
            <AuthCheck :privileges="[Privilege.ManageSystemInfo]">
              <ActionBtn
                v-model="manageSystemInfoInput"
                :icon="mdiPencil"
                :title="$t(`requests.types.managesysteminfo.title`)"
                color="primary"
                :submit="submitManageSystemInfoOperation"
                size="x-small"
                variant="text"
                data-test-id="manage-system-info-btn"
                @failed="useOnFailedOperation"
                @submitted="useOnSuccessfulOperation"
              >
                <template #default="{ model: elem, submit }">
                  <ManageSystemInfoForm
                    v-model="elem.value.model"
                    @valid="isValid => (elem.value.valid = isValid)"
                    @submit="submit"
                  />
                </template>
                <template #actions="{ submit, loading: saving, model: elem }">
                  <VSpacer />
                  <VBtn
                    :loading="saving"
                    :disabled="!elem.value.valid"
                    color="primary"
                    variant="flat"
                    @click="submit"
                  >
                    {{ $t('terms.save') }}
                  </VBtn>
                </template>
              </ActionBtn>
            </AuthCheck>
          </VListItemTitle>
          <VListItemSubtitle>
            <span data-test-id="station-name">{{ station.configuration.details.name }}</span>
            <VChip
              v-if="hasOverridenStationName"
              size="small"
              class="ml-1"
              color="success"
              density="comfortable"
            >
              {{ $t(`terms.overriden`) }}
            </VChip>
          </VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.version`) }}</VListItemTitle>
          <VListItemSubtitle>{{
            station.configuration.details?.version ? station.configuration.details.version : '-'
          }}</VListItemSubtitle>
        </VListItem>
        <VListItem v-if="session.data.stations.length > 1" class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.main`) }}</VListItemTitle>
          <VListItemSubtitle>{{
            isMainStation ? $t(`terms.yes`) : $t(`terms.no`)
          }}</VListItemSubtitle>
        </VListItem>
      </VList>
    </VCardText>
    <VCardActions class="px-4 pb-4">
      <ActionBtn
        v-model="stationConfigInput"
        :text="$t(`app.station_info_card_edit_btn`)"
        :title="$t(`app.station_info_card_edit_btn`)"
        color="primary"
        :submit="save"
        size="small"
        variant="elevated"
        data-test-id="update-station-details-btn"
        @failed="onFailedOperation"
        @submitted="onSuccessfulOperation"
      >
        <template #default="{ model: elem, submit }">
          <VAlert type="info" density="compact" variant="tonal" class="mb-4">
            {{ $t('app.station_info_card_edit_hint') }}
          </VAlert>
          <StationInfoForm
            v-model="elem.value.model"
            @valid="isValid => (elem.value.valid = isValid)"
            @submit="submit"
          />
        </template>
        <template #actions="{ submit, loading: saving, model: elem }">
          <VSpacer />
          <VBtn
            :loading="saving"
            :disabled="!elem.value.valid"
            color="primary"
            variant="flat"
            @click="submit"
          >
            {{ $t('terms.save') }}
          </VBtn>
        </template>
      </ActionBtn>
      <ActionBtn
        data-test-id="remove-station-btn"
        :text="$t(`app.station_info_card_remove_btn`)"
        :title="$t(`app.station_info_card_remove_btn`)"
        :content="$t(`app.station_info_card_remove_btn_confirm`)"
        variant="text"
        :submit="removeStation"
      >
      </ActionBtn>
    </VCardActions>
  </VCard>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiContentCopy, mdiPencil } from '@mdi/js';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VCardTitle,
  VList,
  VListItem,
  VListItemSubtitle,
  VListItemTitle,
  VSpacer,
} from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import ManageSystemInfoForm from '~/components/settings/ManageSystemInfoForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { defaultHomeRoute } from '~/configs/routes.config';
import { ManageSystemInfoOperationInput, Request } from '~/generated/station/station.did';
import { storeUserStationToUserStation } from '~/mappers/stations.mapper';
import { i18n } from '~/plugins/i18n.plugin';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { useStationStore } from '~/stores/station.store';
import { Privilege } from '~/types/auth.types';
import { copyToClipboard } from '~/utils/app.utils';
import StationInfoForm, { StationInfoModel } from './StationInfoForm.vue';

const station = useStationStore();
const session = useSessionStore();
const app = useAppStore();
const router = useRouter();
const isMainStation = computed(() => station.canisterId === session.mainStation?.toText());
const controlPanelService = services().controlPanel;

async function removeStation(): Promise<void> {
  await services().controlPanel.manageUserStations({
    Remove: session.data.stations
      .filter(w => w.canisterId === station.canisterId)
      .map(w => storeUserStationToUserStation(w).canister_id),
  });

  await session.refreshUserStationsList();

  let maybeStationToRedirect = session.mainStation;
  if (!maybeStationToRedirect && session.data.stations[0]?.canisterId) {
    maybeStationToRedirect = Principal.fromText(session.data.stations[0].canisterId);
  }

  if (maybeStationToRedirect) {
    await session.connectStation(maybeStationToRedirect);
  } else {
    session.disconnectStation();
  }

  router.push({ name: defaultHomeRoute });
}

const onFailedOperation = (): void => {
  app.sendNotification({
    type: 'error',
    message: i18n.global.t('app.request_failed_message'),
  });

  stationConfigInput.value = initialCreateInput();
};

const onSuccessfulOperation = (): void => {
  app.sendNotification({
    type: 'success',
    message: i18n.global.t('app.request_completed_message'),
  });

  stationConfigInput.value = initialCreateInput();
};

const save = async ({ model }: { valid: boolean; model: StationInfoModel }): Promise<void> => {
  const maybeStation = session.data.stations.find(w => w.canisterId === station.canisterId);
  if (!maybeStation) {
    return;
  }

  const updatedStation = storeUserStationToUserStation(maybeStation);
  updatedStation.name = model.name;

  await controlPanelService.manageUserStations({
    Update: [
      {
        index: model.main ? [BigInt(0)] : [],
        station: updatedStation,
      },
    ],
  });

  await session.refreshUserStationsList();
};

const hasOverridenStationName = computed(
  () =>
    station.configuration.details.name !==
    session.data.stations.find(w => w.canisterId === station.canisterId)?.name,
);

const initialCreateInput = (): {
  valid: boolean;
  model: StationInfoModel;
} => ({
  valid: false,
  model: {
    name: station.name,
    main: session.mainStation?.toText() === station.canisterId,
  },
});

const stationConfigInput = ref<{
  valid: boolean;
  model: StationInfoModel;
}>(initialCreateInput());

const manageSystemInfoInput = ref<{
  valid: boolean;
  model: ManageSystemInfoOperationInput;
}>({
  valid: false,
  model: { name: [station.configuration.details.name] },
});

const submitManageSystemInfoOperation = async ({
  model,
}: {
  valid: boolean;
  model: ManageSystemInfoOperationInput;
}): Promise<Request> => {
  return station.service.createManageSystemInfoRequest(model);
};
</script>
