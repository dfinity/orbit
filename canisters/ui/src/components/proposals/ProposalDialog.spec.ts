import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { VCard } from 'vuetify/components';
import {
  GetProposalResult,
  GetProposalResultData,
  ProposalOperation,
  ProposalVote,
} from '~/generated/wallet/wallet.did';
import { services } from '~/plugins/services.plugin';
import { mount } from '~/test.utils';
import { ExtractOk } from '~/types/helper.types';
import ProposalDialog from './ProposalDialog.vue';

const transferOperation1 = {
  Transfer: {
    from_account: [
      {
        address: 'fromaddress1',
      },
    ],
    input: {
      to: 'toaddress1',
    },
  },
} as ProposalOperation;

const transferOperation2 = {
  Transfer: {
    from_account: [
      {
        address: 'fromaddress2',
      },
    ],
    input: {
      to: 'toaddress2',
    },
  },
} as ProposalOperation;

const votableProposalResponse = {
  privileges: { can_vote: true },
  proposal: {
    id: 'first-id',
    status: { Created: null },
    operation: transferOperation1,
    votes: [] as ProposalVote[],
  },
  additional_info: {
    id: 'proposerid',
    proposer_name: [],
  },
  //...
} as ExtractOk<GetProposalResult>;

const nextVotableProposalResponse = {
  privileges: { can_vote: true },
  proposal: {
    id: 'next-id',
    status: { Created: null },
    operation: transferOperation2,
    votes: [] as ProposalVote[],
  },
  additional_info: {
    id: 'next-id',
    proposer_name: [],
  },
  //...
} as GetProposalResultData;

const completedProposalResponse = {
  privileges: { can_vote: false },
  proposal: {
    id: 'first-id',
    votes: [] as ProposalVote[],
    status: { Completed: { completed_at: '' } },
    operation: transferOperation1,
  },
  additional_info: {
    id: 'first-id',
    proposer_name: [],
  },
  //...
} as ExtractOk<GetProposalResult>;

describe('ProposalDialog', () => {
  it('renders properly', () => {
    const wrapper = mount(ProposalDialog, {
      props: {
        proposalId: '123',
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('loads the proposal passed in as prop', async () => {
    vi.spyOn(services().wallet, 'getProposal').mockResolvedValueOnce(votableProposalResponse);

    mount(ProposalDialog, {
      props: {
        proposalId: '123',
        open: true,
      },
    });

    expect(services().wallet.getProposal).toHaveBeenCalledWith(
      {
        proposal_id: '123',
      },
      true,
    );
    vi.restoreAllMocks();
  });

  it('displays a switch to load next when the proposal can be voted on', async () => {
    vi.spyOn(services().wallet, 'getProposal').mockResolvedValueOnce(votableProposalResponse);

    const wrapper = mount(ProposalDialog, {
      props: {
        proposalId: '123',
        open: true,
      },
    });

    await flushPromises();

    const contents = wrapper.findComponent(VCard);

    expect(contents.find('[data-test-id="load-next-proposal-switch"]').exists()).toBeTruthy();
    vi.restoreAllMocks();
  });

  it('does not display a switch to load next when the proposal cannot be voted on', async () => {
    // services().wallet.getProposal = vi.fn(() => Promise.resolve(completedProposalResponse));
    vi.spyOn(services().wallet, 'getProposal').mockResolvedValueOnce(completedProposalResponse);

    const wrapper = mount(ProposalDialog, {
      props: {
        proposalId: '123',
        open: true,
      },
    });

    await flushPromises();

    const contents = wrapper.findComponent(VCard);

    expect(contents.find('[data-test-id="load-next-proposal-switch"]').exists()).toBeFalsy();

    vi.restoreAllMocks();
  });

  it('does not load the next proposal after voting when the switch is not turned on', async () => {
    vi.spyOn(services().wallet, 'getProposal').mockResolvedValueOnce(votableProposalResponse);
    vi.spyOn(services().wallet, 'voteOnProposal').mockResolvedValueOnce(
      completedProposalResponse.proposal,
    );

    const wrapper = mount(ProposalDialog, {
      props: {
        proposalId: '123',
        open: true,
      },
    });

    await flushPromises();
    const contents = wrapper.findComponent(VCard);

    const approveBtn = contents.find('[data-test-id="proposal-details-approve"]');
    await approveBtn.trigger('click');

    expect(wrapper.emitted('voted')).toBeTruthy();
    expect(wrapper.emitted('closed')).toBeTruthy();

    vi.restoreAllMocks();
  });

  it('loads the next proposal after voting when the switch is turned on', async () => {
    vi.spyOn(services().wallet, 'getProposal').mockResolvedValueOnce(votableProposalResponse);
    vi.spyOn(services().wallet, 'voteOnProposal').mockResolvedValueOnce(
      completedProposalResponse.proposal,
    );
    vi.spyOn(services().wallet, 'getNextVotableProposal').mockResolvedValueOnce([
      nextVotableProposalResponse,
    ]);

    const wrapper = mount(ProposalDialog, {
      props: {
        proposalId: '123',
        open: true,
      },
    });

    await flushPromises();
    let contents = wrapper.findComponent(VCard);

    expect(
      contents.find<HTMLInputElement>('[data-test-id="transfer-form-destination-address"] input')
        .element.value,
    ).toBe('toaddress1');

    await contents.find('[data-test-id="load-next-proposal-switch"] input').trigger('click');
    await contents.find('[data-test-id="proposal-details-approve"]').trigger('click');

    expect(wrapper.emitted('voted')).toBeFalsy();
    expect(wrapper.emitted('closed')).toBeFalsy();

    await flushPromises();

    expect(services().wallet.getNextVotableProposal).toHaveBeenCalled();

    contents = wrapper.findComponent(VCard);

    expect(
      contents.find<HTMLInputElement>('[data-test-id="transfer-form-destination-address"] input')
        .element.value,
    ).toBe('toaddress2');

    vi.restoreAllMocks();
  });

  it('shows a message when there are no votable proposals left', async () => {
    vi.spyOn(services().wallet, 'getProposal').mockResolvedValueOnce(votableProposalResponse);
    vi.spyOn(services().wallet, 'voteOnProposal').mockResolvedValueOnce(
      completedProposalResponse.proposal,
    );
    vi.spyOn(services().wallet, 'getNextVotableProposal').mockResolvedValueOnce([]);

    const wrapper = mount(ProposalDialog, {
      props: {
        proposalId: '123',
        open: true,
      },
    });

    await flushPromises();
    let contents = wrapper.findComponent(VCard);

    await contents.find('[data-test-id="load-next-proposal-switch"] input').trigger('click');
    await contents.find('[data-test-id="proposal-details-approve"]').trigger('click');

    await flushPromises();

    contents = wrapper.findComponent(VCard);

    expect(contents.find('[data-test-id="no-more-proposals"]').exists()).toBeTruthy();

    vi.restoreAllMocks();
  });
});
