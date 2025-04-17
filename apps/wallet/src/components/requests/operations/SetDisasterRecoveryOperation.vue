<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.user_group_id">
      <template v-if="userGroupName" #name>{{ $t('terms.user_group') }}</template>
      <template v-else #name>{{ $t('terms.user_group_id') }}</template>
      <template v-if="userGroupName" #content>
        {{ userGroupName }} ({{ formValue.user_group_id }})
      </template>
      <template v-else #content>
        {{ formValue.user_group_id }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="formValue.quorum">
      <template #name>{{ $t('terms.quorum') }}</template>
      <template #content>
        {{ formValue.quorum }}
      </template>
    </RequestOperationListRow>
  </div>
  <DisasterRecoveryForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import { Request, SetDisasterRecoveryOperation } from '~/generated/station/station.did';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import DisasterRecoveryForm, {
  DisasterRecoveryModel,
} from '~/components/settings/DisasterRecoveryForm.vue';
import { services } from '~/plugins/services.plugin';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: SetDisasterRecoveryOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<DisasterRecoveryModel> = ref({
  quorum: 0,
  user_group_id: '',
});

const userGroupName = ref<string>(props.operation.committee[0]?.user_group_id ?? '');

onBeforeMount(async () => {
  formValue.value = {
    quorum: props.operation.committee[0]?.quorum ?? 0,
    user_group_id: props.operation.committee[0]?.user_group_id ?? '',
  };

  if (props.operation.committee[0]?.user_group_id) {
    try {
      // load user group if possible
      const result = await services().station.getUserGroup({
        user_group_id: props.operation.committee[0]?.user_group_id,
      });

      userGroupName.value = result.user_group.name;
    } catch {
      // ignore
    }
  }
});
</script>
