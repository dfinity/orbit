<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="operation.input.resource">
      <template #name>{{ $t('terms.resource') }}</template>
      <template #content>
        {{
          $t(
            `permissions.resources.${fromResourceToResourceEnum(operation.input.resource).toLowerCase()}`,
          )
        }}
      </template>
    </RequestOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" />
  <PermissionForm v-else :model-value="permission" mode="view" />
</template>

<script setup lang="ts">
import { computed, onBeforeMount, ref } from 'vue';
import { VProgressCircular } from 'vuetify/components';
import PermissionForm from '~/components/permissions/PermissionForm.vue';
import logger from '~/core/logger.core';
import { Permission, EditPermissionOperation, Request } from '~/generated/station/station.did';
import { fromResourceToResourceEnum } from '~/mappers/permissions.mapper';
import { useStationStore } from '~/stores/station.store';
import RequestOperationListRow from '../RequestOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditPermissionOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const station = useStationStore();
const permission = ref<Partial<Permission>>({});
const loading = ref(false);

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const { permission: result } = await station.service.getPermission({
      resource: props.operation.input.resource,
    });

    result.allow.auth_scope = props.operation.input.auth_scope?.[0] ?? result.allow.auth_scope;
    result.allow.users = props.operation.input.users?.[0] ?? result.allow.users;
    result.allow.user_groups = props.operation.input.user_groups?.[0] ?? result.allow.user_groups;

    permission.value = result;
  } catch (e) {
    logger.error('Failed to fetch permission details', e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  fetchDetails();
});
</script>
