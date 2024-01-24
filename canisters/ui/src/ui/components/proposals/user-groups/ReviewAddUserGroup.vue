<template>
  <UserGroupForm :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { AddUserGroupOperation, Proposal, UserGroup } from '~/generated/wallet/wallet.did';
import UserGroupForm from '~/ui/components/forms/UserGroupForm.vue';

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
