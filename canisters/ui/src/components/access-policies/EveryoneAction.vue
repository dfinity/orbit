<template>
  <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
    <ActionBtn
      size="default"
      density="comfortable"
      data-test-id="everyone-action-btn"
      :model-value="{
        specifier: specifier.specifier,
        everyone: { policyId: specifier.users.allUsers.policy.id },
      }"
      :disabled="isEnabled && !specifier.users.allUsers.policy.canRemove"
      :icon="isEnabled ? mdiCheckboxMarkedOutline : mdiCheckboxBlankOutline"
      :submit="
        ({ specifier, everyone }) => {
          if (everyone.policyId) {
            return wallet.service.removeAccessPolicy({
              policy_id: everyone.policyId,
            });
          }

          return wallet.service.addAccessPolicy({
            user: { Any: null },
            resource: specifier,
          });
        }
      "
      @opened="emit('editing', true)"
      @closed="emit('editing', false)"
      @failed="useOnFailedOperation"
      @submitted="useOnSuccessfulOperation"
    />

    <template #unauthorized>
      <VCheckbox hide-details density="comfortable" disabled :value="isEnabled" />
    </template>
  </AuthCheck>
</template>

<script lang="ts" setup>
import { mdiCheckboxBlankOutline, mdiCheckboxMarkedOutline } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { Privilege } from '~/types/auth.types';
import { ResourceAccessPolicySpecifier } from '~/types/access-policies.types';
import AuthCheck from '~/components/AuthCheck.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { useWalletStore } from '~/stores/wallet.store';

const wallet = useWalletStore();

const props = defineProps<{
  specifier: ResourceAccessPolicySpecifier;
}>();

const { specifier } = toRefs(props);

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();

const isEnabled = computed(() => !!specifier.value.users.allUsers.policy.id);
</script>
