import { Resource, UUID } from '~/generated/station/station.did';
import {
  AggregatedResoucePermissions,
  AuthScopeEnum,
  ResourceAccessAllowLevels,
  ResourceActionEnum,
  ResourceTypeEnum,
} from '~/types/permissions.types';
import {
  isPermissionResourceActionContained,
  isAccountResourceActionContained,
  isRequestResourceActionContained,
  isResourceActionContained,
  isSystemResourceActionContained,
  isUserResourceActionContained,
  isExternalCanisterActionContained,
} from '~/utils/permissions.utils';
import { variantIs } from '~/utils/helper.utils';

export const defaultAllowLevels = (): ResourceAccessAllowLevels => ({
  authScope: AuthScopeEnum.Restrictred,
  membersOfGroup: [],
  specificUsers: [],
});

export const globalPermissions = (): AggregatedResoucePermissions[] => [
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
    resourceType: ResourceTypeEnum.Permission,
    resources: [
      {
        action: ResourceActionEnum.Read,
        resource: { Permission: { Read: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Update,
        resource: { Permission: { Update: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'Permission') && variantIs(resource, 'Permission')) {
        return isPermissionResourceActionContained(specifier.Permission, resource.Permission);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.RequestPolicy,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { RequestPolicy: { List: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Create,
        resource: { RequestPolicy: { Create: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { RequestPolicy: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Update,
        resource: { RequestPolicy: { Update: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Delete,
        resource: { RequestPolicy: { Delete: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'RequestPolicy') && variantIs(resource, 'RequestPolicy')) {
        return isResourceActionContained(specifier.RequestPolicy, resource.RequestPolicy);
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
      {
        action: ResourceActionEnum.ManageSystemInfo,
        resource: { System: { ManageSystemInfo: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.SystemUpgrade,
        resource: { System: { Upgrade: null } },
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
    resourceType: ResourceTypeEnum.Request,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { Request: { List: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { Request: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'Request') && variantIs(resource, 'Request')) {
        return isRequestResourceActionContained(specifier.Request, resource.Request);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.ExternalCanister,
    resources: [
      {
        action: ResourceActionEnum.List,
        resource: { ExternalCanister: { List: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Create,
        resource: { ExternalCanister: { Create: null } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Read,
        resource: { ExternalCanister: { Read: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Change,
        resource: { ExternalCanister: { Change: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
      {
        action: ResourceActionEnum.Fund,
        resource: { ExternalCanister: { Fund: { Any: null } } },
        allow: defaultAllowLevels(),
        canEdit: false,
      },
    ],
    match(specifier: Resource, resource: Resource): boolean {
      if (variantIs(specifier, 'ExternalCanister') && variantIs(resource, 'ExternalCanister')) {
        return isExternalCanisterActionContained(
          specifier.ExternalCanister,
          resource.ExternalCanister,
        );
      }

      return false;
    },
  },
];

export const getAccountPermissions = (accountId: UUID): AggregatedResoucePermissions[] => {
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

export const getUserPermissions = (userId: UUID): AggregatedResoucePermissions[] => {
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

export const getUserGroupPermissions = (groupId: UUID): AggregatedResoucePermissions[] => {
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
