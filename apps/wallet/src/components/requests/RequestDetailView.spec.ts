import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import RequestDetailView from './RequestDetailView.vue';

type RequestDetailViewProps = InstanceType<typeof RequestDetailView>['$props'];

const pendingProps: RequestDetailViewProps = {
  details: {
    can_approve: true,
    requester_name: 'requester',
    approvers: [{ id: 'requester-id', name: '' }],
  },
  request: {
    status: { Created: null },
    approvals: [
      {
        approver_id: 'requester-id',
        status: { Approved: null },
        decided_at: '',
        status_reason: [],
      },
    ],
    operation: {
      AddUser: {
        user: [],
        input: {
          groups: [],
          identities: [],
          name: 'test',
          status: { Active: null },
        },
      },
    },
    created_at: '',
    id: '',
    execution_plan: { Immediate: null },
    expiration_dt: '',
    requested_by: 'requester-id',
    summary: [],
    title: '',
  },
};

const approvedProps: RequestDetailViewProps = {
  details: {
    can_approve: false,
    requester_name: 'requester',
    approvers: [
      { id: 'approver-1-id', name: '' },
      { id: 'approver-2-id', name: '' },
    ],
  },
  request: {
    status: { Completed: { completed_at: '' } },
    approvals: [
      {
        approver_id: 'approver-1-id',
        status: { Approved: null },
        decided_at: '',
        status_reason: [],
      },
      {
        approver_id: 'approver-2-id',
        status: { Approved: null },
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
          name: 'test',
          status: { Active: null },
        },
      },
    },
    created_at: '',
    id: '',
    execution_plan: { Immediate: null },
    expiration_dt: '',
    requested_by: 'approver-1-id',
    summary: [],
    title: '',
  },
};

const rejectedProps: RequestDetailViewProps = {
  details: {
    can_approve: false,
    requester_name: 'requester',
    approvers: [
      { id: 'approver-1-id', name: '' },
      { id: 'approver-2-id', name: '' },
    ],
  },
  request: {
    status: { Rejected: null },
    approvals: [
      {
        approver_id: 'approver-1-id',
        status: { Approved: null },
        decided_at: '',
        status_reason: [],
      },
      {
        approver_id: 'approver-2-id',
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
          name: 'test',
          status: { Active: null },
        },
      },
    },
    created_at: '',
    id: '',
    execution_plan: { Immediate: null },
    expiration_dt: '',
    requested_by: 'approver-1-id',
    summary: [],
    title: '',
  },
};

describe('RequestDetailView', () => {
  it('renders properly', () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the comment field for the request', () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });

    expect(wrapper.find('[data-test-id="request-details-comment"]')).toBeTruthy();
  });

  it('does not show the comment field for an approved', () => {
    const wrapper = mount(RequestDetailView, {
      props: approvedProps,
    });

    expect(wrapper.find('[data-test-id="request-details-comment"]').exists()).toBeFalsy();
  });

  it('does not show the comment field for a rejected', () => {
    const wrapper = mount(RequestDetailView, {
      props: rejectedProps,
    });

    expect(wrapper.find('[data-test-id="request-details-comment"]').exists()).toBeFalsy();
  });

  it('submits the comment when filled out', () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });

    const commentEl = wrapper.find('[data-test-id="request-details-comment"] textarea');
    commentEl.setValue('test comment');

    wrapper.find('[data-test-id="request-details-approve"]').trigger('click');

    expect(wrapper.emitted().approve[0]).toEqual(['test comment']);

    wrapper.find('[data-test-id="request-details-reject"]').trigger('click');

    expect(wrapper.emitted().reject[0]).toEqual(['test comment']);
  });

  it('does not submit a comment when not filled out', () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });

    wrapper.find('[data-test-id="request-details-approve"]').trigger('click');

    expect(wrapper.emitted().approve[0]).toEqual([undefined]);
  });

  it('lists approvals for pending requests', () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });

    expect(wrapper.find('[data-test-id="request-approvals"]').html()).toContain(
      pendingProps.request.approvals[0].approver_id,
    );
  });

  it('lists approvals for approved requests', () => {
    const wrapper = mount(RequestDetailView, {
      props: approvedProps,
    });

    expect(wrapper.find('[data-test-id="request-approvals"]').html()).toContain(
      approvedProps.request.approvals[0].approver_id,
    );
  });

  it('lists approvals for rejected requests', () => {
    const wrapper = mount(RequestDetailView, {
      props: rejectedProps,
    });

    expect(wrapper.find('[data-test-id="request-approvals"]').html()).toContain(
      rejectedProps.request.approvals[0].approver_id,
    );
  });
});
