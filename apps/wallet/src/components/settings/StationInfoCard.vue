<template>
  <VCard :loading="loadingSystemInfo">
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
          <VListItemTitle class="font-weight-bold">
            {{ $t('terms.cycle_obtain_strategy') }}
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
          <VListItemSubtitle class="d-inline">
            <span data-test-id="obtain-cycle-strategy">{{ cycleObtainStrategy }}</span>
          </VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.version`) }}</VListItemTitle>
          <VListItemSubtitle class="mt-2"
            >{{
              station.configuration.details?.version ? station.configuration.details.version : '-'
            }}
          </VListItemSubtitle>
        </VListItem>
        <AuthCheck :privileges="[Privilege.SystemInfo]">
          <tempalte v-if="!loadingSystemInfo">
            <VListItem class="px-0">
              <VListItemTitle class="font-weight-bold">{{
                $t(`terms.upgrader_id`)
              }}</VListItemTitle>
              <VListItemSubtitle v-if="upgraderId"
                >{{ upgraderId }}
                <VBtn
                  size="x-small"
                  variant="text"
                  :icon="mdiContentCopy"
                  @click="
                    copyToClipboard({
                      textToCopy: upgraderId,
                      sendNotification: true,
                    })
                  "
                />
              </VListItemSubtitle>
            </VListItem>

            <VListItem class="px-0">
              <VListItemTitle class="font-weight-bold">{{
                $t(`pages.administration.cycle_balances`)
              }}</VListItemTitle>
              <VListItemSubtitle>
                <table style="min-width: 200px">
                  <tr>
                    <td>{{ $t('terms.station') }}:</td>
                    <td class="text-right">
                      {{ systemInfo ? formatCycles(systemInfo.cycles) : '-' }}
                    </td>
                  </tr>
                  <tr>
                    <td>{{ $t('terms.upgrader') }}:</td>
                    <td class="text-right">
                      {{
                        systemInfo?.upgrader_cycles?.[0]
                          ? formatCycles(systemInfo.upgrader_cycles[0])
                          : '-'
                      }}
                    </td>
                  </tr>
                </table>
              </VListItemSubtitle>
            </VListItem>
          </tempalte>
          <VListItem class="px-0" v-else-if="loadingSystemInfoError">
            <VListItemTitle class="font-weight-bold">{{ $t(`terms.upgrader_id`) }}</VListItemTitle>
            <VListItemSubtitle>
              <VAlert type="error" variant="tonal" density="compact" class="mb-4 mt-2">
                {{ $t('pages.administration.system_info_error') }}
              </VAlert>
            </VListItemSubtitle>
          </VListItem>
        </AuthCheck>
        <VListItem v-if="session.data.stations.length > 1" class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.main`) }}</VListItemTitle>
          <VListItemSubtitle
            >{{ isMainStation ? $t(`terms.yes`) : $t(`terms.no`) }}
          </VListItemSubtitle>
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
import { computed, onMounted, ref } from 'vue';
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
import {
  CycleObtainStrategy,
  CycleObtainStrategyInput,
  ManageSystemInfoOperationInput,
  Request,
  SystemInfo,
} from '~/generated/station/station.did';
import { formatCycles } from '~/mappers/cycles.mapper';
import { storeUserStationToUserStation } from '~/mappers/stations.mapper';
import { i18n } from '~/plugins/i18n.plugin';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { useStationStore } from '~/stores/station.store';
import { Privilege } from '~/types/auth.types';
import { copyToClipboard } from '~/utils/app.utils';
import { hasRequiredPrivilege } from '~/utils/auth.utils';
import { unreachable, variantIs } from '~/utils/helper.utils';
import StationInfoForm, { StationInfoModel } from './StationInfoForm.vue';

const station = useStationStore();
const session = useSessionStore();
const app = useAppStore();
const router = useRouter();
const isMainStation = computed(() => station.canisterId === session.mainStation?.toText());
const controlPanelService = services().controlPanel;
const stationPanelService = services().station;

const loadingSystemInfo = ref(true);
const loadingSystemInfoError = ref(false);

const systemInfo = ref<SystemInfo | null>(null);

onMounted(async () => {
  if (hasRequiredPrivilege({ anyOf: [Privilege.SystemInfo] })) {
    try {
      systemInfo.value = await stationPanelService.systemInfo(true).then(result => result.system);
    } catch (e: unknown) {
      app.sendErrorNotification(e);
      loadingSystemInfoError.value = true;
    }
  }
  loadingSystemInfo.value = false;
});

const upgraderId = computed(() => systemInfo.value?.upgrader_id.toText());

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

function cycleObtainStrategyToInput(strategy: CycleObtainStrategy): CycleObtainStrategyInput {
  if (variantIs(strategy, 'MintFromNativeToken')) {
    return {
      MintFromNativeToken: {
        account_id: strategy.MintFromNativeToken.account_id,
      },
    };
  } else if (variantIs(strategy, 'Disabled')) {
    return { Disabled: null };
  } else {
    return unreachable(strategy);
  }
}

const manageSystemInfoInput = ref<{
  valid: boolean;
  model: ManageSystemInfoOperationInput;
}>({
  valid: false,
  model: {
    name: [station.configuration.details.name],
    cycle_obtain_strategy: station.configuration.cycleObtainStrategy
      ? [cycleObtainStrategyToInput(station.configuration.cycleObtainStrategy)]
      : [],
  },
});

const submitManageSystemInfoOperation = async ({
  model,
}: {
  valid: boolean;
  model: ManageSystemInfoOperationInput;
}): Promise<Request> => {
  return station.service.createManageSystemInfoRequest(model);
};

const cycleObtainStrategy = computed(() => {
  if (variantIs(station.configuration.cycleObtainStrategy, 'MintFromNativeToken')) {
    return `${i18n.global.t('pages.administration.cycle_obtain_strategy_mint_from_native_token')} "${station.configuration.cycleObtainStrategy.MintFromNativeToken.account_name}"`;
  } else if (variantIs(station.configuration.cycleObtainStrategy, 'Disabled')) {
    return i18n.global.t('pages.administration.cycle_obtain_strategy_disabled');
  } else {
    return unreachable(station.configuration.cycleObtainStrategy);
  }
});
</script>
