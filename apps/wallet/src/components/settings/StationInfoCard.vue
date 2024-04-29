<template>
  <VCard>
    <VCardTitle>
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
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.station_name`) }}</VListItemTitle>
          <VListItemSubtitle data-test-id="station-name">{{
            station.name ?? '-'
          }}</VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.version`) }}</VListItemTitle>
          <VListItemSubtitle>{{
            station.configuration.details?.version ? station.configuration.details.version : '-'
          }}</VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0">
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
        :disabled="!isStationRemovable"
      >
      </ActionBtn>
    </VCardActions>
  </VCard>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiContentCopy } from '@mdi/js';
import { computed, ref } from 'vue';
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
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import { UserStation } from '~/generated/control-panel/control_panel.did';
import { stationToUserStation } from '~/mappers/stations.mapper';
import { i18n } from '~/plugins/i18n.plugin';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { useStationStore } from '~/stores/station.store';
import { copyToClipboard } from '~/utils/app.utils';
import StationInfoForm, { StationInfoModel } from './StationInfoForm.vue';
import { useRouter } from 'vue-router';
import { defaultHomeRoute } from '~/configs/routes.config';

const station = useStationStore();
const session = useSessionStore();
const app = useAppStore();
const router = useRouter();
const isMainStation = computed(() => station.canisterId === session.mainStation?.toText());
const isStationRemovable = computed(() => !isMainStation.value);
const controlPanelService = services().controlPanel;

async function removeStation(): Promise<void> {
  if (!isStationRemovable.value) {
    return;
  }

  const updatedUser = await services().controlPanel.editUser({
    main_station: [], // do not change the main station
    stations: [
      session.data.stations
        .filter(w => w.canisterId !== station.canisterId)
        .map(w => stationToUserStation(w)),
    ],
  });

  session.populateUser(updatedUser);

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
  const mainStation = model.main ? Principal.fromText(station.canisterId) : session.mainStation;
  const updatedStations: UserStation[] =
    session.data.stations.map(entry => {
      if (entry.canisterId === station.canisterId) {
        return {
          name: model.name,
          canister_id: Principal.fromText(entry.canisterId),
        };
      }

      return {
        name: entry.name,
        canister_id: Principal.fromText(entry.canisterId),
      };
    }) ?? [];

  const user = await controlPanelService.editUser({
    main_station: mainStation ? [mainStation] : [],
    stations: updatedStations.length ? [updatedStations] : [],
  });

  session.populateUser(user);
};

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
</script>
