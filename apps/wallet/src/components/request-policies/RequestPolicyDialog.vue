<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || saving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-slot="{ data }"
      :load="loadPolicy"
      @loading="loading = $event"
      @loaded="requestPolicy = $event.policy"
    >
      <VCard>
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('pages.request_policies.dialog_title') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
        <VCardText v-else>
          <RequestPolicyForm
            v-if="data"
            v-model="requestPolicy"
            :mode="props.readonly.value ? 'view' : 'edit'"
            @submit="save"
            @valid="valid = $event"
          />
        </VCardText>
        <VDivider />
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn
            v-if="!props.readonly.value"
            color="primary"
            variant="elevated"
            :disabled="!canSave"
            :loading="saving"
            @click="save"
          >
            {{ $t('terms.save') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import RequestPolicyForm from '~/components/request-policies/RequestPolicyForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { RequestPolicy, UUID } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    policyId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    policyId: undefined,
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const props = toRefs(input);
const valid = ref(true);
const loading = ref(false);
const saving = ref(false);
const requestPolicy = ref<Partial<RequestPolicy>>({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const station = useStationStore();

const loadPolicy = async (): Promise<{
  policy: Partial<RequestPolicy>;
}> => {
  if (props.policyId.value === undefined) {
    const createModel: Partial<RequestPolicy> = {
      rule: { AutoApproved: null },
    };

    return { policy: createModel };
  }

  const result = await station.service.getRequestPolicy(props.policyId.value, true);
  return result;
};

const canSave = computed(() => {
  return (
    valid.value && !loading.value && !!requestPolicy.value?.rule && !!requestPolicy.value?.specifier
  );
});

const save = async (): Promise<void> => {
  if (!canSave.value) {
    return;
  }

  try {
    saving.value = true;
    if (requestPolicy.value.id) {
      const request = await station.service.editRequestPolicy({
        policy_id: requestPolicy.value.id,
        specifier: [assertAndReturn(requestPolicy.value.specifier)],
        rule: [assertAndReturn(requestPolicy.value.rule)],
      });

      useOnSuccessfulOperation(request);

      openModel.value = false;
      return;
    }

    const request = await station.service.addRequestPolicy({
      specifier: assertAndReturn(requestPolicy.value.specifier),
      rule: assertAndReturn(requestPolicy.value.rule),
    });

    useOnSuccessfulOperation(request);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to save request policy ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
