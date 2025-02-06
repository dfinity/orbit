<template>
  <template v-if="rule.kind === 'NamedRule'">
    <I18nT tag="span" keypath="request_policies.rule_rich_summary.named_rule">
      <template #name>
        <InteractiveNamedRule :id="rule.id" :name="rule.name" />
      </template>
    </I18nT>
  </template>
  <template v-else-if="rule.kind === RequestPolicyRuleEnum.AutoApproved">
    <span>{{ $t('request_policies.rule_rich_summary.auto_approved') }}</span>
  </template>
  <template v-else-if="rule.kind === RequestPolicyRuleEnum.Quorum">
    <!-- No user can approve -->
    <span
      v-if="
        rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Id &&
        (rule.n === 0 || rule.approvers.users.length === 0)
      "
    >
      <VIcon :icon="mdiAlertCircle"></VIcon>
      {{ $t('request_policies.rule_rich_summary.invalid_rule_auto_approved') }}
    </span>

    <!-- Any user can approve -->
    <I18nT
      v-else-if="rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Any"
      tag="span"
      :plural="rule.n"
      keypath="request_policies.rule_rich_summary.any_user_specifier"
    >
      <template #n>
        {{ rule.n }}
      </template>
    </I18nT>

    <!-- Some individual users can approve -->
    <I18nT
      v-else-if="
        rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Id &&
        rule.approvers.users.length === 1
      "
      tag="span"
      :plural="rule.approvers.users.length"
      keypath="request_policies.rule_rich_summary.single_user_specifier"
    >
      <template #user>
        <RuleSummaryUserSpecifier :user-specifier="rule.approvers" />
      </template>
    </I18nT>

    <I18nT
      v-else-if="rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Id"
      tag="span"
      :plural="rule.n"
      keypath="request_policies.rule_rich_summary.user_specifier"
    >
      <template #users>
        <RuleSummaryUserSpecifier :user-specifier="rule.approvers" />
      </template>
      <template #n>
        {{ rule.n }}
      </template>
    </I18nT>

    <!-- Some groups can approve -->
    <I18nT
      v-else-if="rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Group"
      tag="span"
      :plural="rule.n"
      keypath="request_policies.rule_rich_summary.group_specifier"
    >
      <template #groups>
        <RuleSummaryUserSpecifier :user-specifier="rule.approvers" />
      </template>
      <template #n>
        {{ rule.n }}
      </template>
    </I18nT>
  </template>
  <template
    v-else-if="
      rule.kind === RequestPolicyRuleEnum.QuorumPercentage &&
      rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Any
    "
  >
    <I18nT tag="span" keypath="request_policies.rule_rich_summary.quorum_percentage_any_user">
      <template #n>
        {{ rule.n }}
      </template>
    </I18nT>
  </template>
  <template v-else-if="rule.kind === RequestPolicyRuleEnum.QuorumPercentage">
    <I18nT
      tag="span"
      :plural="rule.n"
      keypath="request_policies.rule_rich_summary.quorum_percentage_rule"
    >
      <template #users>
        <RuleSummaryUserSpecifier :user-specifier="rule.approvers" />
      </template>
    </I18nT>
  </template>
  <template v-else-if="rule.kind === RequestPolicyRuleEnum.AllowListedByMetadata">
    <span>{{
      $t('request_policies.rule_rich_summary.allowlisted_by_metadata', {
        metadata: rule.value ? `"${rule.key}=${rule.value}"` : `"${rule.key}"`,
      })
    }}</span>
  </template>
  <template v-else-if="rule.kind === RequestPolicyRuleEnum.AllowListed">
    <span>{{ $t('request_policies.rule_rich_summary.allowlisted') }}</span>
  </template>
  <template v-else-if="rule.kind === RequestPolicyRuleEnum.Not">
    <I18nT tag="span" keypath="request_policies.rule_rich_summary.not">
      <template #rule>
        <RuleSummaryItem :rule="rule.rule" />
      </template>
    </I18nT>
  </template>
  <template v-else-if="rule.kind === RequestPolicyRuleEnum.AnyOf">
    <template v-for="(subRule, index) in rule.rules" :key="index">
      <RuleSummaryItem :rule="subRule" />
      <span v-if="index < rule.rules.length - 1">
        {{ $t('request_policies.rule_rich_summary.anyof') }}
      </span>
    </template>
  </template>
  <template v-else-if="rule.kind === RequestPolicyRuleEnum.AllOf">
    <template v-for="(subRule, index) in rule.rules" :key="index">
      <RuleSummaryItem :rule="subRule" />
      <span v-if="index < rule.rules.length - 1">
        {{ $t('request_policies.rule_rich_summary.allof') }}
      </span>
    </template>
  </template>

  <template v-else>
    {{ unreachable(rule) }}
  </template>
</template>

<script setup lang="ts">
import { PopulatedRule } from '~/composables/request-policies.composable';
import { RequestPolicyRuleEnum, RequestPolicyRuleUserSpecifierEnum } from '~/types/station.types';
import RuleSummaryUserSpecifier from './RuleSummaryUserSpecifier.vue';
import { unreachable } from '~/utils/helper.utils';
import { mdiAlertCircle } from '@mdi/js';
import InteractiveNamedRule from '~/components/request-policies/InteractiveNamedRule.vue';
defineProps<{
  rule: PopulatedRule;
}>();
</script>
