<template>
  <VRow>
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle data-test-id="read-permission">
          <TextLabel
            :label="$t('external_canisters.config_read_permission')"
            :tooltip="$t('external_canisters.config_read_permission_hint')"
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
        <VCardTitle data-test-id="change-permission">
          <TextLabel
            :label="$t('external_canisters.config_change_permission')"
            :tooltip="$t('external_canisters.config_change_permission_hint')"
          />
        </VCardTitle>
        <VCardText class="py-0">
          <AllowInput v-model="model.change" :mode="props.mode" />
        </VCardText>
      </VCard>
    </VCol>
  </VRow>
</template>
<script lang="ts" setup>
import { computed } from 'vue';
import { VCard, VCardText, VCardTitle, VCol, VDivider, VRow } from 'vuetify/components';
import AllowInput from '~/components/inputs/AllowInput.vue';
import TextLabel from '~/components/ui/TextLabel.vue';
import { CanisterPermissionModel } from './wizard.types';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterPermissionModel;
    mode?: 'view' | 'edit';
  }>(),
  {
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterPermissionModel): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});
</script>
