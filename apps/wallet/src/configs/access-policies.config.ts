import { Resource, UUID } from '~/generated/station/station.did';
import {
  AggregatedResouceAccessPolicies,
  AuthScopeEnum,
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
  isSystemResourceActionContained,
  isUserResourceActionContained,
} from '~/utils/access-policies.utils';
import { variantIs } from '~/utils/helper.utils';

export const defaultAllowLevels = (): ResourceAccessAllowLevels => ({
  authScope: AuthScopeEnum.Restrictred,
  membersOfGroup: [],
  specificUsers: [],
});

export const globalAccessPolicies = (): AggregatedResouceAccessPolicies[] => [
  {
    resourceType: ResourceTypeEnum.User,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { User: { List: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Create,
        resource: { User: { Create: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { User: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Update,
        resource: { User: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'User') && variantIs(resource, 'User')) {
        return isUserResourceActionContained(specifier.User, resource.User);
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
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Create,
        resource: { UserGroup: { Create: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { UserGroup: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Update,
        resource: { UserGroup: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Delete,
        resource: { UserGroup: { Delete: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'UserGroup') && variantIs(resource, 'UserGroup')) {
        return isResourceActionContained(specifier.UserGroup, resource.UserGroup);
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
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Create,
        resource: { Account: { Create: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { Account: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Update,
        resource: { Account: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Transfer,
        resource: { Account: { Transfer: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'Account') && variantIs(resource, 'Account')) {
        return isAccountResourceActionContained(specifier.Account, resource.Account);
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
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Create,
        resource: { AddressBook: { Create: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { AddressBook: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Update,
        resource: { AddressBook: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Delete,
        resource: { AddressBook: { Delete: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'AddressBook') && variantIs(resource, 'AddressBook')) {
        return isResourceActionContained(specifier.AddressBook, resource.AddressBook);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.AccessPolicy,
    resources: [
      {
        action: ResourceActionEnum.Read,
        resource: { AccessPolicy: { Read: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Update,
        resource: { AccessPolicy: { Update: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'AccessPolicy') && variantIs(resource, 'AccessPolicy')) {
        return isAccessPolicyResourceActionContained(specifier.AccessPolicy, resource.AccessPolicy);
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
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Create,
        resource: { ProposalPolicy: { Create: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { ProposalPolicy: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Update,
        resource: { ProposalPolicy: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Delete,
        resource: { ProposalPolicy: { Delete: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'ProposalPolicy') && variantIs(resource, 'ProposalPolicy')) {
        return isResourceActionContained(specifier.ProposalPolicy, resource.ProposalPolicy);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.System,
    resources: [
      {
        action: ResourceActionEnum.SystemInfoCapabilities,
        resource: { System: { Capabilities: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.SystemInfoConfig,
        resource: { System: { SystemInfo: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'System') && variantIs(resource, 'System')) {
        return isSystemResourceActionContained(specifier.System, resource.System);
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
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'ChangeCanister') && variantIs(resource, 'ChangeCanister')) {
        return isChangeCanisterResourceActionContained(
          specifier.ChangeCanister,
          resource.ChangeCanister,
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
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { Proposal: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'Proposal') && variantIs(resource, 'Proposal')) {
        return isProposalResourceActionContained(specifier.Proposal, resource.Proposal);
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
          canEdit: false,
        },
        {
          action: ResourceActionEnum.Update,
          resource: { Account: { Update: { Id: accountId } } },
          allow: defaultAllowLevels(),
          canEdit: false,
        },
        {
          action: ResourceActionEnum.Transfer,
          resource: { Account: { Update: { Id: accountId } } },
          allow: defaultAllowLevels(),
          canEdit: false,
        },
      ],
      match(specifier: Resource, resource: Resource): boolean {
        if (variantIs(specifier, 'Account') && variantIs(resource, 'Account')) {
          return isAccountResourceActionContained(specifier.Account, resource.Account);
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
          canEdit: false,
        },
        {
          action: ResourceActionEnum.Update,
          resource: { User: { Update: { Id: userId } } },
          allow: defaultAllowLevels(),
          canEdit: false,
        },
      ],
      match(specifier: Resource, resource: Resource): boolean {
        if (variantIs(specifier, 'User') && variantIs(resource, 'User')) {
          return isUserResourceActionContained(specifier.User, resource.User);
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
          canEdit: false,
        },
        {
          action: ResourceActionEnum.Update,
          resource: { UserGroup: { Update: { Id: groupId } } },
          allow: defaultAllowLevels(),
          canEdit: false,
        },
        {
          action: ResourceActionEnum.Delete,
          resource: { UserGroup: { Delete: { Id: groupId } } },
          allow: defaultAllowLevels(),
          canEdit: false,
        },
      ],
      match(specifier: Resource, resource: Resource): boolean {
        if (variantIs(specifier, 'UserGroup') && variantIs(resource, 'UserGroup')) {
          return isResourceActionContained(specifier.UserGroup, resource.UserGroup);
        }

        return false;
      },
    },
  ];
};
