import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import PolicyRuleResultView from './PolicyRuleResultView.vue';

describe('PolicyRuleResultView', () => {
  it('renders properly', () => {
    const wrapper = mount(PolicyRuleResultView, {
      props: {
        evaluatedRule: {
          AutoApproved: null,
        },
        requestApprovals: [],
        status: { Approved: null },
      },
    });

    expect(wrapper.exists()).toBe(true);
  });
});
