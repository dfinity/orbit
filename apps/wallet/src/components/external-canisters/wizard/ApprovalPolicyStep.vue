<template>
  <VRow>
    <VCol cols="12" class="px-0">
      <VCard flat>
        <VCardTitle data-test-id="update-approval-policy">
          <TextLabel
            :label="$t('external_canisters.config_change_approval_policy')"
            :tooltip="$t('external_canisters.config_change_approval_policy_hint')"
          />
        </VCardTitle>
        <VCardText class="d-flex flex-column ga-4">
          <div v-for="(_, idx) in model.change" :key="idx" class="d-flex flex-column ga-4">
            <div class="d-flex flex-row">
              <div
                v-if="!isViewMode && model.change.length > 1 && idx !== 0"
                class="d-flex align-center"
              >
                <VBtn
                  :icon="mdiMinus"
                  variant="flat"
                  density="comfortable"
                  class="mr-2"
                  size="x-small"
                  @click.stop="deleteChangePolicyByIndex(idx)"
                />
              </div>
              <div class="flex-grow-1 justify-center d-flex flex-column">
                <RuleBuilder
                  v-model="model.change[idx].rule"
                  :specifier="{ ChangeExternalCanister: { Any: null } }"
                  :disabled="isViewMode"
                  @remove="model.change[idx].rule = undefined"
                />
              </div>
            </div>
            <VDivider v-if="idx < model.change.length - 1" />
          </div>
        </VCardText>
      </VCard>
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import { mdiMinus } from '@mdi/js';
import { computed, onMounted } from 'vue';
import { VBtn, VCard, VCardText, VCardTitle, VCol, VDivider, VRow } from 'vuetify/components';
import { CanisterApprovalPolicyModel } from '~/components/external-canisters/wizard/wizard.types';
import RuleBuilder from '~/components/request-policies/rule/RuleBuilder.vue';
import TextLabel from '~/components/ui/TextLabel.vue';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterApprovalPolicyModel;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterApprovalPolicyModel): void;
}>();

const isViewMode = computed(() => props.mode === 'view');
const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const deleteChangePolicyByIndex = (index: number) => {
  model.value.change = model.value.change.filter((_, idx) => idx !== index);
};

onMounted(() => {
  if (model.value.change.length === 0) {
    model.value.change.push({ policy_id: undefined, rule: undefined });
  }
});
</script>
