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

export const useRequestSpecifierRules = (
  specifier: Ref<RequestSpecifier | null | undefined>,
): ComputedRef<SelectItem<RequestPolicyRuleEnum | { NamedRule: UUID }>[]> => {
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
    const items: SelectItem<RequestSpecifierRule>[] = [];

    if (!specifier.value) {
      allRequestPolicyRules.forEach(rule => {
        items.push({
          value: rule,
          text: i18n.t(`request_policies.rule.${rule.toLowerCase()}`),
        });
      });
    } else {
      const specifierEnum = mapRequestSpecifierToEnum(specifier.value);

      if (allSpecifierRules[specifierEnum]) {
        allSpecifierRules[specifierEnum].forEach(rule => {
          items.push({
            value: rule,
            text: i18n.t(`request_policies.rule.${rule.toLowerCase()}`),
          });
        });
      }
    }

    if (namedRules.value) {
      const namedRuleValues = namedRules.value.map(rule => ({
        value: { NamedRule: rule.id },
        text: i18n.t(`request_policies.rule.named_rule`, { name: rule.name }),
      }));

      items.unshift(...namedRuleValues);
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

type PopulatedUserSpecifier =
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
  if (userSpecifier.kind === RequestPolicyRuleUserSpecifierEnum.Id) {
    return i18n.t('request_policies.rule_user_summary.id', {
      users: userSpecifier.users.map(user => user.name).join(', '),
    });
  } else if (userSpecifier.kind === RequestPolicyRuleUserSpecifierEnum.Group) {
    return i18n.t('request_policies.rule_user_summary.group', {
      groups: userSpecifier.groups.map(group => group.name).join(', '),
    });
  } else {
    return i18n.t('request_policies.rule_user_summary.any');
  }
}

function indentMultilineText(text: string): string {
  return text
    .split('\n')
    .map(line => `    ${line}`)
    .join('\n');
}

function populatedRuleToTooltip(rule: PopulatedRule, i18n: ReturnType<typeof useI18n>): string {
  if (rule.kind === 'NamedRule') {
    return i18n.t('request_policies.rule_tooltip_summary.named_rule', {
      name: rule.name ?? rule.id,
    });
  }
  if (rule.kind === 'Quorum') {
    return i18n.t('request_policies.rule_tooltip_summary.quorum', {
      quorum: rule.n,
      specifier: populatedUserSpecifierToTooltip(rule.approvers, i18n),
    });
  }
  if (rule.kind === 'QuorumPercentage') {
    return i18n.t('request_policies.rule_tooltip_summary.quorumpercentage', {
      percentage: rule.n,
      specifier: populatedUserSpecifierToTooltip(rule.approvers, i18n),
    });
  }
  if (rule.kind === 'AllowListedByMetadata') {
    return i18n.t('request_policies.rule_tooltip_summary.allowlistedbymetadata', {
      metadata: rule.value ? `"${rule.key}=${rule.value}"` : `"${rule.key}"`,
    });
  }
  if (rule.kind === 'AllowListed') {
    return i18n.t('request_policies.rule_tooltip_summary.allowlisted');
  }
  if (rule.kind === 'AutoApproved') {
    return i18n.t('request_policies.rule_tooltip_summary.autoapproved');
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

function populatedRuleToShortSummary(
  rule: PopulatedRule,
  i18n: ReturnType<typeof useI18n>,
): string {
  if (rule.kind === 'AllOf') {
    return rule.rules
      .map(r => populatedRuleToShortSummary(r, i18n))
      .join(i18n.t('request_policies.rule_short_summary.allof'));
  } else if (rule.kind === 'AnyOf') {
    return rule.rules
      .map(r => populatedRuleToShortSummary(r, i18n))
      .join(i18n.t('request_policies.rule_short_summary.anyof'));
  } else if (rule.kind === 'Not') {
    return (
      i18n.t('request_policies.rule_short_summary.not') +
      ' ' +
      populatedRuleToShortSummary(rule.rule, i18n)
    );
  } else if (rule.kind === 'NamedRule') {
    return i18n.t('request_policies.rule_short_summary.named_rule', { name: rule.name ?? rule.id });
  } else if (rule.kind === 'AutoApproved') {
    return i18n.t('request_policies.rule_short_summary.autoapproved');
  } else if (rule.kind === 'Quorum') {
    return i18n.t('request_policies.rule_short_summary.quorum', {
      quorum: rule.n,
      specifier: populatedUserSpecifierToTooltip(rule.approvers, i18n),
    });
  } else if (rule.kind === 'QuorumPercentage') {
    return i18n.t('request_policies.rule_short_summary.quorumpercentage', {
      percentage: rule.n,
      specifier: populatedUserSpecifierToTooltip(rule.approvers, i18n),
    });
  } else if (rule.kind === 'AllowListedByMetadata') {
    return i18n.t('request_policies.rule_short_summary.allowlistedbymetadata', {
      metadata: rule.value ? `"${rule.key}=${rule.value}"` : `"${rule.key}"`,
    });
  } else if (rule.kind === 'AllowListed') {
    return i18n.t('request_policies.rule_short_summary.allowlisted');
  } else {
    // return populatedRuleToTooltip(rule, i18n);
    return unreachable(rule);
  }
}

export function useRuleToShortSummary(rule: Ref<RequestPolicyRule | null>): {
  isComplex: Ref<boolean>;
  summary: Ref<string | null>;
  tooltip: Ref<string | null>;
} {
  const station = services().station;
  const i18n = useI18n();
  const summary = ref<string | null>(null);
  const isComplex = ref<boolean>(false);
  const tooltip = ref<string | null>(null);

  watch(
    rule,
    async rule => {
      if (!rule) {
        summary.value = null;
        return;
      }
      const populatedRule = await populateRule(rule, station);

      summary.value = populatedRuleToShortSummary(populatedRule, i18n);
      isComplex.value = isRuleComplex(populatedRule);

      tooltip.value = populatedRuleToTooltip(populatedRule, i18n);
    },
    { immediate: true },
  );

  return {
    isComplex,
    summary,
    tooltip,
  };
}
