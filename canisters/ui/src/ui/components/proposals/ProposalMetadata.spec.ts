import { describe, expect, it } from 'vitest';
import { mount } from '~/ui/test.utils';
import ProposalMetadata from './ProposalMetadata.vue';
import { Proposal } from '~/generated/wallet/wallet.did';

describe('ProposalMetadata', () => {
  it('renders properly', () => {
    const wrapper = mount(ProposalMetadata, {
      props: {
        proposal: {
          info: {
            can_vote: false,
            proposer_name: [],
          },
          status: { Adopted: null },
        } as Proposal,
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('shows the expiration_dt when still pending', () => {
    const wrapper = mount(ProposalMetadata, {
      props: {
        proposal: {
          info: {
            can_vote: false,
            proposer_name: [],
          },
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
        proposal: {
          info: {
            can_vote: false,
            proposer_name: [],
          },
          expiration_dt: new Date().toISOString(),
          status: { Adopted: null },
        } as Proposal,
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-test-id="expiration-dt"]').exists()).toBe(false);
  });
});
