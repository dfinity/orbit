import { ComputedRef, Ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { proposalSpecifiersIncludedCriterias } from '~/configs/proposal-policies.config';
import { ProposalSpecifier } from '~/generated/station/station.did';
import { mapProposalSpecifierToEnum } from '~/mappers/specifiers.mapper';
import { SelectItem } from '~/types/helper.types';
import { ProposalCriteriaUserSpecifierEnum } from '~/types/wallet.types';

export const useProposalSpecifierCriterias = (
  specifier: Ref<ProposalSpecifier>,
): ComputedRef<SelectItem[]> => {
  const allSpecifierCriterias = proposalSpecifiersIncludedCriterias();
  const i18n = useI18n();

  return computed(() => {
    const items: SelectItem[] = [];
    const specifierEnum = mapProposalSpecifierToEnum(specifier.value);

    if (allSpecifierCriterias[specifierEnum]) {
      allSpecifierCriterias[specifierEnum].forEach(criteria => {
        items.push({
          value: criteria,
          text: i18n.t(`proposal_policies.criteria.${criteria.toLowerCase()}`),
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

    for (const userType of Object.values(ProposalCriteriaUserSpecifierEnum)) {
      items.push({
        value: userType,
        text: i18n.t(`proposal_policies.criteria_user_specifier.${userType.toLowerCase()}`),
      });
    }

    return items;
  });
};
