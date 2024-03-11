import { describe, expect, it } from 'vitest';
import { Proposal } from '~/generated/wallet/wallet.did';
import { mount } from '~/test.utils';
import ProposalMetadata from './ProposalMetadata.vue';

describe('ProposalMetadata', () => {
  it('renders properly', () => {
    const wrapper = mount(ProposalMetadata, {
      props: {
        details: { can_vote: false, proposer_name: undefined, voters: [] },
        proposal: {
          status: { Adopted: null },
        } as Proposal,
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the expiration_dt when still pending', () => {
    const wrapper = mount(ProposalMetadata, {
      props: {
        details: { can_vote: false, proposer_name: undefined, voters: [] },
        proposal: {
          expiration_dt: new Date().toISOString(),
          status: { Created: null },
        } as Proposal,
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-test-id="expiration-dt"]').exists()).toBe(true);
  });

  it('hides the expiration_dt when not pending', () => {
    const wrapper = mount(ProposalMetadata, {
      props: {
        details: { can_vote: false, proposer_name: undefined, voters: [] },
        proposal: {
          expiration_dt: new Date().toISOString(),
          status: { Adopted: null },
        } as Proposal,
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-test-id="expiration-dt"]').exists()).toBe(false);
  });
});
