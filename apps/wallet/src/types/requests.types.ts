import { Request, RequestAdditionalInfo } from '~/generated/station/station.did';

export enum ListRequestsOperationTypeGroup {
  User = 'user',
  UserGroup = 'user_group',
  Account = 'account',
  Transfer = 'transfer',
  AddressBook = 'address_book_entry',
  RequestPolicy = 'request_policy',
  Permission = 'permission',
  ChangeCanister = 'change_canister',
  SystemInfo = 'system_info',
  ManagedCanister = 'managed_canister',
}

export enum RequestApprovalStatusEnum {
  Approved = 'Approved',
  Rejected = 'Rejected',
}

export interface RequestWithDetails {
  request: Request;
  additionalInfo?: RequestAdditionalInfo;
}
