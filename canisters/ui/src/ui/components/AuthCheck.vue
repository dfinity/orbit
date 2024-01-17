<template>
  <slot v-if="hasAccess">{{ props }}</slot>
  <slot v-else name="unauthorized"></slot>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { Privilege } from '~/types';
import { RequiredSessionState } from '~/ui/types';
import { hasRequiredPrivilege, hasRequiredSession } from '~/ui/utils/auth';

const props = withDefaults(
  defineProps<{
    session?: RequiredSessionState;
    privileges?: Privilege[];
  }>(),
  {
    session: RequiredSessionState.Authenticated,
    privileges: undefined,
  },
);

const hasAccess = computed<boolean>(() => {
  const matchesRequiredSessionState = hasRequiredSession(props.session);
  const matchesRequiredPrivilege = hasRequiredPrivilege({ anyOf: props.privileges });

  return matchesRequiredSessionState && matchesRequiredPrivilege;
});
</script>
