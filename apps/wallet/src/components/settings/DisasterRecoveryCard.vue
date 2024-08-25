<template>
  <VCard :loading="loading">
    <VCardTitle>
      {{ $t(`app.disaster_recovery_card_title`) }}
    </VCardTitle>

    <VCardText v-if="!loading && error" class="pb">
      <VAlert type="error" variant="tonal" density="compact">
        {{ $t('app.data_load_error') }}
      </VAlert>
    </VCardText>
    <VCardText v-else-if="!!systemInfo" class="pb">
      <VList v-if="systemInfo.disaster_recovery[0]" lines="two" class="bg-transparent">
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.user_group`) }}: </VListItemTitle>
          <VListItemSubtitle>
            {{ systemInfo.disaster_recovery[0].user_group_name[0] || $t('terms.unknown') }}
          </VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold"
            >{{ $t(`terms.user_group_id`) }}:
          </VListItemTitle>
          <VListItemSubtitle>
            {{ systemInfo.disaster_recovery[0].committee.user_group_id }}

            <VBtn
              size="x-small"
              variant="text"
              :icon="mdiContentCopy"
              @click="
                copyToClipboard({
                  textToCopy: systemInfo.disaster_recovery[0].committee.user_group_id,
                  sendNotification: true,
                })
              "
            />
          </VListItemSubtitle>
        </VListItem>
        <VListItem class="px-0">
          <VListItemTitle class="font-weight-bold">{{ $t(`terms.quorum`) }}: </VListItemTitle>
          <VListItemSubtitle>
            {{ systemInfo.disaster_recovery[0].committee.quorum }}
          </VListItemSubtitle>
        </VListItem>
      </VList>
      <div v-else>
        <VAlert
          type="warning"
          variant="tonal"
          density="compact"
          class="mb-4"
          data-test-id="dr-not-configured"
        >
          {{ $t('app.disaster_recovery_not_configured') }}
        </VAlert>
      </div>
      <AuthCheck :privileges="[Privilege.SystemUpgrade]">
        <ActionBtn
          v-model="setDisasterRecoveryInput"
          :title="$t(`app.disaster_recovery_dialog_title`)"
          color="primary"
          :submit="submitSetDisasterRecovery"
          variant="elevated"
          :text="!!systemInfo ? $t('terms.change') : $t('terms.configure')"
          data-test-id="configure-dr-btn"
          @failed="useOnFailedOperation"
          @submitted="useOnSuccessfulOperation"
        >
          <template #default="{ model: elem, submit }">
            <DisasterRecoveryForm
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
              data-test-id="save-dr-btn"
              @click="submit"
            >
              {{ $t('terms.save') }}
            </VBtn>
          </template>
        </ActionBtn>
      </AuthCheck>
    </VCardText>
  </VCard>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import {
  VAlert,
  VBtn,
  VCard,
  VCardText,
  VCardTitle,
  VList,
  VListItem,
  VListItemSubtitle,
  VListItemTitle,
} from 'vuetify/components';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Request, SystemInfo } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import ActionBtn from '../buttons/ActionBtn.vue';
import DisasterRecoveryForm, { DisasterRecoveryModel } from './DisasterRecoveryForm.vue';
import AuthCheck from '../AuthCheck.vue';
import { Privilege } from '~/types/auth.types';
import { mdiContentCopy } from '@mdi/js';
import { copyToClipboard } from '~/utils/app.utils';

const loading = ref(true);
const error = ref<boolean>(false);
const systemInfo = ref<SystemInfo | null>(null);

const { station } = services();
const app = useAppStore();

onMounted(async () => {
  try {
    loading.value = true;

    systemInfo.value = await station.systemInfo(true).then(result => result.system);

    setDisasterRecoveryInput.value.model.quorum =
      systemInfo.value?.disaster_recovery[0]?.committee.quorum || 1;
    setDisasterRecoveryInput.value.model.user_group_id =
      systemInfo.value?.disaster_recovery[0]?.committee.user_group_id;
  } catch (e: unknown) {
    app.sendErrorNotification(e);
    error.value = true;
  } finally {
    loading.value = false;
  }
});

const setDisasterRecoveryInput = ref<{
  valid: boolean;
  model: DisasterRecoveryModel;
}>({
  valid: false,
  model: {
    quorum: 1,
  },
});

const submitSetDisasterRecovery = async ({
  model,
}: {
  valid: boolean;
  model: DisasterRecoveryModel;
}): Promise<Request> => {
  return station.createSetDisasterRecoveryCommitteeRequest({
    quorum: model.quorum,
    user_group_id: model.user_group_id!,
  });
};
</script>
