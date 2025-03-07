<template>
  <VBtn
    v-bind="$attrs"
    :color="props.color"
    :density="props.density"
    :variant="props.variant"
    :size="props.size"
    @click="open = true"
  >
    <slot name="default">
      <slot name="prepend-icon">
        <VIcon v-if="props.prependIcon" :size="props.size" :icon="props.prependIcon" />
      </slot>
      <slot name="text">
        {{ btnText }}
      </slot>
      <slot name="append-icon">
        <VIcon v-if="props.appendIcon" :size="props.size" :icon="props.appendIcon" />
      </slot>
    </slot>

    <VDialog v-model="open" :max-width="props.dialogMaxWidth" :persistent="loading">
      <VCard :loading="loading" :persistent="loading">
        <VToolbar color="background">
          <VToolbarTitle>{{ dialogTitle }}</VToolbarTitle>
          <VBtn :disabled="loading" :icon="mdiClose" @click="open = false" />
        </VToolbar>
        <VCardText class="px-4 pb-4">
          <VCard
            v-for="(download, idx) of downloads"
            :key="idx"
            color="background"
            density="compact"
            variant="flat"
            class="mb-2"
          >
            <VToolbar color="background" density="compact">
              <VToolbarTitle class="text-body-1">
                {{ $t(`requests.download.${download.group.toLowerCase()}`) }}
              </VToolbarTitle>
              <VBtn
                :loading="download.downloading"
                variant="text"
                size="small"
                @click="startDownload(idx)"
              >
                <VIcon :icon="mdiDownload" class="mr-1" />
                {{ $t('terms.download') }}
              </VBtn>
            </VToolbar>
          </VCard>
          <div v-if="!downloads.length" class="d-flex justify-start align-center text-body-2">
            {{ $t('app.no_download_available') }}
          </div>
        </VCardText>
      </VCard>
    </VDialog>
  </VBtn>
</template>
<script lang="ts" setup>
import { mdiClose, mdiDownload } from '@mdi/js';
import { computed, ref, toRef, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCard,
  VCardText,
  VDialog,
  VIcon,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import { AvailableDomain, Filters, useDownloadItems } from '~/composables/request.composable';
import logger from '~/core/logger.core';
import { RequestStatusCode, UUID } from '~/generated/station/station.did';
import { mapRequestsToCsvTable } from '~/mappers/requests.mapper';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import type { RequestWithDetails } from '~/types/requests.types';
import { convertDate } from '~/utils/date.utils';
import { downloadCsv } from '~/utils/file.utils';

const props = withDefaults(
  defineProps<{
    filters: Filters;
    domains: AvailableDomain[];
    batchFetchLimit?: number;
    icon?: string;
    text?: string;
    color?: string;
    density?: 'comfortable' | 'compact' | 'default';
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined' | 'elevated';
    prependIcon?: string;
    appendIcon?: string;
    dialogMaxWidth?: number;
  }>(),
  {
    batchFetchLimit: 50,
    density: 'default',
    color: 'primary',
    size: 'default',
    variant: 'elevated',
    icon: undefined,
    prependIcon: undefined,
    appendIcon: undefined,
    text: undefined,
    dialogMaxWidth: 600,
  },
);

const filters = toRef(props, 'filters');
const domains = toRef(props, 'domains');
const i18n = useI18n();
const btnText = computed(() => (props.text || props.icon ? '' : i18n.t('app.export_csv')));
const dialogTitle = computed(() => btnText.value ?? i18n.t('app.export_csv'));
const open = ref(false);
const loading = ref(false);
const downloads = useDownloadItems(filters, domains);
const app = useAppStore();
const station = useStationStore();

watch(
  () => downloads.value,
  downloads => {
    loading.value = downloads.some(download => download.downloading);
  },
  { deep: true },
);

const startDownload = async (idx: number): Promise<void> => {
  const downloadItem = downloads.value[idx];
  try {
    if (downloadItem.downloading) {
      return;
    }
    downloadItem.downloading = true;
    const limit = props.batchFetchLimit;
    const requests = new Map<UUID, RequestWithDetails>();
    let offset: number | undefined = undefined;

    do {
      const result = await station.service.listRequests({
        types: downloadItem.filterBy.types,
        created_dt: {
          fromDt: convertDate(downloadItem.filterBy.created.from, {
            time: 'start-of-day',
            tz: 'local',
          }),
          toDt: convertDate(downloadItem.filterBy.created.to, {
            time: 'end-of-day',
            tz: 'local',
          }),
        },
        expiration_dt: {
          fromDt: convertDate(downloadItem.filterBy.expires.from, {
            time: 'start-of-day',
            tz: 'local',
          }),
          toDt: convertDate(downloadItem.filterBy.expires.to, {
            time: 'end-of-day',
            tz: 'local',
          }),
        },
        statuses: downloadItem.filterBy.statuses.map(status => ({
          [status]: null,
        })) as RequestStatusCode[],
        limit,
        offset,
        sortBy: {
          createdAt: 'desc',
        },
      });

      result.requests.forEach(request => {
        requests.set(request.id, {
          request,
          additionalInfo: result.additional_info.find(info => info.id === request.id),
        });
      });

      offset = result.next_offset?.[0] !== undefined ? Number(result.next_offset[0]) : undefined;
    } while (offset !== undefined);

    const requestList = Array.from(requests.values());

    const csv = mapRequestsToCsvTable(
      downloadItem.group,
      requestList,
      window.location.origin,
      station.canisterId,
    );
    const fileName =
      i18n.t(`requests.download.${downloadItem.group.toLowerCase()}`) +
      '_' +
      new Date().toISOString() +
      '.csv';

    await downloadCsv({
      content: csv,
      filename: fileName.replace(/ /g, '_').toLowerCase(),
    });
  } catch (e) {
    logger.error(`Failed to download '${downloadItem.group}', reason: ${e}`);

    app.sendNotification({
      message: i18n.t('app.failed_to_download_item', {
        item: i18n.t(`requests.download.${downloadItem.group.toLowerCase()}`).toLowerCase(),
      }),
      type: 'error',
    });
  } finally {
    downloadItem.downloading = false;
  }
};
</script>
