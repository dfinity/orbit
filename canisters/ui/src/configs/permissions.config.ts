import { variantIs } from '~/core';
import { AccessPolicy, ResourceSpecifier } from '~/generated/wallet/wallet.did';
import {
  ResourceAccessUserSpecifiers,
  ResourceActionEnum,
  ResourceTypeEnum,
} from '~/types/permissions.types';
import {
  isCanisterSettingsActionSpecifier,
  isChangeCanisterActionSpecifier,
  isCommonActionSpecifierContained,
  isProposalActionSpecifier,
  isTransferActionSpecifier,
} from '~/utils/permissions.utils';

export interface ResourcePermissionsSpecifier {
  action: ResourceActionEnum;
  specifier: ResourceSpecifier;
  users: ResourceAccessUserSpecifiers;
}

export interface ResourcePermissions {
  resourceType: ResourceTypeEnum;
  specifiers: ResourcePermissionsSpecifier[];
  match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean;
}

const defaultUserSpecifiers = () => ({
  allUsers: {},
  membersOfGroup: { groups: {} },
  specificUsers: { users: {} },
});

export const globalResourcePermissions = (): ResourcePermissions[] => [
  {
    resourceType: ResourceTypeEnum.User,
    specifiers: [
      {
        action: ResourceActionEnum.List,
        specifier: { User: { List: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Create,
        specifier: { User: { Create: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Read,
        specifier: { User: { Read: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Update,
        specifier: { User: { Update: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Delete,
        specifier: { User: { Delete: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'User') && variantIs(policy.resource, 'User')) {
        return isCommonActionSpecifierContained(specifier.User, policy.resource.User);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.UserGroup,
    specifiers: [
      {
        action: ResourceActionEnum.List,
        specifier: { UserGroup: { List: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Create,
        specifier: { UserGroup: { Create: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Read,
        specifier: { UserGroup: { Read: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Update,
        specifier: { UserGroup: { Update: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Delete,
        specifier: { UserGroup: { Delete: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'UserGroup') && variantIs(policy.resource, 'UserGroup')) {
        return isCommonActionSpecifierContained(specifier.UserGroup, policy.resource.UserGroup);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.Account,
    specifiers: [
      {
        action: ResourceActionEnum.List,
        specifier: { Account: { List: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Create,
        specifier: { Account: { Create: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Read,
        specifier: { Account: { Read: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Update,
        specifier: { Account: { Update: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Delete,
        specifier: { Account: { Delete: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'Account') && variantIs(policy.resource, 'Account')) {
        return isCommonActionSpecifierContained(specifier.Account, policy.resource.Account);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.AddressBook,
    specifiers: [
      {
        action: ResourceActionEnum.List,
        specifier: { AddressBook: { List: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Create,
        specifier: { AddressBook: { Create: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Read,
        specifier: { AddressBook: { Read: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Update,
        specifier: { AddressBook: { Update: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Delete,
        specifier: { AddressBook: { Delete: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'AddressBook') && variantIs(policy.resource, 'AddressBook')) {
        return isCommonActionSpecifierContained(specifier.AddressBook, policy.resource.AddressBook);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.AccessPolicy,
    specifiers: [
      {
        action: ResourceActionEnum.List,
        specifier: { AccessPolicy: { List: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Create,
        specifier: { AccessPolicy: { Create: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Read,
        specifier: { AccessPolicy: { Read: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Update,
        specifier: { AccessPolicy: { Update: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Delete,
        specifier: { AccessPolicy: { Delete: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'AccessPolicy') && variantIs(policy.resource, 'AccessPolicy')) {
        return isCommonActionSpecifierContained(
          specifier.AccessPolicy,
          policy.resource.AccessPolicy,
        );
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.ProposalPolicy,
    specifiers: [
      {
        action: ResourceActionEnum.List,
        specifier: { ProposalPolicy: { List: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Create,
        specifier: { ProposalPolicy: { Create: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Read,
        specifier: { ProposalPolicy: { Read: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Update,
        specifier: { ProposalPolicy: { Update: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Delete,
        specifier: { ProposalPolicy: { Delete: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'ProposalPolicy') && variantIs(policy.resource, 'ProposalPolicy')) {
        return isCommonActionSpecifierContained(
          specifier.ProposalPolicy,
          policy.resource.ProposalPolicy,
        );
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.CanisterSettings,
    specifiers: [
      {
        action: ResourceActionEnum.ReadPublicConfig,
        specifier: { CanisterSettings: { Read: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.ReadSensitiveConfig,
        specifier: { CanisterSettings: { ReadConfig: null } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (
        variantIs(specifier, 'CanisterSettings') &&
        variantIs(policy.resource, 'CanisterSettings')
      ) {
        return isCanisterSettingsActionSpecifier(
          specifier.CanisterSettings,
          policy.resource.CanisterSettings,
        );
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.ChangeCanister,
    specifiers: [
      {
        action: ResourceActionEnum.Create,
        specifier: { ChangeCanister: { Create: null } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'ChangeCanister') && variantIs(policy.resource, 'ChangeCanister')) {
        return isChangeCanisterActionSpecifier(
          specifier.ChangeCanister,
          policy.resource.ChangeCanister,
        );
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.Transfer,
    specifiers: [
      {
        action: ResourceActionEnum.Create,
        specifier: { Transfer: { Create: { account: { Any: null } } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Read,
        specifier: { Transfer: { Read: { account: { Any: null } } } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Delete,
        specifier: { Transfer: { Delete: { account: { Any: null } } } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'Transfer') && variantIs(policy.resource, 'Transfer')) {
        return isTransferActionSpecifier(specifier.Transfer, policy.resource.Transfer);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.Proposal,
    specifiers: [
      {
        action: ResourceActionEnum.List,
        specifier: { Proposal: { List: null } },
        users: defaultUserSpecifiers(),
      },
      {
        action: ResourceActionEnum.Read,
        specifier: { Proposal: { Read: { Any: null } } },
        users: defaultUserSpecifiers(),
      },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'Proposal') && variantIs(policy.resource, 'Proposal')) {
        return isProposalActionSpecifier(specifier.Proposal, policy.resource.Proposal);
      }

      return false;
    },
  },
];
