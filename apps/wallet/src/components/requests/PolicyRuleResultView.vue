<template>
  <VListGroup v-if="variantIs(props.evaluatedRule, 'AllOf')">
    <template v-slot:activator="{ props: activatorProps }">
      <VListItem v-bind="activatorProps">
        <template #title>
          <div class="d-flex justify-space-between align-center">
            <div>{{ $t('requests.evaluation.and_rule') }}</div>
            <div class="text-caption" :class="statusToColor(props.status)">
              {{ ruleToLabel(props.evaluatedRule, props.status) }}
            </div>
          </div>
        </template>
      </VListItem>
    </template>
    <CriteriaResultView
      v-for="(item, idx) in props.evaluatedRule.AllOf"
      :key="idx"
      :evaluatedRule="item.evaluated_rule"
      :status="item.status"
      :requestApprovals="props.requestApprovals"
    ></CriteriaResultView>
  </VListGroup>
  <VListGroup v-else-if="variantIs(props.evaluatedRule, 'AnyOf')">
    <template v-slot:activator="{ props: activatorProps }">
      <VListItem v-bind="activatorProps">
        <template #title>
          <div class="d-flex justify-space-between align-center">
            <div>{{ $t('requests.evaluation.or_rule') }}</div>
            <div class="text-caption" :class="statusToColor(props.status)">
              {{ ruleToLabel(props.evaluatedRule, props.status) }}
            </div>
          </div>
        </template>
      </VListItem>
    </template>
    <CriteriaResultView
      v-for="(item, idx) in props.evaluatedRule.AnyOf"
      :key="idx"
      :evaluatedRule="item.evaluated_rule"
      :status="item.status"
      :requestApprovals="props.requestApprovals"
    ></CriteriaResultView>
  </VListGroup>
  <VListGroup v-else-if="variantIs(props.evaluatedRule, 'Not')">
    <template v-slot:activator="{ props: activatorProps }">
      <VListItem v-bind="activatorProps">
        <template #title>
          <div class="d-flex justify-space-between align-center">
            <div>{{ $t('requests.evaluation.not_rule') }}</div>
            <div class="text-caption" :class="statusToColor(props.status)">
              {{ ruleToLabel(props.evaluatedRule, props.status) }}
            </div>
          </div>
        </template>
      </VListItem>
    </template>
    <CriteriaResultView
      :evaluatedRule="props.evaluatedRule.Not.evaluated_rule"
      :status="props.evaluatedRule.Not.status"
      :requestApprovals="props.requestApprovals"
    ></CriteriaResultView>
  </VListGroup>

  <VListItem
    v-else-if="variantIs(props.evaluatedRule, 'AllowListed')"
    :title="$t('requests.evaluation.allowlisted_rule')"
  >
    <template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ ruleToLabel(props.evaluatedRule, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem
    v-else-if="variantIs(props.evaluatedRule, 'AllowListedByMetadata')"
    :title="$t('requests.evaluation.allowlisted_with_metadata_rule')"
    :subtitle="ruleToLabel(props.evaluatedRule, props.status)"
  >
    <template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ ruleToLabel(props.evaluatedRule, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem
    v-else-if="variantIs(props.evaluatedRule, 'Quorum')"
    :title="
      $t('requests.evaluation.quorum_rule', {
        min: props.evaluatedRule.Quorum.min_approved,
      })
    "
    :subtitle="ruleToLabel(props.evaluatedRule, props.status)"
    ><template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ ruleToLabel(props.evaluatedRule, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem
    v-else-if="variantIs(props.evaluatedRule, 'QuorumPercentage')"
    :title="
      $t('requests.evaluation.quorum_percentage_rule', {
        min: props.evaluatedRule.QuorumPercentage.min_approved,
      })
    "
    :subtitle="ruleToLabel(props.evaluatedRule, props.status)"
    ><template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ ruleToLabel(props.evaluatedRule, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem
    v-else-if="variantIs(props.evaluatedRule, 'AutoApproved')"
    :title="$t('requests.evaluation.auto_approved')"
    :subtitle="ruleToLabel(props.evaluatedRule, props.status)"
    ><template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ ruleToLabel(props.evaluatedRule, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem v-else>{{ unreachable(props.evaluatedRule) }}</VListItem>
</template>

<script lang="ts" setup>
import {
  EvaluatedRequestPolicyRule,
  EvaluationStatus,
  RequestApproval,
} from '~/generated/station/station.did';
import { unreachable, variantIs } from '~/utils/helper.utils';
import { useI18n } from 'vue-i18n';
import { VListGroup, VListItem } from 'vuetify/components';

const i18n = useI18n();

const props = defineProps<{
  status: EvaluationStatus;
  evaluatedRule: EvaluatedRequestPolicyRule;
  requestApprovals: RequestApproval[];
}>();

function statusToColor(status: EvaluationStatus): string {
  if (variantIs(status, 'Approved')) {
    return 'text-success';
  } else if (variantIs(status, 'Rejected')) {
    return 'text-error';
  } else {
    return '';
  }
}

function getApprovalSummary(approverIds: string[], status: EvaluationStatus): string {
  const allApprovals = approverIds.map(userId =>
    props.requestApprovals.find(approval => approval.approver_id === userId),
  );

  const approvals = allApprovals.filter(
    approval => approval && variantIs(approval.status, 'Approved'),
  ).length;
  const rejections = allApprovals.filter(
    approval => approval && variantIs(approval.status, 'Rejected'),
  ).length;

  let append = '';

  if (variantIs(status, 'Approved')) {
    append = i18n.t('requests.evaluation.approval_summary_approved', {
      n: approvals,
      m: rejections,
    });
  } else if (variantIs(status, 'Rejected')) {
    append = i18n.t('requests.evaluation.approval_summary_rejected', {
      n: approvals,
      m: rejections,
    });
  } else {
    append = i18n.t('requests.evaluation.approval_summary_pending', {
      n: approvals,
      m: rejections,
    });
  }

  return append;
}

function statusToSimpleLabel(status: EvaluationStatus): string {
  if (variantIs(status, 'Approved')) {
    return i18n.t('requests.evaluation.approved');
  } else if (variantIs(status, 'Rejected')) {
    return i18n.t('requests.evaluation.rejected');
  } else if (variantIs(status, 'Pending')) {
    return i18n.t('requests.evaluation.pending');
  } else {
    return unreachable(status);
  }
}

function ruleToLabel(rule: EvaluatedRequestPolicyRule, status: EvaluationStatus): string {
  if (variantIs(rule, 'AllOf')) {
    if (rule.AllOf.length === 1) {
      return ruleToLabel(rule.AllOf[0].evaluated_rule, rule.AllOf[0].status);
    }

    return statusToSimpleLabel(status);
  } else if (variantIs(rule, 'AnyOf')) {
    if (rule.AnyOf.length === 1) {
      return ruleToLabel(rule.AnyOf[0].evaluated_rule, rule.AnyOf[0].status);
    }
    return statusToSimpleLabel(status);
  } else if (variantIs(rule, 'Not')) {
    if (variantIs(status, 'Approved')) {
      return i18n.t('requests.evaluation.approved');
    } else if (variantIs(status, 'Rejected')) {
      return i18n.t('requests.evaluation.rejected');
    } else {
      return i18n.t('requests.evaluation.pending');
    }
  } else if (variantIs(rule, 'AllowListed')) {
    if (variantIs(status, 'Approved')) {
      return i18n.t('requests.evaluation.found_in_allow_list');
    } else if (variantIs(status, 'Rejected')) {
      return i18n.t('requests.evaluation.not_found_in_allow_list');
    } else {
      return '';
    }
  } else if (variantIs(rule, 'AllowListedByMetadata')) {
    const metadata =
      rule.AllowListedByMetadata.metadata.key + '=' + rule.AllowListedByMetadata.metadata.value;

    if (variantIs(status, 'Approved')) {
      return i18n.t('requests.evaluation.allow_list_metadata_found', {
        metadata,
      });
    } else if (variantIs(status, 'Rejected')) {
      return i18n.t('requests.evaluation.allow_list_metadata_not_found', {
        metadata,
      });
    } else {
      return '';
    }
  } else if (variantIs(rule, 'Quorum')) {
    return getApprovalSummary(rule.Quorum.approvers, status);
  } else if (variantIs(rule, 'QuorumPercentage')) {
    return getApprovalSummary(rule.QuorumPercentage.approvers, status);
  } else if (variantIs(rule, 'AutoApproved')) {
    return '';
  } else {
    return unreachable(rule);
  }
}
</script>
