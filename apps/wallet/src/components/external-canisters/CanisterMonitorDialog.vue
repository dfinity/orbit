<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard data-test-id="canister-monitor-card">
      <VToolbar color="background">
        <VToolbarTitle>
          {{ dialogTitle }}
        </VToolbarTitle>
        <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
      </VToolbar>
      <VDivider />
      <CanisterMonitorForm
        v-model="monitorModel"
        :display="{ canisterId: !monitorModel.canisterId }"
        @submitting="canClose = !$event"
        @submitted="open = false"
      >
      </CanisterMonitorForm>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@icp-sdk/core/principal';
import { mdiClose } from '@mdi/js';
import { Ref, computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCard, VDialog, VDivider, VToolbar, VToolbarTitle } from 'vuetify/components';
import { CanisterMonitorModel } from './external-canisters.types';
import CanisterMonitorForm from '~/components/external-canisters/monitor/CanisterMonitorForm.vue';

const props = withDefaults(
  defineProps<{
    open?: boolean;
    canisterId?: Principal;
    dialogMaxWidth?: number;
    title?: string;
  }>(),
  {
    open: false,
    canisterId: undefined,
    dialogMaxWidth: 800,
    title: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const i18n = useI18n();
const canClose = ref(true);
const dialogTitle = computed(() => props.title || i18n.t('external_canisters.monitor.title'));

const buildModel = (): CanisterMonitorModel => ({
  canisterId: props.canisterId,
  fundingStrategy: undefined,
  cycleObtainStrategy: undefined,
});

const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const monitorModel = ref(buildModel()) as Ref<CanisterMonitorModel>;
</script>
