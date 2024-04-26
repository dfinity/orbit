<template>
  <ActionBtn
    v-slot="{ model: elem, submit }"
    size="small"
    density="comfortable"
    data-test-id="everyone-action-btn"
    :class="{ 'ml-1': !everyone.label }"
    :model-value="{
      everyone: specifier.allow.authScope,
    }"
    :disabled="!specifier.canEdit"
    :text="everyone.label"
    :icon="everyone.icon"
    :submit="
      ({ everyone: authScope }) => {
        return station.service.editPermission({
          auth_scope: [toAuthScope(authScope)],
          user_groups: [],
          users: [],
          resource: specifier.resource,
        });
      }
    "
    @opened="emit('editing', true)"
    @closed="emit('editing', false)"
    @failed="useOnFailedOperation"
    @submitted="useOnSuccessfulOperation"
  >
    <EveryoneForm v-model="elem.value.everyone" @submit="submit" />
  </ActionBtn>
</template>

<script lang="ts" setup>
import { mdiAccountKey, mdiEarth, mdiPencil } from '@mdi/js';
import { computed, toRefs } from 'vue';
import { useI18n } from 'vue-i18n';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { useStationStore } from '~/stores/station.store';
import { AuthScopeEnum, ResourcePermissionSpecifier } from '~/types/permissions.types';
import EveryoneForm from './EveryoneForm.vue';
import { toAuthScope } from '~/mappers/permissions.mapper';

const station = useStationStore();
const props = defineProps<{
  specifier: ResourcePermissionSpecifier;
}>();

const { specifier } = toRefs(props);
const i18n = useI18n();

const everyone = computed(() => {
  if (specifier.value.allow.authScope === AuthScopeEnum.Public) {
    return { icon: mdiEarth, label: i18n.t('permissions.allow.anyone') };
  }

  if (specifier.value.allow.authScope === AuthScopeEnum.Authenticated) {
    return { icon: mdiAccountKey, label: i18n.t('permissions.allow.authenticated') };
  }

  return { icon: mdiPencil };
});

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();
</script>
