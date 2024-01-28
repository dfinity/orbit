export enum ResourceTypeEnum {
  AccessPolicy = 'AccessPolicy',
  Account = 'Account',
  AddressBook = 'AddressBook',
  ProposalPolicy = 'ProposalPolicy',
  User = 'User',
  UserGroup = 'UserGroup',
}

export enum ResourceActionEnum {
  List = 'List',
  Create = 'Create',
  Delete = 'Delete',
  Read = 'Read',
  Update = 'Update',
}

export enum CommonSpecifierEnum {
  Any = 'Any',
  Id = 'Id',
  Group = 'Group',
}

export enum ResourceSpecifierEnum {
  ChangeCanister = 'ChangeCanister',
  Transfer = 'Transfer',
  CanisterSettings = 'CanisterSettings',
  Common = 'Common',
}
