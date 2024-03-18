<template>
  <ActionBtn
    v-slot="{ model: elem, submit }"
    size="small"
    density="comfortable"
    data-test-id="everyone-action-btn"
    :class="{ 'ml-1': !everyone.label }"
    :model-value="{
      everyone: specifier.allow.allUsers,
    }"
    dialog-content-class="pa-0"
    :disabled="!specifier.canEdit"
    :text="everyone.label"
    :icon="everyone.icon"
    :submit="
      ({ everyone }) => {
        const userAuthentication = toUserAuthentication(everyone);
        if (!userAuthentication) {
          const deny =
            specifier.allow.allUsers === AccessPolicyForAllUsers.AuthenticationRequired
              ? { Authenticated: null }
              : { Any: null };

          return wallet.service.editAccessPolicy({
            access: { Deny: deny },
            resource: specifier.resource,
          });
        }

        return wallet.service.editAccessPolicy({
          access: { Allow: { user_groups: [], users: [], authentication: [userAuthentication] } },
          resource: specifier.resource,
        });
      }
    "
    @opened="emit('editing', true)"
    @closed="emit('editing', false)"
    @failed="useOnFailedOperation"
    @submitted="useOnSuccessfulOperation"
  >
    <EveryoneForm v-model="elem.value.everyone" @submit="submit" />
  </ActionBtn>
</template>

<script lang="ts" setup>
import { mdiAccountKey, mdiEarth, mdiPencil } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { useI18n } from 'vue-i18n';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { toUserAuthentication } from '~/mappers/access-policies.mapper';
import { useWalletStore } from '~/stores/wallet.store';
import {
  AccessPolicyForAllUsers,
  ResourceAccessPolicySpecifier,
} from '~/types/access-policies.types';
import EveryoneForm from './EveryoneForm.vue';

const wallet = useWalletStore();
const props = defineProps<{
  specifier: ResourceAccessPolicySpecifier;
}>();

const { specifier } = toRefs(props);
const i18n = useI18n();

const everyone = computed(() => {
  if (specifier.value.allow.allUsers === AccessPolicyForAllUsers.Public) {
    return { icon: mdiEarth, label: i18n.t('access_policies.allow.anyone') };
  }

  if (specifier.value.allow.allUsers === AccessPolicyForAllUsers.AuthenticationRequired) {
    return { icon: mdiAccountKey, label: i18n.t('access_policies.allow.authenticated') };
  }

  return { icon: mdiPencil };
});

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
