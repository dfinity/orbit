import { Request, RequestAdditionalInfo } from '~/generated/station/station.did';

export enum ListRequestsOperationTypeGroup {
  User = 'user',
  UserGroup = 'user_group',
  Account = 'account',
  Transfer = 'transfer',
  AddressBook = 'address_book_entry',
  RequestPolicy = 'request_policy',
  Permission = 'permission',
  SystemUpgrade = 'system_upgrade',
  SystemInfo = 'system_info',
  ExternalCanister = 'external_canister',
  Asset = 'asset',
}

export enum RequestApprovalStatusEnum {
  Approved = 'Approved',
  Rejected = 'Rejected',
}

export interface RequestWithDetails {
  request: Request;
  additionalInfo?: RequestAdditionalInfo;
}
