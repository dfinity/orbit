<template>
  <VSelect
    :key="allStations.length"
    v-model="selectedStation"
    :loading="session.loading"
    class="station-selector"
    eager
    :variant="app.isMobile ? 'filled' : 'solo'"
    density="default"
    hide-details
    :bg-color="props.bgColor"
    item-value="canisterId"
    :items="allStations"
  >
    <template #item="{ props: itemProps, item }">
      <VListItem
        v-bind="itemProps"
        :title="computedStationName({ canisterId: Principal.fromText(item.raw.canisterId) })"
        :subtitle="item.raw.canisterId"
      />
    </template>
    <template #selection="{ item }">
      <VListItem
        v-if="allStations.length"
        :title="computedStationName({ canisterId: Principal.fromText(item.raw.canisterId) })"
        :prepend-icon="mdiWallet"
      />
      <VListItem v-else :title="noneSelectedText" :prepend-icon="mdiWallet" />
    </template>

    <template #append-item>
      <AddStationListItem />
    </template>
  </VSelect>
</template>
<script lang="ts" setup>
import { Principal } from '@icp-sdk/core/principal';
import { mdiWallet } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { computedStationName } from '~/utils/app.utils';
import AddStationListItem from './add-station/AddStationListItem.vue';
import { VListItem, VSelect } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    bgColor?: string;
  }>(),
  {
    bgColor: 'surface',
  },
);

const session = useSessionStore();
const app = useAppStore();
const i18n = useI18n();

const allStations = computed(() => session.data.stations);

const noneSelectedText = computed(() => i18n.t('stations.no_stations'));

const selectedStation = computed({
  get(): string | null {
    return session.data.selected.canisterId ? session.data.selected.canisterId : null;
  },
  set(newStationId: string | null) {
    if (!newStationId) {
      session.disconnectStation();
      return;
    }

    session.connectStation(Principal.fromText(newStationId));
  },
});
</script>

<style lang="scss">
.station-selector {
  .v-field__input {
    padding-top: calc(var(--ds-bdu) / 2);
    padding-bottom: calc(var(--ds-bdu) / 2);
  }

  .v-select__selection {
    .v-list-item__prepend {
      .v-list-item__spacer {
        width: calc(var(--ds-bdu) * 2);
      }
    }
    > .v-list-item {
      padding-left: 0;
    }

    .v-list-item__content {
      text-overflow: ellipsis;
    }
  }
}
</style>
