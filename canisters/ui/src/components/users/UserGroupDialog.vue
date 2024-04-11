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
      :load="loadUserGroup"
      @loading="loading = $event"
      @loaded="userGroup = $event.userGroup"
    >
      <VCard :loading="loading">
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('terms.user_group') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText>
          <UserGroupForm
            v-if="data"
            v-model="userGroup"
            v-model:trigger-submit="triggerSubmit"
            :mode="props.readonly.value ? 'view' : 'edit'"
            @submit="save"
            @valid="valid = $event"
          />
        </VCardText>
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
            {{ props.userGroupId.value ? $t('terms.save') : $t('terms.create') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import UserGroupForm from '~/components/users/UserGroupForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { UUID, UserGroup } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { assertAndReturn } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    userGroupId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    userGroupId: undefined,
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
const userGroup = ref<Partial<UserGroup>>({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const wallet = useWalletStore();

const loadUserGroup = async (): Promise<{
  userGroup: Partial<UserGroup>;
}> => {
  if (props.userGroupId.value === undefined) {
    const createModel: Partial<UserGroup> = {};

    return { userGroup: createModel };
  }

  const result = await wallet.service.getUserGroup({
    user_group_id: props.userGroupId.value,
  });
  return { userGroup: result.user_group };
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
    if (userGroup.value.id) {
      const proposal = await wallet.service.editUserGroup({
        user_group_id: userGroup.value.id,
        name: assertAndReturn(userGroup.value.name, 'name'),
      });

      useOnSuccessfulOperation(proposal);

      openModel.value = false;
      return;
    }

    const proposal = await wallet.service.addUserGroup({
      name: assertAndReturn(userGroup.value.name, 'name'),
    });

    useOnSuccessfulOperation(proposal);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to save user group ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
