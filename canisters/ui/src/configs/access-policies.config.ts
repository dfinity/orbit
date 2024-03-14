import { AccessPolicy, Resource, UUID } from '~/generated/wallet/wallet.did';
import {
  AggregatedResouceAccessPolicies,
  ResourceAccessAllowLevels,
  ResourceActionEnum,
  ResourceTypeEnum,
} from '~/types/access-policies.types';
import {
  isAccessPolicyResourceActionContained,
  isAccountResourceActionContained,
  isChangeCanisterResourceActionContained,
  isProposalResourceActionContained,
  isResourceActionContained,
  isSettingsResourceActionContained,
  isUserResourceActionContained,
} from '~/utils/access-policies.utils';
import { variantIs } from '~/utils/helper.utils';

export const defaultAllowLevels = (): ResourceAccessAllowLevels => ({
  allUsers: { policy: { resource: null, canEdit: false } },
  membersOfGroup: { policy: { resource: null, canEdit: false }, groups: [] },
  specificUsers: { policy: { resource: null, canEdit: false }, users: [] },
});

export const globalAccessPolicies = (): AggregatedResouceAccessPolicies[] => [
  {
    resourceType: ResourceTypeEnum.User,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { User: { List: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Create,
        resource: { User: { Create: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Read,
        resource: { User: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Update,
        resource: { User: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'User') && variantIs(policy.resource, 'User')) {
        return isUserResourceActionContained(specifier.User, policy.resource.User);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.UserGroup,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { UserGroup: { List: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Create,
        resource: { UserGroup: { Create: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Read,
        resource: { UserGroup: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Update,
        resource: { UserGroup: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Delete,
        resource: { UserGroup: { Delete: { Any: null } } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'UserGroup') && variantIs(policy.resource, 'UserGroup')) {
        return isResourceActionContained(specifier.UserGroup, policy.resource.UserGroup);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.Account,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { Account: { List: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Create,
        resource: { Account: { Create: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Read,
        resource: { Account: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Update,
        resource: { Account: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Transfer,
        resource: { Account: { Transfer: { Any: null } } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'Account') && variantIs(policy.resource, 'Account')) {
        return isAccountResourceActionContained(specifier.Account, policy.resource.Account);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.AddressBook,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { AddressBook: { List: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Create,
        resource: { AddressBook: { Create: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Read,
        resource: { AddressBook: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Update,
        resource: { AddressBook: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Delete,
        resource: { AddressBook: { Delete: { Any: null } } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'AddressBook') && variantIs(policy.resource, 'AddressBook')) {
        return isResourceActionContained(specifier.AddressBook, policy.resource.AddressBook);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.AccessPolicy,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { AccessPolicy: { List: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Read,
        resource: { AccessPolicy: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Update,
        resource: { AccessPolicy: { Edit: { Any: null } } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'AccessPolicy') && variantIs(policy.resource, 'AccessPolicy')) {
        return isAccessPolicyResourceActionContained(
          specifier.AccessPolicy,
          policy.resource.AccessPolicy,
        );
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.ProposalPolicy,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { ProposalPolicy: { List: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Create,
        resource: { ProposalPolicy: { Create: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Read,
        resource: { ProposalPolicy: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Update,
        resource: { ProposalPolicy: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Delete,
        resource: { ProposalPolicy: { Delete: { Any: null } } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'ProposalPolicy') && variantIs(policy.resource, 'ProposalPolicy')) {
        return isResourceActionContained(specifier.ProposalPolicy, policy.resource.ProposalPolicy);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.Settings,
    resources: [
      {
        action: ResourceActionEnum.ReadSensitiveConfig,
        resource: { Settings: { Read: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.ReadPublicConfig,
        resource: { Settings: { ReadConfig: null } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'Settings') && variantIs(policy.resource, 'Settings')) {
        return isSettingsResourceActionContained(specifier.Settings, policy.resource.Settings);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.ChangeCanister,
    resources: [
      {
        action: ResourceActionEnum.Create,
        resource: { ChangeCanister: { Create: null } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'ChangeCanister') && variantIs(policy.resource, 'ChangeCanister')) {
        return isChangeCanisterResourceActionContained(
          specifier.ChangeCanister,
          policy.resource.ChangeCanister,
        );
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.Proposal,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { Proposal: { List: null } },
        allow: defaultAllowLevels(),
      },
      {
        action: ResourceActionEnum.Read,
        resource: { Proposal: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
      },
    ],
    match(specifier: Resource, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'Proposal') && variantIs(policy.resource, 'Proposal')) {
        return isProposalResourceActionContained(specifier.Proposal, policy.resource.Proposal);
      }

      return false;
    },
  },
];

export const getAccountAccessPolicies = (accountId: UUID): AggregatedResouceAccessPolicies[] => {
  return [
    {
      resourceType: ResourceTypeEnum.Account,
      resources: [
        {
          action: ResourceActionEnum.Read,
          resource: { Account: { Read: { Id: accountId } } },
          allow: defaultAllowLevels(),
        },
        {
          action: ResourceActionEnum.Update,
          resource: { Account: { Update: { Id: accountId } } },
          allow: defaultAllowLevels(),
        },
        {
          action: ResourceActionEnum.Transfer,
          resource: { Account: { Update: { Id: accountId } } },
          allow: defaultAllowLevels(),
        },
      ],
      match(specifier: Resource, policy: AccessPolicy): boolean {
        if (variantIs(specifier, 'Account') && variantIs(policy.resource, 'Account')) {
          return isAccountResourceActionContained(specifier.Account, policy.resource.Account);
        }

        return false;
      },
    },
  ];
};

export const getUserAccessPolicies = (userId: UUID): AggregatedResouceAccessPolicies[] => {
  return [
    {
      resourceType: ResourceTypeEnum.User,
      resources: [
        {
          action: ResourceActionEnum.Read,
          resource: { User: { Read: { Id: userId } } },
          allow: defaultAllowLevels(),
        },
        {
          action: ResourceActionEnum.Update,
          resource: { User: { Update: { Id: userId } } },
          allow: defaultAllowLevels(),
        },
      ],
      match(specifier: Resource, policy: AccessPolicy): boolean {
        if (variantIs(specifier, 'User') && variantIs(policy.resource, 'User')) {
          return isUserResourceActionContained(specifier.User, policy.resource.User);
        }

        return false;
      },
    },
  ];
};

export const getUserGroupAccessPolicies = (groupId: UUID): AggregatedResouceAccessPolicies[] => {
  return [
    {
      resourceType: ResourceTypeEnum.UserGroup,
      resources: [
        {
          action: ResourceActionEnum.Read,
          resource: { UserGroup: { Read: { Id: groupId } } },
          allow: defaultAllowLevels(),
        },
        {
          action: ResourceActionEnum.Update,
          resource: { UserGroup: { Update: { Id: groupId } } },
          allow: defaultAllowLevels(),
        },
        {
          action: ResourceActionEnum.Delete,
          resource: { UserGroup: { Delete: { Id: groupId } } },
          allow: defaultAllowLevels(),
        },
      ],
      match(specifier: Resource, policy: AccessPolicy): boolean {
        if (variantIs(specifier, 'UserGroup') && variantIs(policy.resource, 'UserGroup')) {
          return isResourceActionContained(specifier.UserGroup, policy.resource.UserGroup);
        }

        return false;
      },
    },
  ];
};
