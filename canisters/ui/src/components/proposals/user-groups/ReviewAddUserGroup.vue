<template>
  <UserGroupForm :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import UserGroupForm from '~/components/forms/UserGroupForm.vue';
import { AddUserGroupOperation, Proposal, UserGroup } from '~/generated/wallet/wallet.did';

const props = defineProps<{
  proposal: Proposal & {
    operation: { AddUserGroup: AddUserGroupOperation };
  };
}>();

const formValue = computed<Partial<UserGroup>>(() => {
  return {
    name: props.proposal.operation.AddUserGroup.input.name,
  };
});
</script>
