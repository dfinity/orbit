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
          <VToolbarTitle>
            <p class="mt-3">{{ dialogTitle }}</p>
            <p class="text-body-2">
              {{ $t('pages.account.csv_transfer_subtitle') }}
            </p>
          </VToolbarTitle>
          <VBtn :disabled="loading" :icon="mdiClose" @click="open = false" />
        </VToolbar>
        <VCardText class="px-4 pb-4">
          <VFileInput
            v-model="transfersCsv"
            name="transfers_file"
            :label="$t('terms.transfers')"
            :rules="[requiredRule]"
            :prepend-icon="mdiTable"
            accept=".csv, text/csv"
            variant="underlined"
            :persistent-hint="true"
            :hint="csvHint"
          />

          <template v-if="rows.length">
            <div class="d-flex align-center text-body-2 mt-4 mb-2">
              {{ $t('pages.account.csv_transfer_file_rows_title', { count: rows.length }) }}
            </div>
            <VTable density="compact" class="text-body-2 mvh-50" fixed-header fixed-footer>
              <thead>
                <tr>
                  <th class="text-center bg-background">#</th>
                  <th class="bg-background">{{ $t('terms.to') }}</th>
                  <th v-if="hasAnyComments" class="bg-background">{{ $t('terms.comment') }}</th>
                  <th class="text-right bg-background">{{ $t('terms.amount') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="({ transfer, requesting, requested, failed }, idx) of rows" :key="idx">
                  <td class="text-center">
                    <VIcon v-if="failed" :icon="mdiClose" color="error" />
                    <VProgressCircular
                      v-else-if="requesting"
                      indeterminate
                      color="default"
                      size="18"
                    />
                    <VIcon v-else-if="requested" :icon="mdiCashFast" color="success" />
                    <small v-else>{{ idx + 1 }}</small>
                  </td>
                  <td class="w-75">
                    <span v-if="transfer.to">
                      {{ transfer.to }}
                    </span>
                    <template v-else>
                      <VIcon :icon="mdiAlertCircle" color="error" class="mr-1" />
                      {{ $t('terms.invalid') }}
                    </template>
                  </td>
                  <td v-if="hasAnyComments" class="w-75">
                    <span v-if="transfer.comment">{{ transfer.comment }}</span>
                  </td>
                  <td class="text-right text-no-wrap">
                    <span v-if="transfer.amount">
                      {{ formatBalance(transfer.amount, asset.decimals) }}
                    </span>
                    <template v-else>
                      <VIcon :icon="mdiAlertCircle" color="error" class="mr-1" />
                      {{ $t('terms.invalid') }}
                    </template>
                  </td>
                </tr>
              </tbody>
              <tfoot>
                <tr>
                  <td colspan="4" class="text-right bg-background">
                    <span class="font-weight-bold">{{ $t('terms.total') }}:</span>
                    {{ formatBalance(totalAmount, asset.decimals) }}
                  </td>
                </tr>
              </tfoot>
            </VTable>
          </template>
        </VCardText>
        <VDivider />
        <VCardActions class="pa-3 d-flex flex-column flex-md-row ga-4">
          <div v-if="hasInvalidTransfers" class="d-flex flex-column ga-1">
            <span class="text-body-2 pl-2">
              * {{ $t('pages.account.csv_ignored_transfers_hint') }}
            </span>
            <VBtn
              :loading="downloadingInvalid"
              variant="tonal"
              size="x-small"
              class="ml-1"
              :prepend-icon="mdiDownload"
              @click="downloadInvalidCsvRows"
            >
              {{ $t('pages.account.csv_download_invalid') }}
            </VBtn>
          </div>
          <VSpacer />
          <VBtn
            :loading="loading"
            :disabled="!canSubmit"
            density="comfortable"
            variant="elevated"
            color="primary"
            :block="app.isMobile"
            @click="startBatchTransfer"
          >
            {{ $t('terms.transfer') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </VDialog>
  </VBtn>
</template>
<script lang="ts" setup>
import { mdiAlertCircle, mdiCashFast, mdiClose, mdiDownload, mdiTable } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VFileInput,
  VIcon,
  VProgressCircular,
  VSpacer,
  VTable,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import logger from '~/core/logger.core';
import { Account, Asset, TransferOperationInput } from '~/generated/station/station.did';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import { CsvTable } from '~/types/app.types';
import {
  maybeTransformBlockchainAddress,
  registerBeforeUnloadConfirmation,
  unregisterBeforeUnloadConfirmation,
} from '~/utils/app.utils';
import { detectAddressStandard } from '~/utils/asset.utils';
import { downloadCsv, readFileAsCsvTable } from '~/utils/file.utils';
import { requiredRule } from '~/utils/form.utils';
import {
  amountToBigInt,
  arrayBatchMaker,
  assertAndReturn,
  formatBalance,
} from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    account: Account;
    asset: Asset;
    // address: string;
    batchChunkSize?: number;
    icon?: string;
    text?: string;
    color?: string;
    density?: 'comfortable' | 'compact' | 'default';
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined';
    prependIcon?: string;
    appendIcon?: string;
    dialogMaxWidth?: number;
  }>(),
  {
    batchChunkSize: 10,
    density: 'default',
    color: 'primary-variant',
    size: 'default',
    variant: 'outlined',
    icon: undefined,
    prependIcon: undefined,
    appendIcon: undefined,
    text: undefined,
    dialogMaxWidth: 1000,
  },
);

const i18n = useI18n();
const app = useAppStore();
const station = useStationStore();
const btnText = computed(() =>
  props.text || props.icon ? '' : i18n.t('pages.accounts.btn_upload_csv'),
);
const dialogTitle = computed(() => btnText.value ?? i18n.t('pages.accounts.btn_upload_csv'));
const open = ref(false);
const loading = ref(false);
const transfersCsv = ref<File[] | undefined>(undefined);
const csvToColumn = computed(() => i18n.t('pages.account.csv_transfer_file_column_to'));
const csvCommentColumn = computed(() => i18n.t('pages.account.csv_transfer_file_column_comment'));
const csvAmountColumn = computed(() => i18n.t('pages.account.csv_transfer_file_column_amount'));
const csvHint = computed(() =>
  i18n.t('pages.account.csv_transfer_file_format_hint', {
    to: csvToColumn.value,
    amount: csvAmountColumn.value,
    comment: csvCommentColumn.value,
  }),
);
const hasInvalidTransfers = computed(() => rows.value.some(row => !row.valid));
const rawCsvTable = ref<CsvTable | null>(null);
const invalidRawCsvTable = ref<CsvTable | null>(null);
const downloadingInvalid = ref(false);

type CsvTransferWithComment = Partial<TransferOperationInput> & { comment?: string };

const rows = ref<
  {
    transfer: CsvTransferWithComment;
    valid: boolean;
    requesting: boolean;
    failed: boolean;
    requested: boolean;
  }[]
>([]);

const canSubmit = computed(() => {
  if (rows.value.length === 0) {
    return false;
  }

  const remainingValidRows = rows.value.filter(row => row.valid && !row.requested);

  return !rows.value.some(row => row.requesting) && remainingValidRows.length > 0;
});

const hasAnyComments = computed(() => rows.value.some(row => row.transfer.comment));

const totalAmount = computed(() =>
  rows.value.reduce((acc, row) => {
    if (row.valid && row.transfer.amount) {
      return acc + row.transfer.amount;
    }

    return acc;
  }, 0n),
);

watch(
  () => open.value,
  open => {
    if (!open) {
      transfersCsv.value = undefined;
      rows.value = [];
      rawCsvTable.value = null;
    }
  },
);

watch(
  () => transfersCsv.value,
  async files => {
    if (!files || !files.length) {
      rows.value = [];
      rawCsvTable.value = null;
      invalidRawCsvTable.value = null;
      return;
    }

    const table = await readFileAsCsvTable(files[0]);
    rows.value = [];
    rawCsvTable.value = table;
    invalidRawCsvTable.value = {
      headers: table.headers,
      rows: [],
    };

    for (const row of rawCsvTable.value.rows) {
      const transfer: CsvTransferWithComment = {};
      let valid = true;
      const maybeToAddress = row?.[csvToColumn.value];
      if (maybeToAddress !== undefined) {
        const maybeStandard = detectAddressStandard(
          props.asset,
          maybeToAddress,
          station.configuration.details.supported_blockchains,
        );

        if (maybeStandard) {
          transfer.to = maybeToAddress;
          transfer.with_standard = maybeStandard.standard;
        } else {
          valid = false;
        }
      }

      if (row?.[csvCommentColumn.value] !== undefined) {
        transfer.comment = row[csvCommentColumn.value];
      }

      if (row?.[csvAmountColumn.value] !== undefined) {
        try {
          transfer.amount = amountToBigInt(row[csvAmountColumn.value], props.asset.decimals);
        } catch (e) {
          valid = false;
        }
      }

      if (transfer.to === undefined || transfer.amount === undefined) {
        valid = false;
      }

      if (!valid) {
        invalidRawCsvTable.value.rows.push(row);
      }

      rows.value.push({
        transfer: transfer,
        valid,
        requesting: false,
        requested: false,
        failed: false,
      });
    }
  },
);

const downloadInvalidCsvRows = async (): Promise<void> => {
  if (!invalidRawCsvTable.value?.rows.length || downloadingInvalid.value) {
    return;
  }

  try {
    downloadingInvalid.value = true;

    await downloadCsv({
      content: invalidRawCsvTable.value,
      filename: 'invalid_' + new Date().toISOString().toLowerCase() + '.csv',
    });
  } catch (e) {
    logger.error(`Failed to download invalid csv rows, reason: ${e}`);

    app.sendNotification({
      type: 'error',
      message: i18n.t('app.download_error'),
    });
  } finally {
    downloadingInvalid.value = false;
  }
};

const startBatchTransfer = async (): Promise<void> => {
  if (!canSubmit.value || loading.value) {
    return;
  }

  try {
    registerBeforeUnloadConfirmation();
    loading.value = true;
    const transfersToProcess: {
      rowId: number;
      transfer: TransferOperationInput;
      comment?: string;
    }[] = [];
    for (let rowId = 0; rowId < rows.value.length; rowId++) {
      const row = rows.value[rowId];
      if (row.valid && !row.requested && !row.requesting) {
        transfersToProcess.push({
          rowId,
          transfer: {
            from_account_id: props.account.id,
            from_asset_id: props.asset.id,
            with_standard: assertAndReturn(row.transfer.with_standard, 'with_standard'),
            amount: assertAndReturn(row.transfer.amount, 'amount'),
            to: maybeTransformBlockchainAddress(
              props.asset.blockchain,
              assertAndReturn(row.transfer.with_standard, 'with_standard'),
              assertAndReturn(row.transfer.to, 'to'),
            ),
            network: [],
            fee: [],
            metadata: [],
          },
          comment: row.transfer.comment,
        });
      }
    }

    for (const currentChunk of arrayBatchMaker(transfersToProcess, props.batchChunkSize)) {
      const inProgressRequests = currentChunk.map(entry => {
        const row = rows.value[entry.rowId];
        row.requesting = true;
        return station.service
          .transfer(entry.transfer, entry.comment)
          .then(() => {
            row.requested = true;
          })
          .catch(e => {
            row.failed = true;

            logger.error(`Failed to process batch of transfer #${entry.rowId}, reason: ${e}`);
          })
          .finally(() => {
            row.requesting = false;
          });
      });

      await Promise.all(inProgressRequests);
    }
  } catch (e) {
    logger.error('Failed to start batch transfer', e);

    app.sendNotification({
      type: 'error',
      message: i18n.t('pages.account.csv_transfer_failed'),
    });
  } finally {
    loading.value = false;
    unregisterBeforeUnloadConfirmation();
  }
};
</script>
