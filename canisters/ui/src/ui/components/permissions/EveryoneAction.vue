<template>
  <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
    <ActionBtn
      size="default"
      density="comfortable"
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
import { computed } from 'vue';
import { toRefs } from 'vue';
import { ResourcePermissionsSpecifier } from '~/configs/permissions.config';
import { Privilege } from '~/types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/ui/composables/notifications.composable';
import { useWalletStore } from '~/ui/stores/wallet';

const wallet = useWalletStore();

const props = defineProps<{
  specifier: ResourcePermissionsSpecifier;
}>();

const { specifier } = toRefs(props);

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();

const isEnabled = computed(() => !!specifier.value.users.allUsers.policy.id);
</script>
