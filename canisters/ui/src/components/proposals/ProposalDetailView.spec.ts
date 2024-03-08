import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import ProposalDetailView from './ProposalDetailView.vue';

type ProposalDetailViewProps = InstanceType<typeof ProposalDetailView>['$props'];

const pendingProps: ProposalDetailViewProps = {
  details: {
    can_vote: true,
    proposer_name: undefined,
    voters: [{ id: 'proposer-id', name: [] }],
  },
  proposal: {
    status: { Created: null },
    votes: [
      { user_id: 'proposer-id', status: { Accepted: null }, decided_at: '', status_reason: [] },
    ],
    operation: {
      AddUser: {
        user: [],
        input: {
          groups: [],
          identities: [],
          name: ['test'],
          status: { Active: null },
        },
      },
    },
    created_at: '',
    id: '',
    execution_plan: { Immediate: null },
    expiration_dt: '',
    proposed_by: 'proposer-id',
    summary: [],
    title: '',
  },
};

const approvedProps: ProposalDetailViewProps = {
  details: {
    can_vote: false,
    proposer_name: undefined,
    voters: [
      { id: 'voter-1-id', name: [] },
      { id: 'voter-2-id', name: [] },
    ],
  },
  proposal: {
    status: { Completed: { completed_at: '' } },
    votes: [
      { user_id: 'voter-1-id', status: { Accepted: null }, decided_at: '', status_reason: [] },
      {
        user_id: 'voter-2-id',
        status: { Accepted: null },
        decided_at: '',
        status_reason: ['Test comment'],
      },
    ],
    operation: {
      AddUser: {
        user: [],
        input: {
          groups: [],
          identities: [],
          name: ['test'],
          status: { Active: null },
        },
      },
    },
    created_at: '',
    id: '',
    execution_plan: { Immediate: null },
    expiration_dt: '',
    proposed_by: 'voter-1-id',
    summary: [],
    title: '',
  },
};

const rejectedProps: ProposalDetailViewProps = {
  details: {
    can_vote: false,
    proposer_name: undefined,
    voters: [
      { id: 'voter-1-id', name: [] },
      { id: 'voter-2-id', name: [] },
    ],
  },
  proposal: {
    status: { Rejected: null },
    votes: [
      { user_id: 'voter-1-id', status: { Accepted: null }, decided_at: '', status_reason: [] },
      {
        user_id: 'voter-2-id',
        status: { Rejected: null },
        decided_at: '',
        status_reason: ['Test comment'],
      },
    ],
    operation: {
      AddUser: {
        user: [],
        input: {
          groups: [],
          identities: [],
          name: ['test'],
          status: { Active: null },
        },
      },
    },
    created_at: '',
    id: '',
    execution_plan: { Immediate: null },
    expiration_dt: '',
    proposed_by: 'voter-1-id',
    summary: [],
    title: '',
  },
};

describe('ProposalDetailView', () => {
  it('renders properly', () => {
    const wrapper = mount(ProposalDetailView, {
      props: pendingProps,
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the comment field for the proposal', () => {
    const wrapper = mount(ProposalDetailView, {
      props: pendingProps,
    });

    expect(wrapper.find('[data-test-id="proposal-details-comment"]')).toBeTruthy();
  });

  it('does not show the comment field for an approved', () => {
    const wrapper = mount(ProposalDetailView, {
      props: approvedProps,
    });

    expect(wrapper.find('[data-test-id="proposal-details-comment"]').exists()).toBeFalsy();
  });

  it('does not show the comment field for a rejected', () => {
    const wrapper = mount(ProposalDetailView, {
      props: rejectedProps,
    });

    expect(wrapper.find('[data-test-id="proposal-details-comment"]').exists()).toBeFalsy();
  });

  it('submits the comment when filled out', () => {
    const wrapper = mount(ProposalDetailView, {
      props: pendingProps,
    });

    const commentEl = wrapper.find('[data-test-id="proposal-details-comment"] textarea');
    commentEl.setValue('test comment');

    wrapper.find('[data-test-id="proposal-details-approve"]').trigger('click');

    expect(wrapper.emitted().approve[0]).toEqual(['test comment']);

    wrapper.find('[data-test-id="proposal-details-reject"]').trigger('click');

    expect(wrapper.emitted().reject[0]).toEqual(['test comment']);
  });

  it('does not submit a comment when not filled out', () => {
    const wrapper = mount(ProposalDetailView, {
      props: pendingProps,
    });

    wrapper.find('[data-test-id="proposal-details-approve"]').trigger('click');

    expect(wrapper.emitted().approve[0]).toEqual([undefined]);
  });

  it('lists votes for pending proposals', () => {
    const wrapper = mount(ProposalDetailView, {
      props: pendingProps,
    });

    expect(wrapper.find('[data-test-id="proposal-votes"]').html()).toContain(
      pendingProps.proposal.votes[0].user_id,
    );
  });

  it('lists votes for approved proposals', () => {
    const wrapper = mount(ProposalDetailView, {
      props: approvedProps,
    });

    expect(wrapper.find('[data-test-id="proposal-votes"]').html()).toContain(
      approvedProps.proposal.votes[0].user_id,
    );
  });

  it('lists votes for rejected proposals', () => {
    const wrapper = mount(ProposalDetailView, {
      props: rejectedProps,
    });

    expect(wrapper.find('[data-test-id="proposal-votes"]').html()).toContain(
      rejectedProps.proposal.votes[0].user_id,
    );
  });
});
