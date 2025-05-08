<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="formValue.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name }}
      </template>
    </RequestOperationListRow>
  </div>
  <template v-else>
    <VAlert
      v-if="currentUserGroupFailed"
      type="error"
      variant="tonal"
      density="compact"
      class="mb-4"
    >
      {{ $t('requests.failed_to_fetch_details') }}
      <div>{{ currentUserGroupFailed }}</div>
    </VAlert>
    <UserGroupForm :model-value="formValue" mode="view" :current-user-group="currentUserGroup" />
  </template>
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import UserGroupForm from '~/components/users/UserGroupForm.vue';
import { EditUserGroupOperation, Request, UserGroup } from '~/generated/station/station.did';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import { services } from '~/plugins/services.plugin';
import { useAppStore } from '~/stores/app.store';
import { variantIs } from '~/utils/helper.utils';
import { VAlert } from 'vuetify/components';
import { getErrorMessage } from '~/utils/error.utils';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditUserGroupOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const isDiffMode = computed(() => !isListMode.value && variantIs(props.request.status, 'Created'));
const formValue: Ref<Partial<UserGroup>> = ref({});
const currentUserGroup: Ref<UserGroup | undefined> = ref();
const currentUserGroupFailed = ref<string | undefined>();

const stationService = services().station;
const appStore = useAppStore();

onBeforeMount(async () => {
  formValue.value = {
    id: props.operation.input.user_group_id,
    name: props.operation.input.name,
  };

  if (isDiffMode.value) {
    try {
      currentUserGroup.value = (
        await stationService.getUserGroup({ user_group_id: props.operation.input.user_group_id })
      ).user_group;
    } catch (e) {
      currentUserGroupFailed.value = getErrorMessage(e);
      appStore.sendErrorNotification(e);
    }
  }
});
</script>
