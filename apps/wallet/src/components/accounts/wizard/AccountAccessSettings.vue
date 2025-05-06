<template>
  <VRow>
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle data-test-id="read-access">
          <TextLabel
            :label="$t('app.account_dialog_access_read')"
            :tooltip="$t('app.account_dialog_access_read_hint')"
          />
        </VCardTitle>
        <VCardText>
          <DiffView :before-value="props.currentPermissions?.read" :after-value="model.read">
            <template #default="{ value, diffMode }">
              <AllowInput
                v-if="value"
                :model-value="value"
                :mode="diffMode === 'before' ? 'view' : props.mode"
                @update:model-value="val => diffMode === 'after' && (model.read = val)"
              />
            </template>
          </DiffView>
        </VCardText>
      </VCard>
    </VCol>
    <VDivider />
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle data-test-id="update-access">
          <TextLabel
            :label="$t('app.account_dialog_access_configuration')"
            :tooltip="$t('app.account_dialog_access_configuration_hint')"
          />
        </VCardTitle>
        <VCardText>
          <DiffView
            :before-value="props.currentPermissions?.configuration"
            :after-value="model.configuration"
          >
            <template #default="{ value, diffMode }">
              <AllowInput
                v-if="value"
                :model-value="value"
                :mode="diffMode === 'before' ? 'view' : props.mode"
                @update:model-value="val => diffMode === 'after' && (model.configuration = val)"
              />
            </template>
          </DiffView>
        </VCardText>
      </VCard>
    </VCol>
    <VDivider />
    <VCol cols="12" class="px-0 pb-8">
      <VCard flat>
        <VCardTitle data-test-id="transfer-access">
          <TextLabel
            :label="$t('app.account_dialog_access_transfer')"
            :tooltip="$t('app.account_dialog_access_transfer_hint')"
          />
        </VCardTitle>
        <VCardText>
          <DiffView
            :before-value="props.currentPermissions?.transfer"
            :after-value="model.transfer"
          >
            <template #default="{ value, diffMode }">
              <AllowInput
                v-if="value"
                :model-value="value"
                :mode="diffMode === 'before' ? 'view' : props.mode"
                @update:model-value="val => diffMode === 'after' && (model.transfer = val)"
              />
            </template>
          </DiffView>
        </VCardText>
      </VCard>
    </VCol>
  </VRow>
</template>
<script lang="ts" setup>
import { computed } from 'vue';
import { VCard, VCardText, VCardTitle, VCol, VDivider, VRow } from 'vuetify/components';
import TextLabel from '~/components/ui/TextLabel.vue';
import AllowInput from '~/components/inputs/AllowInput.vue';
import { Allow } from '~/generated/station/station.did';
import DiffView from '~/components/requests/DiffView.vue';

export interface AccountPermissionModel {
  read: Allow;
  transfer: Allow;
  configuration: Allow;
}

const props = withDefaults(
  defineProps<{
    modelValue: AccountPermissionModel;
    mode?: 'view' | 'edit';
    currentPermissions?: AccountPermissionModel;
  }>(),
  {
    mode: 'edit',
    currentPermissions: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AccountPermissionModel): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
