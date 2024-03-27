<template>
  <VRow>
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle>
          <TextLabel
            :label="$t('app.account_dialog_access_read')"
            :tooltip="$t('app.account_dialog_access_read_hint')"
          />
        </VCardTitle>
        <VCardText class="py-0">
          <AllowInput v-model="model.read" :mode="props.mode" />
        </VCardText>
      </VCard>
    </VCol>
    <VDivider />
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle>
          <TextLabel
            :label="$t('app.account_dialog_access_configuration')"
            :tooltip="$t('app.account_dialog_access_configuration_hint')"
          />
        </VCardTitle>
        <VCardText class="py-0">
          <AllowInput v-model="model.configuration" :mode="props.mode" />
        </VCardText>
      </VCard>
    </VCol>
    <VDivider />
    <VCol cols="12" class="px-0 pb-8">
      <VCard flat>
        <VCardTitle>
          <TextLabel
            :label="$t('app.account_dialog_access_transfer')"
            :tooltip="$t('app.account_dialog_access_transfer_hint')"
          />
        </VCardTitle>
        <VCardText class="py-0">
          <AllowInput v-model="model.transfer" :mode="props.mode" />
        </VCardText>
      </VCard>
    </VCol>
  </VRow>
</template>
<script lang="ts" setup>
import { computed } from 'vue';
import { VCard, VCardText, VCardTitle, VCol, VDivider, VRow } from 'vuetify/components';
import TextLabel from '~/components/TextLabel.vue';
import AllowInput from '~/components/inputs/AllowInput.vue';
import { Allow } from '~/generated/wallet/wallet.did';

export interface AccountAccessPolicyModel {
  read: Allow;
  transfer: Allow;
  configuration: Allow;
}

const props = withDefaults(
  defineProps<{
    modelValue: AccountAccessPolicyModel;
    mode: 'view' | 'edit';
  }>(),
  {
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AccountAccessPolicyModel): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
