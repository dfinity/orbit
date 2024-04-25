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
      :load="loadUser"
      @loading="loading = $event"
      @loaded="user = $event.user"
    >
      <VCard>
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('terms.user') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
        <VCardText v-else>
          <UserForm
            v-if="data"
            v-model="user"
            v-model:trigger-submit="triggerSubmit"
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
            :disabled="!canSave"
            :loading="saving"
            color="primary"
            variant="elevated"
            @click="triggerSubmit = true"
          >
            {{ props.userId.value ? $t('terms.save') : $t('terms.create') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { Ref, computed, ref, toRefs } from 'vue';
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
import UserForm from '~/components/users/UserForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { UUID, User } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { assertAndReturn } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    userId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    userId: undefined,
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
const user: Ref<Partial<User>> = ref({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const wallet = useWalletStore();

const loadUser = async (): Promise<{
  user: Partial<User>;
}> => {
  if (props.userId.value === undefined) {
    const createModel: Partial<User> = {};

    return { user: createModel };
  }

  const result = await wallet.service.getUser(
    {
      user_id: props.userId.value,
    },
    true,
  );
  return { user: result.user };
};

const canSave = computed(() => {
  return valid.value && !loading.value;
});

const triggerSubmit = ref(false);

const save = async (): Promise<void> => {
  if (!canSave.value) {
    return;
  }

  try {
    saving.value = true;
    if (user.value.id) {
      const proposal = await wallet.service.editUser({
        id: user.value.id,
        groups: [assertAndReturn(user.value.groups, 'groups').map(g => g.id)],
        identities: [assertAndReturn(user.value.identities, 'identities')],
        name: user.value.name !== undefined ? user.value.name : [],
        status: [assertAndReturn(user.value.status, 'status')],
      });

      useOnSuccessfulOperation(proposal);

      openModel.value = false;
      return;
    }

    const proposal = await wallet.service.addUser({
      groups: assertAndReturn(user.value.groups, 'groups').map(g => g.id),
      identities: assertAndReturn(user.value.identities, 'identities'),
      name: user.value.name !== undefined ? user.value.name : [],
      status: assertAndReturn(user.value.status, 'status'),
    });

    useOnSuccessfulOperation(proposal);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to save user ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
