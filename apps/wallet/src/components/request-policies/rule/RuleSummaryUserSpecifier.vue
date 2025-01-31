<template>
  <template v-if="userSpecifier.kind === RequestPolicyRuleUserSpecifierEnum.Any">
    {{ $t('request_policies.rule_rich_summary.any_user_specifier') }}
  </template>
  <template v-else-if="userSpecifier.kind === RequestPolicyRuleUserSpecifierEnum.Id">
    <template v-if="userSpecifier.users.length === 0">
      <VIcon :icon="mdiAlertCircle"></VIcon>
      {{ $t('request_policies.rule_rich_summary.no_user_specifier') }}
    </template>
    <template v-for="(user, index) in userSpecifier.users" :key="index">
      <InteractiveUser :id="user.id" :name="user.name" />
      <span v-if="index < userSpecifier.users.length - 1">, </span>
    </template>
  </template>
  <template v-else-if="userSpecifier.kind === RequestPolicyRuleUserSpecifierEnum.Group">
    <template v-for="(group, index) in userSpecifier.groups" :key="index">
      <InteractiveUserGroup :id="group.id" :name="group.name" />
      <span v-if="index < userSpecifier.groups.length - 1">, </span>
    </template>
  </template>
</template>

<script setup lang="ts">
import { RequestPolicyRuleUserSpecifierEnum } from '~/types/station.types';
import { PopulatedUserSpecifier } from '~/composables/request-policies.composable';
import InteractiveUserGroup from '~/components/users/InteractiveUserGroup.vue';
import InteractiveUser from '~/components/users/InteractiveUser.vue';
import { mdiAlertCircle } from '@mdi/js';

defineProps<{
  userSpecifier: PopulatedUserSpecifier;
}>();
</script>
