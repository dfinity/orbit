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

  if (variantIs(specifier, 'SystemUpgrade')) {
    return RequestSpecifierEnum.SystemUpgrade;
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

  if (variantIs(specifier, 'ChangeExternalCanister')) {
    return RequestSpecifierEnum.ChangeExternalCanister;
  }

  if (variantIs(specifier, 'CreateExternalCanister')) {
    return RequestSpecifierEnum.CreateExternalCanister;
  }

  if (variantIs(specifier, 'CallExternalCanister')) {
    return RequestSpecifierEnum.CallExternalCanister;
  }

  if (variantIs(specifier, 'FundExternalCanister')) {
    return RequestSpecifierEnum.FundExternalCanister;
  }

  if (variantIs(specifier, 'SetDisasterRecovery')) {
    return RequestSpecifierEnum.SetDisasterRecovery;
  }

  if (variantIs(specifier, 'AddAsset')) {
    return RequestSpecifierEnum.AddAsset;
  }

  if (variantIs(specifier, 'EditAsset')) {
    return RequestSpecifierEnum.EditAsset;
  }

  if (variantIs(specifier, 'RemoveAsset')) {
    return RequestSpecifierEnum.RemoveAsset;
  }

  if (variantIs(specifier, 'AddNamedRule')) {
    return RequestSpecifierEnum.AddNamedRule;
  }

  if (variantIs(specifier, 'EditNamedRule')) {
    return RequestSpecifierEnum.EditNamedRule;
  }

  if (variantIs(specifier, 'RemoveNamedRule')) {
    return RequestSpecifierEnum.RemoveNamedRule;
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
