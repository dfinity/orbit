import { onMounted, watch } from 'vue';
import { ComputedRef, Ref, computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  allRequestPolicyRules,
  requestSpecifiersIncludedRules,
} from '~/configs/request-policies.config';
import {
  NamedRule,
  RequestPolicyRule,
  RequestSpecifier,
  UserSpecifier,
  UUID,
} from '~/generated/station/station.did';
import { mapRequestSpecifierToEnum } from '~/mappers/request-specifiers.mapper';
import { services } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { useStationStore } from '~/stores/station.store';
import { SelectItem } from '~/types/helper.types';
import { RequestPolicyRuleEnum, RequestPolicyRuleUserSpecifierEnum } from '~/types/station.types';
import { unreachable, variantIs } from '~/utils/helper.utils';

export type RequestSpecifierRule = RequestPolicyRuleEnum | { NamedRule: UUID };

export type RuleSelectItem = SelectItem<RequestSpecifierRule> | { header: string };

export const useRequestSpecifierRules = (
  specifier: Ref<RequestSpecifier | null | undefined>,
): ComputedRef<RuleSelectItem[]> => {
  const allSpecifierRules = requestSpecifiersIncludedRules();
  const i18n = useI18n();
  const station = useStationStore();
  const namedRules = ref<NamedRule[] | null>(null); // Adjust the type as needed

  // Fetch the named rules asynchronously
  onMounted(async () => {
    const { named_rules } = await station.service.listNamedRules();
    namedRules.value = named_rules;
  });

  return computed(() => {
    const items: RuleSelectItem[] = [];

    if (!specifier.value) {
      items.push({
        header: i18n.t('request_policies.rule_groups.custom_rules'),
      });
      allRequestPolicyRules.forEach(rule => {
        items.push({
          value: rule,
          text: i18n.t(`request_policies.rule.${rule.toLowerCase()}`),
        });
      });
    } else {
      const specifierEnum = mapRequestSpecifierToEnum(specifier.value);

      if (allSpecifierRules[specifierEnum]) {
        items.push({
          header: i18n.t('request_policies.rule_groups.custom_rules'),
        });

        allSpecifierRules[specifierEnum].forEach(rule => {
          items.push({
            value: rule,
            text: i18n.t(`request_policies.rule.${rule.toLowerCase()}`),
          });
        });
      }
    }

    if (namedRules.value && namedRules.value.length > 0) {
      const namedRuleValues = namedRules.value.map(rule => ({
        value: { NamedRule: rule.id },
        text: rule.name,
      }));

      items.unshift(
        {
          header: i18n.t('request_policies.rule_groups.named_rules'),
        },
        ...namedRuleValues,
      );
    }

    return items;
  });
};

export const useUserSpecifierSelectorItems = (): ComputedRef<SelectItem[]> => {
  const i18n = useI18n();

  return computed(() => {
    const items: SelectItem[] = [];

    for (const userType of Object.values(RequestPolicyRuleUserSpecifierEnum)) {
      items.push({
        value: userType,
        text: i18n.t(`request_policies.rule_user_specifier.${userType.toLowerCase()}`),
      });
    }

    return items;
  });
};

export type PopulatedUserSpecifier =
  | {
      kind: RequestPolicyRuleUserSpecifierEnum.Any;
    }
  | {
      kind: RequestPolicyRuleUserSpecifierEnum.Id;
      users: {
        id: UUID;
        name?: string;
      }[];
    }
  | {
      kind: RequestPolicyRuleUserSpecifierEnum.Group;
      groups: {
        id: UUID;
        name?: string;
      }[];
    };

export type PopulatedRule =
  | {
      kind: RequestPolicyRuleEnum.AutoApproved;
    }
  | {
      kind: RequestPolicyRuleEnum.Quorum;
      n: number;
      approvers: PopulatedUserSpecifier;
    }
  | {
      kind: RequestPolicyRuleEnum.QuorumPercentage;
      n: number;
      approvers: PopulatedUserSpecifier;
    }
  | {
      kind: RequestPolicyRuleEnum.AllowListedByMetadata;
      key: string;
      value: string;
    }
  | {
      kind: RequestPolicyRuleEnum.AllowListed;
    }
  | {
      kind: RequestPolicyRuleEnum.AllOf;
      rules: PopulatedRule[];
    }
  | {
      kind: RequestPolicyRuleEnum.AnyOf;
      rules: PopulatedRule[];
    }
  | {
      kind: RequestPolicyRuleEnum.Not;
      rule: PopulatedRule;
    }
  | {
      kind: 'NamedRule';
      name?: string;
      id: UUID;
      rule?: PopulatedRule;
    };

async function populateUserSpecifier(
  userSpecifier: UserSpecifier,
  station: StationService,
): Promise<PopulatedUserSpecifier> {
  if (variantIs(userSpecifier, 'Id')) {
    const users = await Promise.all(
      userSpecifier.Id.map(id =>
        station
          .getUser({ user_id: id })
          .catch(_ => ({
            user: {
              id,
              name: undefined,
            },
          }))
          .then(result => ({
            id,
            name: result.user.name || id,
          })),
      ),
    );

    return {
      kind: RequestPolicyRuleUserSpecifierEnum.Id,
      users,
    };
  } else if (variantIs(userSpecifier, 'Group')) {
    const groups = await Promise.all(
      userSpecifier.Group.map(group_id =>
        station
          .getUserGroup({ user_group_id: group_id })
          .catch(_ => ({
            user_group: {
              name: group_id,
            },
          }))
          .then(result => ({
            id: group_id,
            name: result.user_group.name || group_id,
          })),
      ),
    );
    return {
      kind: RequestPolicyRuleUserSpecifierEnum.Group,
      groups,
    };
  } else {
    return {
      kind: RequestPolicyRuleUserSpecifierEnum.Any,
    };
  }
}

async function populateRule(
  rule: RequestPolicyRule,
  station: StationService,
): Promise<PopulatedRule> {
  if (variantIs(rule, 'AutoApproved')) {
    return {
      kind: RequestPolicyRuleEnum.AutoApproved,
    };
  } else if (variantIs(rule, 'Quorum')) {
    return {
      kind: RequestPolicyRuleEnum.Quorum,
      n: rule.Quorum.min_approved,
      approvers: await populateUserSpecifier(rule.Quorum.approvers, station),
    };
  } else if (variantIs(rule, 'QuorumPercentage')) {
    return {
      kind: RequestPolicyRuleEnum.QuorumPercentage,
      n: rule.QuorumPercentage.min_approved,
      approvers: await populateUserSpecifier(rule.QuorumPercentage.approvers, station),
    };
  } else if (variantIs(rule, 'AllowListedByMetadata')) {
    return {
      kind: RequestPolicyRuleEnum.AllowListedByMetadata,
      key: rule.AllowListedByMetadata.key,
      value: rule.AllowListedByMetadata.value,
    };
  } else if (variantIs(rule, 'AllowListed')) {
    return {
      kind: RequestPolicyRuleEnum.AllowListed,
    };
  } else if (variantIs(rule, 'AllOf')) {
    return {
      kind: RequestPolicyRuleEnum.AllOf,
      rules: await Promise.all(rule.AllOf.map(r => populateRule(r, station))),
    };
  } else if (variantIs(rule, 'AnyOf')) {
    return {
      kind: RequestPolicyRuleEnum.AnyOf,
      rules: await Promise.all(rule.AnyOf.map(r => populateRule(r, station))),
    };
  } else if (variantIs(rule, 'Not')) {
    return {
      kind: RequestPolicyRuleEnum.Not,
      rule: await populateRule(rule.Not, station),
    };
  } else if (variantIs(rule, 'NamedRule')) {
    const { named_rule } = await station.getNamedRule(rule.NamedRule).catch(_ => ({
      named_rule: {
        name: undefined,
        id: rule.NamedRule,
        rule: undefined,
      },
    }));

    return {
      kind: 'NamedRule',
      name: named_rule.name,
      id: named_rule.id,
      rule: named_rule.rule ? await populateRule(named_rule.rule, station) : undefined,
    };
  } else {
    return unreachable(rule);
  }
}

function populatedUserSpecifierToTooltip(
  userSpecifier: PopulatedUserSpecifier,
  i18n: ReturnType<typeof useI18n>,
): string {
  if (userSpecifier.kind === RequestPolicyRuleUserSpecifierEnum.Any) {
    return i18n.t('request_policies.rule_rich_summary.any_user_specifier');
  } else if (userSpecifier.kind === RequestPolicyRuleUserSpecifierEnum.Id) {
    if (userSpecifier.users.length === 0) {
      return i18n.t('request_policies.rule_rich_summary.no_user_specifier');
    }
    return userSpecifier.users.map(user => user.name).join(', ');
  } else if (userSpecifier.kind === RequestPolicyRuleUserSpecifierEnum.Group) {
    return userSpecifier.groups.map(group => group.name).join(', ');
  } else {
    return unreachable(userSpecifier);
  }
}

function indentMultilineText(text: string): string {
  return text
    .split('\n')
    .map(line => `    ${line}`)
    .join('\n');
}

export function populatedRuleToTooltip(
  rule: PopulatedRule,
  i18n: ReturnType<typeof useI18n>,
): string {
  if (rule.kind === 'NamedRule') {
    return i18n.t('request_policies.rule_rich_summary.named_rule', {
      name: rule.name ?? rule.id,
    });
  } else if (rule.kind === 'Quorum') {
    // Any user can approve
    if (rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Any) {
      return i18n.t('request_policies.rule_rich_summary.any_user_specifier', rule.n);
    }

    // No user can approve
    else if (
      rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Id &&
      (rule.n === 0 || rule.approvers.users.length === 0)
    ) {
      return i18n.t('request_policies.rule_rich_summary.invalid_rule_auto_approved');
    }

    // Single user can approve
    else if (
      rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Id &&
      rule.approvers.users.length === 1
    ) {
      return i18n.t('request_policies.rule_rich_summary.single_user_specifier', {
        user: populatedUserSpecifierToTooltip(rule.approvers, i18n),
      });
    }

    // Multiple users can approve
    else if (rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Id) {
      return i18n.t(
        'request_policies.rule_rich_summary.user_specifier',
        {
          users: populatedUserSpecifierToTooltip(rule.approvers, i18n),
          n: rule.n,
        },
        rule.n,
      );
    }

    // Groups can approve
    else {
      return i18n.t(
        'request_policies.rule_rich_summary.group_specifier',
        {
          groups: populatedUserSpecifierToTooltip(rule.approvers, i18n),
          n: rule.n,
        },
        rule.n,
      );
    }
  } else if (rule.kind === 'QuorumPercentage') {
    if (rule.approvers.kind === RequestPolicyRuleUserSpecifierEnum.Any) {
      return i18n.t('request_policies.rule_rich_summary.quorum_percentage_any_user', {
        n: rule.n,
      });
    }
    return i18n.t('request_policies.rule_rich_summary.quorum_percentage_rule', {
      n: rule.n,
      users: populatedUserSpecifierToTooltip(rule.approvers, i18n),
    });
  } else if (rule.kind === 'AllowListedByMetadata') {
    return i18n.t('request_policies.rule_rich_summary.allowlisted_by_metadata', {
      metadata: rule.value ? `"${rule.key}=${rule.value}"` : `"${rule.key}"`,
    });
  } else if (rule.kind === 'AllowListed') {
    return i18n.t('request_policies.rule_rich_summary.allowlisted');
  } else if (rule.kind === 'AutoApproved') {
    return i18n.t('request_policies.rule_rich_summary.auto_approved');
  } else if (rule.kind === 'AllOf') {
    return (
      i18n.t('request_policies.rule_tooltip_summary.allof') +
      '\n' +
      indentMultilineText(rule.rules.map(r => '- ' + populatedRuleToTooltip(r, i18n)).join('\n'))
    );
  } else if (rule.kind === 'AnyOf') {
    return (
      i18n.t('request_policies.rule_tooltip_summary.anyof') +
      '\n' +
      indentMultilineText(rule.rules.map(r => '- ' + populatedRuleToTooltip(r, i18n)).join('\n'))
    );
  } else if (rule.kind === 'Not') {
    return (
      i18n.t('request_policies.rule_tooltip_summary.not') +
      '\n' +
      indentMultilineText('- ' + populatedRuleToTooltip(rule.rule, i18n))
    );
  } else {
    return unreachable(rule);
  }
}

export function useRuleToTooltip(rule: Ref<RequestPolicyRule | null>): Ref<string | null> {
  const station = services().station;
  const i18n = useI18n();
  const tooltip = ref<string | null>(null);

  watch(
    rule,
    async rule => {
      if (!rule) {
        tooltip.value = null;
        return;
      }

      const populatedRule = await populateRule(rule, station);
      tooltip.value = populatedRuleToTooltip(populatedRule, i18n);
    },
    { immediate: true },
  );

  return tooltip;
}

function complexRuleCount(rule: PopulatedRule): number {
  if (rule.kind === 'AllOf' || rule.kind === 'AnyOf') {
    const childrenComplexCount = rule.rules.reduce((count, r) => count + complexRuleCount(r), 0);
    return 1 + childrenComplexCount;
  } else if (rule.kind === 'Not') {
    return 1 + complexRuleCount(rule.rule);
  }
  return 0;
}

export function isRuleComplex(rule: PopulatedRule): boolean {
  return complexRuleCount(rule) > 1;
}

export function usePopulatedRule(rule: Ref<RequestPolicyRule | null>): {
  populatedRule: Ref<PopulatedRule | null>;
  complexRuleSummary: Ref<string | null>;
} {
  const station = services().station;
  const populatedRule = ref<PopulatedRule | null>(null);
  const complexRuleSummary = ref<string | null>(null);
  const i18n = useI18n();
  watch(
    rule,
    async rule => {
      if (!rule) {
        populatedRule.value = null;
        return;
      }
      populatedRule.value = await populateRule(rule, station);
      const isComplex = isRuleComplex(populatedRule.value);
      complexRuleSummary.value = isComplex
        ? populatedRuleToTooltip(populatedRule.value, i18n)
        : null;
    },
    { immediate: true },
  );

  return {
    populatedRule,
    complexRuleSummary,
  };
}
