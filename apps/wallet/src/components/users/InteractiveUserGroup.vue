<template>
  <VTooltip
    v-if="hasListUsersPrivilege"
    :text="tooltipContent"
    :loading="isLoadingTooltip"
    :location="'bottom'"
  >
    <template #activator="{ props }">
      <span
        class="underline-dotted font-weight-bold"
        v-bind="props"
        @mouseenter="fetchGroupDetails"
      >
        {{ input.name ?? input.id }}
      </span>
    </template>
  </VTooltip>
  <span v-else class="font-weight-bold">
    {{ input.name ?? input.id }}
  </span>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { services } from '~/plugins/services.plugin';
import { Privilege } from '~/types/auth.types';
import { hasRequiredPrivilege } from '~/utils/auth.utils';

const input = defineProps<{
  id: string;
  name?: string;
}>();

const tooltipContent = ref('');
const isLoadingTooltip = ref(false);
const service = services().station;

const hasListUsersPrivilege = computed(() =>
  hasRequiredPrivilege({ anyOf: [Privilege.ListUsers] }),
);

async function fetchGroupDetails() {
  if (tooltipContent.value) return; // Don't fetch if we already have the data

  isLoadingTooltip.value = true;
  try {
    const groupUsers = await service.listUsers({
      groups: [input.id],
      limit: 5,
    });
    tooltipContent.value = `${groupUsers.total} user${groupUsers.total > 1 ? 's' : ''} in group: ${groupUsers.users.map(user => user.name).join(', ')}`;
  } catch (error) {
    console.error('Failed to fetch group details:', error);
    tooltipContent.value = 'Failed to load group details';
  } finally {
    isLoadingTooltip.value = false;
  }
}
</script>
