import { ProposalSpecifier, UserSpecifier } from '~/generated/station/station.did';
import { ProposalCriteriaUserSpecifierEnum, ProposalSpecifierEnum } from '~/types/wallet.types';
import { unreachable, variantIs } from '~/utils/helper.utils';

export const mapProposalSpecifierToEnum = (specifier: ProposalSpecifier): ProposalSpecifierEnum => {
  if (variantIs(specifier, 'EditAccessPolicy')) {
    return ProposalSpecifierEnum.EditAccessPolicy;
  }

  if (variantIs(specifier, 'AddProposalPolicy')) {
    return ProposalSpecifierEnum.AddProposalPolicy;
  }

  if (variantIs(specifier, 'EditProposalPolicy')) {
    return ProposalSpecifierEnum.EditProposalPolicy;
  }

  if (variantIs(specifier, 'RemoveProposalPolicy')) {
    return ProposalSpecifierEnum.RemoveProposalPolicy;
  }

  if (variantIs(specifier, 'AddAccount')) {
    return ProposalSpecifierEnum.AddAccount;
  }

  if (variantIs(specifier, 'EditAccount')) {
    return ProposalSpecifierEnum.EditAccount;
  }

  if (variantIs(specifier, 'AddUser')) {
    return ProposalSpecifierEnum.AddUser;
  }

  if (variantIs(specifier, 'EditUser')) {
    return ProposalSpecifierEnum.EditUser;
  }

  if (variantIs(specifier, 'AddUserGroup')) {
    return ProposalSpecifierEnum.AddUserGroup;
  }

  if (variantIs(specifier, 'EditUserGroup')) {
    return ProposalSpecifierEnum.EditUserGroup;
  }

  if (variantIs(specifier, 'RemoveUserGroup')) {
    return ProposalSpecifierEnum.RemoveUserGroup;
  }

  if (variantIs(specifier, 'ChangeCanister')) {
    return ProposalSpecifierEnum.ChangeCanister;
  }

  if (variantIs(specifier, 'Transfer')) {
    return ProposalSpecifierEnum.Transfer;
  }

  if (variantIs(specifier, 'AddAddressBookEntry')) {
    return ProposalSpecifierEnum.AddAddressBookEntry;
  }

  if (variantIs(specifier, 'EditAddressBookEntry')) {
    return ProposalSpecifierEnum.EditAddressBookEntry;
  }

  if (variantIs(specifier, 'RemoveAddressBookEntry')) {
    return ProposalSpecifierEnum.RemoveAddressBookEntry;
  }

  return unreachable(specifier);
};

export const mapProposalCriteriaUserSpecifierToEnum = (
  specifier: UserSpecifier,
): ProposalCriteriaUserSpecifierEnum => {
  if (variantIs(specifier, 'Owner')) {
    return ProposalCriteriaUserSpecifierEnum.Owner;
  }

  if (variantIs(specifier, 'Proposer')) {
    return ProposalCriteriaUserSpecifierEnum.Proposer;
  }

  if (variantIs(specifier, 'Any')) {
    return ProposalCriteriaUserSpecifierEnum.Any;
  }

  if (variantIs(specifier, 'Group')) {
    return ProposalCriteriaUserSpecifierEnum.Group;
  }

  if (variantIs(specifier, 'Id')) {
    return ProposalCriteriaUserSpecifierEnum.Id;
  }

  return unreachable(specifier);
};

export const mapProposalCriteriaUserSpecifierEnumToVariant = (
  specifier: ProposalCriteriaUserSpecifierEnum,
): UserSpecifier => {
  switch (specifier) {
    case ProposalCriteriaUserSpecifierEnum.Owner:
      return { Owner: null };
    case ProposalCriteriaUserSpecifierEnum.Proposer:
      return { Proposer: null };
    case ProposalCriteriaUserSpecifierEnum.Any:
      return { Any: null };
    case ProposalCriteriaUserSpecifierEnum.Group:
      return { Group: [] };
    case ProposalCriteriaUserSpecifierEnum.Id:
      return { Id: [] };
  }
};
