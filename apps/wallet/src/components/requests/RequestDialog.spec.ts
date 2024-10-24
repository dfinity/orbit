import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { VCard } from 'vuetify/components';
import {
  GetRequestResult,
  GetRequestResultData,
  RequestOperation,
  RequestApproval,
  Asset,
} from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { mount } from '~/test.utils';
import { ExtractOk } from '~/types/helper.types';
import RequestDialog from './RequestDialog.vue';

const mockAsset: Asset = {
  blockchain: 'icp',
  decimals: 2,
  id: '1',
  metadata: [],
  name: 'ICP',
  symbol: 'ICP',
  standards: ['icp_native', 'icrc1'],
};

const transferOperation1 = {
  Transfer: {
    from_account: [
      {
        addresses: [{ address: 'fromaddress1' }],
      },
    ],
    input: {
      to: 'toaddress1',
    },
    from_asset: mockAsset,
  },
} as RequestOperation;

const transferOperation2 = {
  Transfer: {
    from_account: [
      {
        addresses: [{ address: 'fromaddress2' }],
      },
    ],
    input: {
      to: 'toaddress2',
    },
    from_asset: mockAsset,
  },
} as RequestOperation;

const approvableRequestResponse = {
  privileges: { can_approve: true },
  request: {
    id: 'first-id',
    status: { Created: null },
    operation: transferOperation1,
    approvals: [] as RequestApproval[],
  },
  additional_info: {
    id: 'requesterid',
    requester_name: '',
    evaluation_result: [],
  },
  //...
} as ExtractOk<GetRequestResult>;

const nextApprovableRequestResponse = {
  privileges: { can_approve: true },
  request: {
    id: 'next-id',
    status: { Created: null },
    operation: transferOperation2,
    approvals: [] as RequestApproval[],
  },
  additional_info: {
    id: 'next-id',
    requester_name: '',
    evaluation_result: [],
  },
  //...
} as GetRequestResultData;

const completedRequestResponse = {
  privileges: { can_approve: false },
  request: {
    id: 'first-id',
    approvals: [] as RequestApproval[],
    status: { Completed: { completed_at: '' } },
    operation: transferOperation1,
  },
  additional_info: {
    id: 'first-id',
    requester_name: '',
    evaluation_result: [],
  },
  //...
} as ExtractOk<GetRequestResult>;

describe('RequestDialog', () => {
  it('renders properly', () => {
    const wrapper = mount(RequestDialog, {
      props: {
        requestId: '123',
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('loads the request passed in as prop', async () => {
    vi.spyOn(services().station, 'getRequest').mockResolvedValueOnce(approvableRequestResponse);

    mount(RequestDialog, {
      props: {
        requestId: '123',
        open: true,
      },
    });

    expect(services().station.getRequest).toHaveBeenCalledWith(
      {
        request_id: '123',
        with_full_info: [],
      },
      true,
    );
    vi.restoreAllMocks();
  });

  it('displays a switch to load next when the request can be approved on', async () => {
    vi.spyOn(services().station, 'getRequest').mockResolvedValueOnce(approvableRequestResponse);

    const wrapper = mount(RequestDialog, {
      props: {
        requestId: '123',
        open: true,
      },
    });

    await flushPromises();

    const contents = wrapper.findComponent(VCard);

    expect(contents.find('[data-test-id="load-next-request-switch"]').exists()).toBeTruthy();
    vi.restoreAllMocks();
  });

  it('does not display a switch to load next when the request cannot be approved on', async () => {
    // services().station.getRequest = vi.fn(() => Promise.resolve(completedRequestResponse));
    vi.spyOn(services().station, 'getRequest').mockResolvedValueOnce(completedRequestResponse);

    const wrapper = mount(RequestDialog, {
      props: {
        requestId: '123',
        open: true,
      },
    });

    await flushPromises();

    const contents = wrapper.findComponent(VCard);

    expect(contents.find('[data-test-id="load-next-request-switch"]').exists()).toBeFalsy();

    vi.restoreAllMocks();
  });

  it('does not load the next request after approving when the switch is not turned on', async () => {
    vi.spyOn(services().station, 'getRequest').mockResolvedValueOnce(approvableRequestResponse);
    vi.spyOn(services().station, 'submitRequestApproval').mockResolvedValueOnce(
      completedRequestResponse.request,
    );

    const wrapper = mount(RequestDialog, {
      props: {
        requestId: '123',
        open: true,
      },
    });

    await flushPromises();
    const contents = wrapper.findComponent(VCard);

    const approveBtn = contents.find('[data-test-id="request-details-approve"]');
    await approveBtn.trigger('click');

    expect(wrapper.emitted('approved')).toBeTruthy();
    expect(wrapper.emitted('closed')).toBeTruthy();

    vi.restoreAllMocks();
  });

  it('loads the next request after approving when the switch is turned on', async () => {
    vi.spyOn(services().station, 'getRequest').mockResolvedValueOnce(approvableRequestResponse);
    vi.spyOn(services().station, 'submitRequestApproval').mockResolvedValueOnce(
      completedRequestResponse.request,
    );
    vi.spyOn(services().station, 'getNextApprovableRequest').mockResolvedValueOnce([
      nextApprovableRequestResponse,
    ]);

    const wrapper = mount(RequestDialog, {
      props: {
        requestId: '123',
        open: true,
      },
    });

    await flushPromises();
    let contents = wrapper.findComponent(VCard);

    expect(
      contents.find<HTMLInputElement>('[data-test-id="transfer-form-destination-address"] input')
        .element.value,
    ).toBe('toaddress1');

    await contents.find('[data-test-id="load-next-request-switch"] input').trigger('click');
    await contents.find('[data-test-id="request-details-approve"]').trigger('click');

    expect(wrapper.emitted('approved')).toBeFalsy();
    expect(wrapper.emitted('closed')).toBeFalsy();

    await flushPromises();

    expect(services().station.getNextApprovableRequest).toHaveBeenCalled();

    contents = wrapper.findComponent(VCard);

    expect(
      contents.find<HTMLInputElement>('[data-test-id="transfer-form-destination-address"] input')
        .element.value,
    ).toBe('toaddress2');

    vi.restoreAllMocks();
  });

  it('shows a message when there are no approvable requests left', async () => {
    vi.spyOn(services().station, 'getRequest').mockResolvedValueOnce(approvableRequestResponse);
    vi.spyOn(services().station, 'submitRequestApproval').mockResolvedValueOnce(
      completedRequestResponse.request,
    );
    vi.spyOn(services().station, 'getNextApprovableRequest').mockResolvedValueOnce([]);

    const wrapper = mount(RequestDialog, {
      props: {
        requestId: '123',
        open: true,
      },
    });

    await flushPromises();
    let contents = wrapper.findComponent(VCard);

    await contents.find('[data-test-id="load-next-request-switch"] input').trigger('click');
    await contents.find('[data-test-id="request-details-approve"]').trigger('click');

    await flushPromises();

    contents = wrapper.findComponent(VCard);

    expect(contents.find('[data-test-id="no-more-requests"]').exists()).toBeTruthy();

    vi.restoreAllMocks();
  });
});
