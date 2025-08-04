import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import RequestDetailView from './RequestDetailView.vue';
import { variantIs } from '~/utils/helper.utils';
import { useStationStore } from '~/stores/station.store';
import { flushPromises } from '@vue/test-utils';

type RequestDetailViewProps = InstanceType<typeof RequestDetailView>['$props'];

const pendingProps: RequestDetailViewProps = {
  details: {
    can_approve: true,
    requester_name: 'requester',
    approvers: [{ id: 'requester-id', name: 'TestApprover' }],
    evaluationResult: {
      request_id: 'request-id',
      status: { Pending: null },
      policy_results: [
        {
          status: { Pending: null },
          evaluated_rule: {
            AutoApproved: null,
          },
        },
      ],
      result_reasons: [],
    },
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
    tags: [],
    title: '',
    deduplication_key: [],
  },
};

const approvedProps: RequestDetailViewProps = {
  details: {
    can_approve: false,
    requester_name: 'requester',
    approvers: [
      { id: 'approver-1-id', name: 'Approver1' },
      { id: 'approver-2-id', name: 'Approver2' },
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
    tags: [],
    title: '',
    deduplication_key: [],
  },
};

const rejectedProps: RequestDetailViewProps = {
  details: {
    can_approve: false,
    requester_name: 'requester',
    approvers: [
      { id: 'approver-1-id', name: 'Approver1' },
      { id: 'approver-2-id', name: 'Approver2' },
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
    tags: [],
    title: '',
    deduplication_key: [],
  },
};
const failedProps: RequestDetailViewProps = {
  details: {
    can_approve: false,
    requester_name: 'requester',
    approvers: [
      { id: 'approver-1-id', name: '' },
      { id: 'approver-2-id', name: '' },
    ],
  },
  request: {
    status: { Failed: { reason: ['test-reason'] } },
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
    tags: [],
    title: '',
    deduplication_key: [],
  },
};

const cancelledProps: RequestDetailViewProps = {
  details: {
    can_approve: false,
    requester_name: 'requester',
    approvers: [
      { id: 'approver-1-id', name: '' },
      { id: 'approver-2-id', name: '' },
    ],
  },
  request: {
    status: { Cancelled: { reason: ['cancellation reason'] } },
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
    tags: [],
    title: '',
    deduplication_key: [],
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

  it('lists approvals for pending requests', async () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });

    await wrapper.find('[data-test-id="request-approvals-and-evaluation"] button').trigger('click');

    expect(wrapper.find('[data-test-id="request-approvals"]').html()).toContain(
      pendingProps.details.approvers[0].name,
    );
  });

  it('lists approvals for approved requests', async () => {
    const wrapper = mount(RequestDetailView, {
      props: approvedProps,
    });

    await wrapper.find('[data-test-id="request-approvals-and-evaluation"] button').trigger('click');

    expect(wrapper.find('[data-test-id="request-approvals"]').html()).toContain(
      approvedProps.details.approvers[0].name,
    );
  });

  it('lists approvals for rejected requests', async () => {
    const wrapper = mount(RequestDetailView, {
      props: rejectedProps,
    });

    await wrapper.find('[data-test-id="request-approvals-and-evaluation"] button').trigger('click');

    expect(wrapper.find('[data-test-id="request-approvals"]').html()).toContain(
      rejectedProps.details.approvers[0].name,
    );
  });

  it('shows failure reason for failed requests', () => {
    const wrapper = mount(RequestDetailView, {
      props: failedProps,
    });

    const failed = variantIs(failedProps.request.status, 'Failed')
      ? failedProps.request.status.Failed
      : null;
    expect(failed).toBeTruthy();

    expect(wrapper.find('[data-test-id="request-details-failure"]').html()).toContain(
      failed!.reason[0],
    );
  });

  it('shows acceptance rules', async () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });

    await wrapper.find('[data-test-id="request-approvals-and-evaluation"] button').trigger('click');

    expect(wrapper.find('[data-test-id="request-acceptance-rules"]').exists()).toBeTruthy();
  });

  it('shows a cancel button for cancellable requests', async () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });
    const station = useStationStore();
    station.$patch({
      user: { id: 'requester-id' },
    });
    await flushPromises();
    expect(wrapper.find('[data-test-id="request-details-cancel"]').exists()).toBeTruthy();

    await wrapper.find('[data-test-id="request-details-cancel"]').trigger('click');
    expect(wrapper.emitted().cancel).toBeTruthy();
  });

  it("doesn't show cancel button for cancelled requests", async () => {
    const wrapper = mount(RequestDetailView, {
      props: cancelledProps,
    });
    const station = useStationStore();
    station.$patch({
      user: { id: 'requester-id' },
    });

    await flushPromises();

    expect(wrapper.find('[data-test-id="request-details-cancel"]').exists()).toBeFalsy();
  });

  it("doesn't show cancel button for non-requester", async () => {
    const wrapper = mount(RequestDetailView, {
      props: pendingProps,
    });
    const station = useStationStore();
    station.$patch({
      user: { id: 'not-requester-id' },
    });

    await flushPromises();
    expect(wrapper.find('[data-test-id="request-details-cancel"]').exists()).toBeFalsy();
  });
});
