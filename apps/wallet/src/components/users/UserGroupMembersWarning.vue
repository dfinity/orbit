<template>
  <VAlert
    v-if="warning"
    type="warning"
    variant="tonal"
    density="compact"
    data-test-id="user-group-members-warning"
  >
    {{ warning }}
  </VAlert>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VAlert } from 'vuetify/components';
import logger from '~/core/logger.core';
import { UUID } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';

const props = defineProps<{
  userGroupId: UUID;
}>();

const MAX_LISTED_MEMBERS = 10;

const i18n = useI18n();
const station = useStationStore();

const memberNames = ref<string[]>([]);
const totalMembers = ref(0);
const failedToLoadMembers = ref(false);

const warning = computed(() => {
  if (failedToLoadMembers.value) {
    return i18n.t('user_groups.members_removal_warning_unknown');
  }

  if (totalMembers.value === 0) {
    return '';
  }

  const users = memberNames.value.join(', ');
  const remaining = totalMembers.value - memberNames.value.length;

  if (remaining > 0) {
    return i18n.t('user_groups.members_removal_warning_more', { users, count: remaining });
  }

  return i18n.t('user_groups.members_removal_warning', { users }, totalMembers.value);
});

onMounted(async () => {
  try {
    const result = await station.service.listUsers({
      groups: [props.userGroupId],
      limit: MAX_LISTED_MEMBERS,
    });

    memberNames.value = result.users.map(user => user.name);
    totalMembers.value = Number(result.total);
  } catch (e) {
    logger.error('Failed to fetch user group members', e);

    failedToLoadMembers.value = true;
  }
});
</script>
