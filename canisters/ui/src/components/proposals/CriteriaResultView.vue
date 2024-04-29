<template>
  <VListGroup v-if="variantIs(props.evaluatedCriteria, 'And')">
    <template v-slot:activator="{ props: activatorProps }">
      <VListItem v-bind="activatorProps">
        <template #title>
          <div class="d-flex justify-space-between align-center">
            <div>{{ $t('proposals.evaluation.and_rule') }}</div>
            <div class="text-caption" :class="statusToColor(props.status)">
              {{ criteriaToLabel(props.evaluatedCriteria, props.status) }}
            </div>
          </div>
        </template>
      </VListItem>
    </template>
    <CriteriaResultView
      v-for="(item, idx) in props.evaluatedCriteria.And"
      :key="idx"
      :evaluated-criteria="item.evaluated_criteria"
      :status="item.status"
      :proposal-votes="props.proposalVotes"
    ></CriteriaResultView>
  </VListGroup>
  <VListGroup v-else-if="variantIs(props.evaluatedCriteria, 'Or')">
    <template v-slot:activator="{ props: activatorProps }">
      <VListItem v-bind="activatorProps">
        <template #title>
          <div class="d-flex justify-space-between align-center">
            <div>{{ $t('proposals.evaluation.or_rule') }}</div>
            <div class="text-caption" :class="statusToColor(props.status)">
              {{ criteriaToLabel(props.evaluatedCriteria, props.status) }}
            </div>
          </div>
        </template>
      </VListItem>
    </template>
    <CriteriaResultView
      v-for="(item, idx) in props.evaluatedCriteria.Or"
      :key="idx"
      :evaluated-criteria="item.evaluated_criteria"
      :status="item.status"
      :proposal-votes="props.proposalVotes"
    ></CriteriaResultView>
  </VListGroup>
  <VListGroup v-else-if="variantIs(props.evaluatedCriteria, 'Not')">
    <template v-slot:activator="{ props: activatorProps }">
      <VListItem v-bind="activatorProps">
        <template #title>
          <div class="d-flex justify-space-between align-center">
            <div>{{ $t('proposals.evaluation.not_rule') }}</div>
            <div class="text-caption" :class="statusToColor(props.status)">
              {{ criteriaToLabel(props.evaluatedCriteria, props.status) }}
            </div>
          </div>
        </template>
      </VListItem>
    </template>
    <CriteriaResultView
      :evaluated-criteria="props.evaluatedCriteria.Not.evaluated_criteria"
      :status="props.evaluatedCriteria.Not.status"
      :proposal-votes="props.proposalVotes"
    ></CriteriaResultView>
  </VListGroup>

  <VListItem
    v-else-if="variantIs(props.evaluatedCriteria, 'HasAddressInAddressBook')"
    :title="$t('proposals.evaluation.has_address_in_address_book')"
  >
    <template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ criteriaToLabel(props.evaluatedCriteria, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem
    v-else-if="variantIs(props.evaluatedCriteria, 'HasAddressBookMetadata')"
    :title="$t('proposals.evaluation.has_address_book_metadata')"
    :subtitle="criteriaToLabel(props.evaluatedCriteria, props.status)"
  >
    <template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ criteriaToLabel(props.evaluatedCriteria, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem
    v-else-if="variantIs(props.evaluatedCriteria, 'MinimumVotes')"
    :title="
      $t('proposals.evaluation.minimum_votes', {
        min: props.evaluatedCriteria.MinimumVotes.min_required_votes,
      })
    "
    :subtitle="criteriaToLabel(props.evaluatedCriteria, props.status)"
    ><template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ criteriaToLabel(props.evaluatedCriteria, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem
    v-else-if="variantIs(props.evaluatedCriteria, 'ApprovalThreshold')"
    :title="
      $t('proposals.evaluation.approval_threshold', {
        min: props.evaluatedCriteria.ApprovalThreshold.min_required_votes,
      })
    "
    :subtitle="criteriaToLabel(props.evaluatedCriteria, props.status)"
    ><template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ criteriaToLabel(props.evaluatedCriteria, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem
    v-else-if="variantIs(props.evaluatedCriteria, 'AutoAdopted')"
    :title="$t('proposals.evaluation.auto_adopted')"
    :subtitle="criteriaToLabel(props.evaluatedCriteria, props.status)"
    ><template #subtitle>
      <span :class="statusToColor(props.status)">
        {{ criteriaToLabel(props.evaluatedCriteria, props.status) }}
      </span>
    </template>
  </VListItem>

  <VListItem v-else>{{ unreachable(props.evaluatedCriteria) }}</VListItem>
</template>

<script lang="ts" setup>
import { EvaluatedCriteria, EvaluationStatus, ProposalVote } from '~/generated/wallet/wallet.did';
import { unreachable, variantIs } from '~/utils/helper.utils';
import { useI18n } from 'vue-i18n';
import { useDisplay } from 'vuetify';
import { VListGroup, VListItem } from 'vuetify/components';

const i18n = useI18n();
const { mobile } = useDisplay();

function itemProps(props: object): unknown {
  const result = props as {
    baseColor?: string;
    subtitle?: string;
  } & Pick<TreeViewItem, 'status' | 'title' | 'append'>;
  const status = (props as TreeViewItem).status;

  if (variantIs(status, 'Adopted')) {
    result.baseColor = 'success';
  } else if (variantIs(status, 'Rejected')) {
    result.baseColor = 'error';
  }

  if (result.append && mobile.value) {
    result.subtitle = result.append;
    delete result.append;
  }

  if (result.subtitle && !mobile.value) {
    result.append = result.subtitle;
    delete result.subtitle;
  }

  return result;
}

const props = defineProps<{
  status: EvaluationStatus;
  evaluatedCriteria: EvaluatedCriteria;
  proposalVotes: ProposalVote[];
}>();

type TreeViewItem = {
  title: string;
  status: EvaluationStatus;
  append?: string;
  children?: TreeViewItem[];
};

function statusToColor(status: EvaluationStatus): string {
  if (variantIs(status, 'Adopted')) {
    return 'text-success';
  } else if (variantIs(status, 'Rejected')) {
    return 'text-error';
  } else {
    return '';
  }
}

function getVotingSummary(voterIds: string[], status: EvaluationStatus): string {
  const votes = voterIds.map(userId => props.proposalVotes.find(vote => vote.user_id === userId));

  const approvals = votes.filter(vote => vote && variantIs(vote.status, 'Accepted')).length;
  const rejections = votes.filter(vote => vote && variantIs(vote.status, 'Rejected')).length;

  let append = '';

  if (variantIs(status, 'Adopted')) {
    append = i18n.t('proposals.evaluation.vote_summary_approved', {
      n: approvals,
      m: rejections,
    });
  } else if (variantIs(status, 'Rejected')) {
    append = i18n.t('proposals.evaluation.vote_summary_rejected', {
      n: approvals,
      m: rejections,
    });
  } else {
    append = i18n.t('proposals.evaluation.vote_summary_pending', {
      n: approvals,
      m: rejections,
    });
  }

  return append;
}

function statusToSimpleLabel(status: EvaluationStatus): string {
  if (variantIs(status, 'Adopted')) {
    return i18n.t('proposals.evaluation.approved');
  } else if (variantIs(status, 'Rejected')) {
    return i18n.t('proposals.evaluation.rejected');
  } else if (variantIs(status, 'Pending')) {
    return i18n.t('proposals.evaluation.pending');
  } else {
    return unreachable(status);
  }
}

function criteriaToLabel(criteria: EvaluatedCriteria, status: EvaluationStatus): string {
  if (variantIs(criteria, 'And')) {
    if (criteria.And.length === 1) {
      return criteriaToLabel(criteria.And[0].evaluated_criteria, criteria.And[0].status);
    }

    return statusToSimpleLabel(status);
  } else if (variantIs(criteria, 'Or')) {
    if (criteria.Or.length === 1) {
      return criteriaToLabel(criteria.Or[0].evaluated_criteria, criteria.Or[0].status);
    }
    return statusToSimpleLabel(status);
  } else if (variantIs(criteria, 'Not')) {
    if (variantIs(status, 'Adopted')) {
      return i18n.t('proposals.evaluation.approved');
    } else if (variantIs(status, 'Rejected')) {
      return i18n.t('proposals.evaluation.rejected');
    } else {
      return i18n.t('proposals.evaluation.pending');
    }
  } else if (variantIs(criteria, 'HasAddressInAddressBook')) {
    if (variantIs(status, 'Adopted')) {
      return i18n.t('proposals.evaluation.found_in_address_book');
    } else if (variantIs(status, 'Rejected')) {
      return i18n.t('proposals.evaluation.not_found_in_address_book');
    } else {
      return '';
    }
  } else if (variantIs(criteria, 'HasAddressBookMetadata')) {
    const metadata =
      criteria.HasAddressBookMetadata.metadata.key +
      '=' +
      criteria.HasAddressBookMetadata.metadata.value;

    if (variantIs(status, 'Adopted')) {
      return i18n.t('proposals.evaluation.address_book_metadata_found', {
        metadata,
      });
    } else if (variantIs(status, 'Rejected')) {
      return i18n.t('proposals.evaluation.address_book_metadata_not_found', {
        metadata,
      });
    } else {
      return '';
    }
  } else if (variantIs(criteria, 'MinimumVotes')) {
    return getVotingSummary(criteria.MinimumVotes.votes, status);
  } else if (variantIs(criteria, 'ApprovalThreshold')) {
    return getVotingSummary(criteria.ApprovalThreshold.votes, status);
  } else if (variantIs(criteria, 'AutoAdopted')) {
    return '';
  } else {
    return unreachable(criteria);
  }
}
</script>
