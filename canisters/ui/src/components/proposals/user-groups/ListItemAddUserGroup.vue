<template>
  <p>
    <span class="text-body-2">{{ props.operation.input.name }}</span>
  </p>
</template>

<script setup lang="ts">
import { AddUserGroupOperation, Proposal } from '~/generated/wallet/wallet.did';

const props = defineProps<{
  proposal: Proposal;
  operation: AddUserGroupOperation;
}>();
</script>
