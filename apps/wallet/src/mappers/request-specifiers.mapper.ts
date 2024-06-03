import { RequestSpecifier, UserSpecifier } from '~/generated/station/station.did';
import { RequestPolicyRuleUserSpecifierEnum, RequestSpecifierEnum } from '~/types/station.types';
import { unreachable, variantIs } from '~/utils/helper.utils';

export const mapRequestSpecifierToEnum = (specifier: RequestSpecifier): RequestSpecifierEnum => {
  if (variantIs(specifier, 'EditPermission')) {
    return RequestSpecifierEnum.EditPermission;
  }

  if (variantIs(specifier, 'AddRequestPolicy')) {
    return RequestSpecifierEnum.AddRequestPolicy;
  }

  if (variantIs(specifier, 'EditRequestPolicy')) {
    return RequestSpecifierEnum.EditRequestPolicy;
  }

  if (variantIs(specifier, 'RemoveRequestPolicy')) {
    return RequestSpecifierEnum.RemoveRequestPolicy;
  }

  if (variantIs(specifier, 'AddAccount')) {
    return RequestSpecifierEnum.AddAccount;
  }

  if (variantIs(specifier, 'EditAccount')) {
    return RequestSpecifierEnum.EditAccount;
  }

  if (variantIs(specifier, 'AddUser')) {
    return RequestSpecifierEnum.AddUser;
  }

  if (variantIs(specifier, 'EditUser')) {
    return RequestSpecifierEnum.EditUser;
  }

  if (variantIs(specifier, 'AddUserGroup')) {
    return RequestSpecifierEnum.AddUserGroup;
  }

  if (variantIs(specifier, 'EditUserGroup')) {
    return RequestSpecifierEnum.EditUserGroup;
  }

  if (variantIs(specifier, 'RemoveUserGroup')) {
    return RequestSpecifierEnum.RemoveUserGroup;
  }

  if (variantIs(specifier, 'ChangeCanister')) {
    return RequestSpecifierEnum.ChangeCanister;
  }

  if (variantIs(specifier, 'Transfer')) {
    return RequestSpecifierEnum.Transfer;
  }

  if (variantIs(specifier, 'AddAddressBookEntry')) {
    return RequestSpecifierEnum.AddAddressBookEntry;
  }

  if (variantIs(specifier, 'EditAddressBookEntry')) {
    return RequestSpecifierEnum.EditAddressBookEntry;
  }

  if (variantIs(specifier, 'RemoveAddressBookEntry')) {
    return RequestSpecifierEnum.RemoveAddressBookEntry;
  }

  if (variantIs(specifier, 'ManageSystemInfo')) {
    return RequestSpecifierEnum.ManageSystemInfo;
  }

  if (variantIs(specifier, 'ChangeManagedCanister')) {
    return RequestSpecifierEnum.ChangeManagedCanister;
  }

  return unreachable(specifier);
};

export const mapRequestPolicyRuleUserSpecifierToEnum = (
  specifier: UserSpecifier,
): RequestPolicyRuleUserSpecifierEnum => {
  if (variantIs(specifier, 'Any')) {
    return RequestPolicyRuleUserSpecifierEnum.Any;
  }

  if (variantIs(specifier, 'Group')) {
    return RequestPolicyRuleUserSpecifierEnum.Group;
  }

  if (variantIs(specifier, 'Id')) {
    return RequestPolicyRuleUserSpecifierEnum.Id;
  }

  return unreachable(specifier);
};

export const mapRequestPolicyRuleUserSpecifierEnumToVariant = (
  specifier: RequestPolicyRuleUserSpecifierEnum,
): UserSpecifier => {
  switch (specifier) {
    case RequestPolicyRuleUserSpecifierEnum.Any:
      return { Any: null };
    case RequestPolicyRuleUserSpecifierEnum.Group:
      return { Group: [] };
    case RequestPolicyRuleUserSpecifierEnum.Id:
      return { Id: [] };
  }
};
