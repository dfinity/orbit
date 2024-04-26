<template>
  <VTreeview
    class="text-caption"
    :items="[items]"
    :open-all="true"
    :item-props="itemProps"
    :density="'compact'"
  >
    <template #append="{ item }">
      <template v-if="item.append">{{ item.append }}</template>
    </template>
  </VTreeview>
</template>

<script lang="ts" setup>
import { EvaluatedCriteria, EvaluationStatus, ProposalVote } from '~/generated/wallet/wallet.did';
import { unreachable, variantIs } from '~/utils/helper.utils';
import { computed } from 'vue';
import { VTreeview } from 'vuetify/labs/components';
import { useI18n } from 'vue-i18n';
import { useDisplay } from 'vuetify';

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

function criteriaToTreeViewItem(
  criteria: EvaluatedCriteria,
  status: EvaluationStatus,
): TreeViewItem {
  if (variantIs(criteria, 'And')) {
    if (criteria.And.length === 1) {
      return criteriaToTreeViewItem(criteria.And[0].evaluated_criteria, criteria.And[0].status);
    }

    return {
      title: i18n.t('proposals.evaluation.and_rule', { n: criteria.And.length }),
      status,
      append: statusToSimpleLabel(status),
      children: criteria.And.map(item =>
        criteriaToTreeViewItem(item.evaluated_criteria, item.status),
      ),
    };
  } else if (variantIs(criteria, 'Or')) {
    if (criteria.Or.length === 1) {
      return criteriaToTreeViewItem(criteria.Or[0].evaluated_criteria, criteria.Or[0].status);
    }
    return {
      title: i18n.t('proposals.evaluation.or_rule', { n: criteria.Or.length }),
      status,
      append: statusToSimpleLabel(status),

      children: criteria.Or.map(item =>
        criteriaToTreeViewItem(item.evaluated_criteria, item.status),
      ),
    };
  } else if (variantIs(criteria, 'Not')) {
    return {
      title: i18n.t('proposals.evaluation.not_rule'),
      status,
      children: [criteriaToTreeViewItem(criteria.Not.evaluated_criteria, criteria.Not.status)],
    };
  } else if (variantIs(criteria, 'HasAddressInAddressBook')) {
    let append: string | undefined = undefined;

    if (variantIs(status, 'Adopted')) {
      append = i18n.t('proposals.evaluation.found_in_address_book');
    } else if (variantIs(status, 'Rejected')) {
      append = i18n.t('proposals.evaluation.not_found_in_address_book');
    }

    return {
      title: i18n.t('proposals.evaluation.has_address_in_address_book'),
      status,
      append,
    };
  } else if (variantIs(criteria, 'HasAddressBookMetadata')) {
    let append: string | undefined;
    const metadata =
      criteria.HasAddressBookMetadata.metadata.key +
      '=' +
      criteria.HasAddressBookMetadata.metadata.value;

    if (variantIs(status, 'Adopted')) {
      append = i18n.t('proposals.evaluation.address_book_metadata_found', {
        metadata,
      });
    } else if (variantIs(status, 'Rejected')) {
      append = i18n.t('proposals.evaluation.address_book_metadata_not_found', {
        metadata,
      });
    }

    return {
      title: i18n.t('proposals.evaluation.has_address_book_metadata'),
      status,
      append,
    };
  } else if (variantIs(criteria, 'MinimumVotes')) {
    const append = getVotingSummary(criteria.MinimumVotes.votes, status);

    return {
      title: i18n.t('proposals.evaluation.minimum_votes', {
        n: criteria.MinimumVotes.min_required_votes,
      }),
      status,
      append,
    };
  } else if (variantIs(criteria, 'ApprovalThreshold')) {
    const append = getVotingSummary(criteria.ApprovalThreshold.votes, status);
    return {
      title: i18n.t('proposals.evaluation.approval_threshold', {
        n: criteria.ApprovalThreshold.min_required_votes,
      }),
      status,
      append,
    };
  } else if (variantIs(criteria, 'AutoAdopted')) {
    return {
      title: i18n.t('proposals.evaluation.auto_adopted'),
      status,
    };
  } else {
    return unreachable(criteria);
  }
}

const items = computed(() => criteriaToTreeViewItem(props.evaluatedCriteria, props.status));
</script>
