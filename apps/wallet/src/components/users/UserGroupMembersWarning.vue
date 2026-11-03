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
const membersLoaded = ref(false);

const warning = computed(() => {
  // The generic warning is shown until the members are loaded, so it is never hidden while the
  // confirmation is already possible, and when none are listed, since the caller might not be
  // allowed to see them.
  if (!membersLoaded.value || memberNames.value.length === 0) {
    return i18n.t('user_groups.members_removal_warning_unknown');
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
    membersLoaded.value = true;
  } catch (e) {
    logger.error('Failed to fetch user group members', e);
  }
});
</script>
