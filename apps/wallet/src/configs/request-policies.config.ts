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
  [RequestSpecifierEnum.ChangeCanister]: [...defaultRequestPolicyRules],
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
  [RequestSpecifierEnum.ChangeManagedCanister]: [...defaultRequestPolicyRules],
  [RequestSpecifierEnum.CreateManagedCanister]: [...defaultRequestPolicyRules],
});
