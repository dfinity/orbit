import { describe, expect, it } from 'vitest';
import { PopulatedRule, populatedRuleToTooltip } from './request-policies.composable';
import { RequestPolicyRuleEnum, RequestPolicyRuleUserSpecifierEnum } from '~/types/station.types';
import { i18n } from '~/plugins/i18n.plugin';

describe('request-policies.composable', () => {
  it('display the any user quorum rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.Quorum,
      approvers: {
        kind: RequestPolicyRuleUserSpecifierEnum.Any,
      },
      n: 5,
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });
  it('display the some users quorum rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.Quorum,
      approvers: {
        kind: RequestPolicyRuleUserSpecifierEnum.Id,
        users: [
          {
            id: '1',
            name: 'Alice',
          },
          {
            id: '2',
            name: 'Bob',
          },
        ],
      },
      n: 5,
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });

  it('display the some groups quorum rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.Quorum,
      approvers: {
        kind: RequestPolicyRuleUserSpecifierEnum.Group,
        groups: [
          {
            id: '1',
            name: 'Admin',
          },
          {
            id: '2',
            name: 'Developers',
          },
        ],
      },
      n: 1,
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });

  it('display the any user percentage quorum rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.QuorumPercentage,
      approvers: {
        kind: RequestPolicyRuleUserSpecifierEnum.Any,
      },
      n: 50,
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });

  it('display the some users percentage quorum rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.QuorumPercentage,
      approvers: {
        kind: RequestPolicyRuleUserSpecifierEnum.Id,
        users: [{ id: '1', name: 'Alice' }],
      },
      n: 50,
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });

  it('display the allowlisted by metadata rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.AllowListedByMetadata,
      key: 'test',
      value: 'test',
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });

  it('display the allowlisted rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.AllowListed,
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });

  it('display the auto approved rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.AutoApproved,
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });

  it('display the all of rule correctly', () => {
    const rule: PopulatedRule = {
      kind: RequestPolicyRuleEnum.AllOf,
      rules: [
        {
          kind: RequestPolicyRuleEnum.AnyOf,
          rules: [
            {
              kind: RequestPolicyRuleEnum.AllowListed,
            },
            {
              kind: RequestPolicyRuleEnum.AutoApproved,
            },
          ],
        },
        {
          kind: RequestPolicyRuleEnum.Not,
          rule: {
            kind: RequestPolicyRuleEnum.AllowListed,
          },
        },
        {
          kind: RequestPolicyRuleEnum.Quorum,
          approvers: { kind: RequestPolicyRuleUserSpecifierEnum.Any },
          n: 1,
        },
      ],
    };
    const tooltip = populatedRuleToTooltip(rule, i18n.global);
    expect(tooltip).toMatchSnapshot();
  });
});
