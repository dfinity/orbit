import { RequestPolicyRuleEnum, RequestSpecifierEnum } from '~/types/station.types';

export const defaultRequestPolicyRules = [
  RequestPolicyRuleEnum.AutoApproved,
  RequestPolicyRuleEnum.QuorumPercentage,
  RequestPolicyRuleEnum.Quorum,
  RequestPolicyRuleEnum.AllOf,
  RequestPolicyRuleEnum.AnyOf,
  RequestPolicyRuleEnum.Not,
];

export const requestSpecifiersIncludedRules = (): Record<
  RequestSpecifierEnum,
  RequestPolicyRuleEnum[]
> => ({
  [RequestSpecifierEnum.Transfer]: [
    RequestPolicyRuleEnum.AllowListedByMetadata,
    RequestPolicyRuleEnum.AllowListed,
    ...defaultRequestPolicyRules,
  ],
  [RequestSpecifierEnum.EditPermission]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.AddRequestPolicy]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.EditRequestPolicy]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.RemoveRequestPolicy]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.SystemUpgrade]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.AddUserGroup]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.EditUserGroup]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.RemoveUserGroup]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.AddUser]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.EditUser]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.AddAccount]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.EditAccount]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.AddAddressBookEntry]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.EditAddressBookEntry]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.RemoveAddressBookEntry]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.ManageSystemInfo]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.ChangeExternalCanister]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.CreateExternalCanister]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.CallExternalCanister]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.FundExternalCanister]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.SetDisasterRecovery]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.AddAsset]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.EditAsset]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.RemoveAsset]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.AddNamedRule]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.EditNamedRule]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.RemoveNamedRule]: [...defaultRequestPolicyRules],
});
