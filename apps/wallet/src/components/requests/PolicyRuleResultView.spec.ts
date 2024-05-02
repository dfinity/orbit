import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import PolicyRuleResultView from './PolicyRuleResultView.vue';
import { EvaluatedRequestPolicyRule } from '~/generated/station/station.did';

describe('PolicyRuleResultView', () => {
  it('renders properly', () => {});

  it.each<EvaluatedRequestPolicyRule>([
    {
      AllowListed: null,
    },
    {
      AutoApproved: null,
    },
    {
      AllowListedByMetadata: {
        metadata: {
          key: 'key',
          value: 'value',
        },
      },
    },
    {
      Quorum: {
        approvers: [],
        min_approved: 1n,
        total_possible_approvers: 2n,
      },
    },
    {
      QuorumPercentage: {
        approvers: [],
        min_approved: 1n,
        total_possible_approvers: 2n,
      },
    },
  ])('renders %o properly', rule => {
    const wrapper = mount(PolicyRuleResultView, {
      props: {
        evaluatedRule: rule,
        requestApprovals: [],
        status: { Approved: null },
      },
    });
    expect(wrapper.exists()).toBe(true);
  });
  it.each<{ rule: EvaluatedRequestPolicyRule; expectedChildren: number }>([
    {
      rule: {
        Not: { evaluated_rule: { AutoApproved: null }, status: { Approved: null } },
      } satisfies EvaluatedRequestPolicyRule,
      expectedChildren: 1,
    },
    {
      rule: {
        AllOf: [
          { evaluated_rule: { AutoApproved: null }, status: { Approved: null } },
          { evaluated_rule: { AllowListed: null }, status: { Approved: null } },
        ],
      } satisfies EvaluatedRequestPolicyRule,
      expectedChildren: 2,
    },
    {
      rule: {
        AnyOf: [
          { evaluated_rule: { AutoApproved: null }, status: { Approved: null } },
          { evaluated_rule: { AllowListed: null }, status: { Approved: null } },
        ],
      } satisfies EvaluatedRequestPolicyRule,
      expectedChildren: 2,
    },
    {
      rule: {
        AnyOf: [
          { evaluated_rule: { AutoApproved: null }, status: { Approved: null } },
          {
            evaluated_rule: {
              AllOf: [
                { evaluated_rule: { AutoApproved: null }, status: { Approved: null } },
                { evaluated_rule: { AllowListed: null }, status: { Approved: null } },
              ],
            },
            status: { Approved: null },
          },
        ],
      } satisfies EvaluatedRequestPolicyRule,
      expectedChildren: 4,
    },
  ])('renders nested rules properly', ({ rule, expectedChildren }) => {
    const wrapper = mount(PolicyRuleResultView, {
      props: {
        evaluatedRule: rule,
        requestApprovals: [],
        status: { Approved: null },
      },
    });
    expect(wrapper.exists()).toBe(true);
    const children = wrapper.findAllComponents(PolicyRuleResultView);
    expect(children).toHaveLength(expectedChildren);
  });
});
