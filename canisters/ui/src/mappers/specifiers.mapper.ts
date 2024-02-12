import { ProposalSpecifier } from '~/generated/wallet/wallet.did';
import { ProposalSpecifierEnum } from '~/types/wallet.types';
import { unreachable, variantIs } from '~/utils/helper.utils';

export const mapProposalSpecifierToEnum = (specifier: ProposalSpecifier): ProposalSpecifierEnum => {
  if (variantIs(specifier, 'AddAccessPolicy')) {
    return ProposalSpecifierEnum.AddAccessPolicy;
  }

  if (variantIs(specifier, 'EditAccessPolicy')) {
    return ProposalSpecifierEnum.EditAccessPolicy;
  }

  if (variantIs(specifier, 'RemoveAccessPolicy')) {
    return ProposalSpecifierEnum.RemoveAccessPolicy;
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
