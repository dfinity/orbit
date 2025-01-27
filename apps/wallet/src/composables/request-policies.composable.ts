import { ComputedRef, Ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  allRequestPolicyRules,
  requestSpecifiersIncludedRules,
} from '~/configs/request-policies.config';
import { RequestSpecifier } from '~/generated/station/station.did';
import { mapRequestSpecifierToEnum } from '~/mappers/request-specifiers.mapper';
import { SelectItem } from '~/types/helper.types';
import { RequestPolicyRuleUserSpecifierEnum } from '~/types/station.types';

export const useRequestSpecifierRules = (
  specifier: Ref<RequestSpecifier | null | undefined>,
): ComputedRef<SelectItem[]> => {
  const allSpecifierRules = requestSpecifiersIncludedRules();
  const i18n = useI18n();

  return computed(() => {
    if (!specifier.value) {
      return allRequestPolicyRules.map(rule => ({
        value: rule,
        text: i18n.t(`request_policies.rule.${rule.toLowerCase()}`),
      }));
    }

    const items: SelectItem[] = [];
    const specifierEnum = mapRequestSpecifierToEnum(specifier.value);

    if (allSpecifierRules[specifierEnum]) {
      allSpecifierRules[specifierEnum].forEach(rule => {
        items.push({
          value: rule,
          text: i18n.t(`request_policies.rule.${rule.toLowerCase()}`),
        });
      });
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
