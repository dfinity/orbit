<template>
  <VContainer>
    <VRow fluid>
      <VCol cols="12" class="px-0 pt-0">
        <VTable density="compact" hover>
          <thead>
            <tr>
              <th class="w-50">{{ $t(`permissions.resource_title`) }}</th>
              <th>{{ $t(`permissions.group_members_title`) }}</th>
              <th>{{ $t(`permissions.specific_users_title`) }}</th>
              <th>{{ $t(`permissions.everyone_title`) }}</th>
            </tr>
          </thead>
          <tbody>
            <ResourcePermissionsRow
              v-for="(resource, idx) in resourcePermissions"
              :key="idx"
              :resource="resource"
              :policies="props.accessPolicies"
              @editing="emit('editing', $event)"
            />
          </tbody>
        </VTable>
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { resourcePermissions } from '~/configs/permissions.config';
import { AccessPolicy } from '~/generated/wallet/wallet.did';
import ResourcePermissionsRow from './ResourcePermissionsRow.vue';

const props = withDefaults(
  defineProps<{
    accessPolicies: AccessPolicy[];
  }>(),
  {
    accessPolicies: () => [],
  },
);

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
