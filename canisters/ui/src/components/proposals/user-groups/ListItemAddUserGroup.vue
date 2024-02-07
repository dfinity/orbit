<template>
  <p>
    <span class="text-body-2">{{ proposal.operation.AddUserGroup.input.name }}</span>
  </p>
</template>

<script setup lang="ts">
import { AddUserGroupOperation, Proposal } from '~/generated/wallet/wallet.did';

defineProps<{
  proposal: Proposal & {
    operation: { AddUserGroup: AddUserGroupOperation };
  };
}>();
</script>
