<template>
  <ActionBtn
    v-slot="{ model: elem }"
    size="small"
    density="comfortable"
    data-test-id="everyone-action-btn"
    :class="{ 'ml-1': !everyone.label }"
    :model-value="{
      everyone: specifier.allow.allUsers,
    }"
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
    <VRadioGroup
      v-model="elem.value.everyone"
      :label="$t('access_policies.allow.everyone_edit_label')"
    >
      <VRadio
        :label="$t('access_policies.allow.notset')"
        :value="AccessPolicyForAllUsers.NotSet"
      ></VRadio>
      <VRadio
        :label="$t('access_policies.allow.authenticated')"
        :value="AccessPolicyForAllUsers.AuthenticationRequired"
      ></VRadio>
      <VRadio
        :label="$t('access_policies.allow.anyone')"
        :value="AccessPolicyForAllUsers.Public"
      ></VRadio>
    </VRadioGroup>
  </ActionBtn>
</template>

<script lang="ts" setup>
import { mdiAccountKey, mdiEarth, mdiPencil } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { useI18n } from 'vue-i18n';
import { VRadio, VRadioGroup } from 'vuetify/components';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { UserAuthentication } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import {
  AccessPolicyForAllUsers,
  ResourceAccessPolicySpecifier,
} from '~/types/access-policies.types';

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

const toUserAuthentication = (everyone: AccessPolicyForAllUsers): UserAuthentication | null => {
  if (everyone === AccessPolicyForAllUsers.Public) {
    return { None: null };
  }

  if (everyone === AccessPolicyForAllUsers.AuthenticationRequired) {
    return { Required: null };
  }

  return null;
};

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
