import {
  ListRequestsOperationType,
  Request,
  RequestApprovalStatus,
  RequestOperation,
  RequestStatus,
} from '~/generated/station/station.did';
import { i18n } from '~/plugins/i18n.plugin';
import { CsvRow, CsvTable } from '~/types/app.types';
import {
  ListRequestsOperationTypeGroup,
  RequestApprovalStatusEnum,
  RequestWithDetails,
} from '~/types/requests.types';
import { RequestOperationEnum, RequestStatusEnum } from '~/types/station.types';
import { detectAddressFormat } from '~/utils/asset.utils';
import { formatBalance, stringify, unreachable, variantIs } from '~/utils/helper.utils';

export const mapRequestsOperationTypeToGroup = (
  operationType: ListRequestsOperationType,
): ListRequestsOperationTypeGroup => {
  if (variantIs(operationType, 'AddAccount') || variantIs(operationType, 'EditAccount')) {
    return ListRequestsOperationTypeGroup.Account;
  }

  if (variantIs(operationType, 'Transfer')) {
    return ListRequestsOperationTypeGroup.Transfer;
  }

  if (variantIs(operationType, 'AddUser') || variantIs(operationType, 'EditUser')) {
    return ListRequestsOperationTypeGroup.User;
  }

  if (
    variantIs(operationType, 'AddAddressBookEntry') ||
    variantIs(operationType, 'EditAddressBookEntry') ||
    variantIs(operationType, 'RemoveAddressBookEntry')
  ) {
    return ListRequestsOperationTypeGroup.AddressBook;
  }

  if (
    variantIs(operationType, 'AddRequestPolicy') ||
    variantIs(operationType, 'EditRequestPolicy') ||
    variantIs(operationType, 'RemoveRequestPolicy')
  ) {
    return ListRequestsOperationTypeGroup.RequestPolicy;
  }

  if (variantIs(operationType, 'EditPermission')) {
    return ListRequestsOperationTypeGroup.Permission;
  }

  if (
    variantIs(operationType, 'SystemUpgrade') ||
    variantIs(operationType, 'SetDisasterRecovery')
  ) {
    return ListRequestsOperationTypeGroup.SystemUpgrade;
  }

  if (
    variantIs(operationType, 'AddUserGroup') ||
    variantIs(operationType, 'EditUserGroup') ||
    variantIs(operationType, 'RemoveUserGroup')
  ) {
    return ListRequestsOperationTypeGroup.UserGroup;
  }

  if (variantIs(operationType, 'ManageSystemInfo')) {
    return ListRequestsOperationTypeGroup.SystemInfo;
  }

  if (
    variantIs(operationType, 'ChangeExternalCanister') ||
    variantIs(operationType, 'CreateExternalCanister') ||
    variantIs(operationType, 'CallExternalCanister') ||
    variantIs(operationType, 'ConfigureExternalCanister') ||
    variantIs(operationType, 'FundExternalCanister') ||
    variantIs(operationType, 'PruneExternalCanister') ||
    variantIs(operationType, 'RestoreExternalCanister') ||
    variantIs(operationType, 'SnapshotExternalCanister')
  ) {
    return ListRequestsOperationTypeGroup.ExternalCanister;
  }

  if (
    variantIs(operationType, 'AddAsset') ||
    variantIs(operationType, 'EditAsset') ||
    variantIs(operationType, 'RemoveAsset')
  ) {
    return ListRequestsOperationTypeGroup.Asset;
  }

  return unreachable(operationType);
};

export const mapRequestStatusToText = (status: RequestStatus): RequestStatusEnum => {
  if (variantIs(status, 'Approved')) {
    return RequestStatusEnum.Approved;
  }

  if (variantIs(status, 'Cancelled')) {
    return RequestStatusEnum.Cancelled;
  }

  if (variantIs(status, 'Completed')) {
    return RequestStatusEnum.Completed;
  }

  if (variantIs(status, 'Created')) {
    return RequestStatusEnum.Created;
  }

  if (variantIs(status, 'Failed')) {
    return RequestStatusEnum.Failed;
  }

  if (variantIs(status, 'Processing')) {
    return RequestStatusEnum.Processing;
  }

  if (variantIs(status, 'Rejected')) {
    return RequestStatusEnum.Rejected;
  }

  if (variantIs(status, 'Scheduled')) {
    return RequestStatusEnum.Scheduled;
  }

  return unreachable(status);
};

export const mapRequestStatusToReason = (status: RequestStatus): string => {
  if (
    variantIs(status, 'Approved') ||
    variantIs(status, 'Completed') ||
    variantIs(status, 'Created') ||
    variantIs(status, 'Rejected')
  ) {
    return '';
  }

  if (variantIs(status, 'Cancelled')) {
    return status.Cancelled.reason?.[0] ?? '';
  }

  if (variantIs(status, 'Failed')) {
    return status.Failed.reason?.[0] ?? '';
  }

  if (variantIs(status, 'Processing')) {
    return status.Processing.started_at;
  }

  if (variantIs(status, 'Scheduled')) {
    return status.Scheduled.scheduled_at;
  }

  return unreachable(status);
};

export const mapRequestOperationEnumToTranslation = (operation: RequestOperationEnum): string => {
  const translationKey = `requests.types.${operation.toLowerCase()}.title`;
  const translation = i18n.global.t(translationKey);

  if (translation === translationKey) {
    return operation;
  }

  return translation;
};

export const mapRequestCsvHeaderToTranslation = (header: string): string => {
  const translationKey = `requests.headers.${header}`;
  const translation = i18n.global.t(translationKey);

  if (translation === translationKey) {
    return header;
  }

  return translation;
};

export const mapRequestOperationToTypeEnum = (
  operation: RequestOperation,
): RequestOperationEnum => {
  if (variantIs(operation, 'AddAccount')) {
    return RequestOperationEnum.AddAccount;
  }
  if (variantIs(operation, 'EditAccount')) {
    return RequestOperationEnum.EditAccount;
  }
  if (variantIs(operation, 'Transfer')) {
    return RequestOperationEnum.Transfer;
  }
  if (variantIs(operation, 'AddUser')) {
    return RequestOperationEnum.AddUser;
  }
  if (variantIs(operation, 'EditUser')) {
    return RequestOperationEnum.EditUser;
  }
  if (variantIs(operation, 'AddAddressBookEntry')) {
    return RequestOperationEnum.AddAddressBookEntry;
  }
  if (variantIs(operation, 'EditAddressBookEntry')) {
    return RequestOperationEnum.EditAddressBookEntry;
  }
  if (variantIs(operation, 'RemoveAddressBookEntry')) {
    return RequestOperationEnum.RemoveAddressBookEntry;
  }
  if (variantIs(operation, 'AddRequestPolicy')) {
    return RequestOperationEnum.AddRequestPolicy;
  }
  if (variantIs(operation, 'EditRequestPolicy')) {
    return RequestOperationEnum.EditRequestPolicy;
  }
  if (variantIs(operation, 'RemoveRequestPolicy')) {
    return RequestOperationEnum.RemoveRequestPolicy;
  }
  if (variantIs(operation, 'EditPermission')) {
    return RequestOperationEnum.EditPermission;
  }
  if (variantIs(operation, 'SystemUpgrade')) {
    return RequestOperationEnum.SystemUpgrade;
  }
  if (variantIs(operation, 'AddUserGroup')) {
    return RequestOperationEnum.AddUserGroup;
  }
  if (variantIs(operation, 'EditUserGroup')) {
    return RequestOperationEnum.EditUserGroup;
  }
  if (variantIs(operation, 'RemoveUserGroup')) {
    return RequestOperationEnum.RemoveUserGroup;
  }
  if (variantIs(operation, 'ManageSystemInfo')) {
    return RequestOperationEnum.ManageSystemInfo;
  }
  if (variantIs(operation, 'ChangeExternalCanister')) {
    return RequestOperationEnum.ChangeExternalCanister;
  }
  if (variantIs(operation, 'CreateExternalCanister')) {
    return RequestOperationEnum.CreateExternalCanister;
  }
  if (variantIs(operation, 'CallExternalCanister')) {
    return RequestOperationEnum.CallExternalCanister;
  }
  if (variantIs(operation, 'ConfigureExternalCanister')) {
    return RequestOperationEnum.ConfigureExternalCanister;
  }
  if (variantIs(operation, 'FundExternalCanister')) {
    return RequestOperationEnum.FundExternalCanister;
  }
  if (variantIs(operation, 'PruneExternalCanister')) {
    return RequestOperationEnum.PruneExternalCanister;
  }
  if (variantIs(operation, 'SnapshotExternalCanister')) {
    return RequestOperationEnum.SnapshotExternalCanister;
  }
  if (variantIs(operation, 'RestoreExternalCanister')) {
    return RequestOperationEnum.RestoreExternalCanister;
  }
  if (variantIs(operation, 'SetDisasterRecovery')) {
    return RequestOperationEnum.SetDisasterRecovery;
  }
  if (variantIs(operation, 'AddAsset')) {
    return RequestOperationEnum.AddAsset;
  }
  if (variantIs(operation, 'EditAsset')) {
    return RequestOperationEnum.EditAsset;
  }
  if (variantIs(operation, 'RemoveAsset')) {
    return RequestOperationEnum.RemoveAsset;
  }

  return unreachable(operation);
};

export const mapListRequestsOperationTypeToGroups = (
  types: ListRequestsOperationType[],
): Map<ListRequestsOperationTypeGroup, ListRequestsOperationType[]> => {
  const groups = new Map<ListRequestsOperationTypeGroup, ListRequestsOperationType[]>();

  for (const type of types) {
    const group = mapRequestsOperationTypeToGroup(type);
    const groupTypes = groups.get(group) ?? [];
    groupTypes.push(type);

    groups.set(group, Array.from(new Set(groupTypes)));
  }

  return groups;
};

export const mapRequestOperationToListRequestsOperationType = (
  requestOperation: RequestOperation,
): ListRequestsOperationType => {
  if (variantIs(requestOperation, 'AddAccount')) {
    return { AddAccount: null };
  } else if (variantIs(requestOperation, 'EditAccount')) {
    return { EditAccount: null };
  } else if (variantIs(requestOperation, 'Transfer')) {
    return { Transfer: [] };
  } else if (variantIs(requestOperation, 'AddUser')) {
    return { AddUser: null };
  } else if (variantIs(requestOperation, 'EditUser')) {
    return { EditUser: null };
  } else if (variantIs(requestOperation, 'AddAddressBookEntry')) {
    return { AddAddressBookEntry: null };
  } else if (variantIs(requestOperation, 'EditAddressBookEntry')) {
    return { EditAddressBookEntry: null };
  } else if (variantIs(requestOperation, 'RemoveAddressBookEntry')) {
    return { RemoveAddressBookEntry: null };
  } else if (variantIs(requestOperation, 'AddRequestPolicy')) {
    return { AddRequestPolicy: null };
  } else if (variantIs(requestOperation, 'EditRequestPolicy')) {
    return { EditRequestPolicy: null };
  } else if (variantIs(requestOperation, 'RemoveRequestPolicy')) {
    return { RemoveRequestPolicy: null };
  } else if (variantIs(requestOperation, 'EditPermission')) {
    return { EditPermission: null };
  } else if (variantIs(requestOperation, 'SystemUpgrade')) {
    return { SystemUpgrade: null };
  } else if (variantIs(requestOperation, 'AddUserGroup')) {
    return { AddUserGroup: null };
  } else if (variantIs(requestOperation, 'EditUserGroup')) {
    return { EditUserGroup: null };
  } else if (variantIs(requestOperation, 'RemoveUserGroup')) {
    return { RemoveUserGroup: null };
  } else if (variantIs(requestOperation, 'ManageSystemInfo')) {
    return { ManageSystemInfo: null };
  } else if (variantIs(requestOperation, 'ChangeExternalCanister')) {
    return { ChangeExternalCanister: [] };
  } else if (variantIs(requestOperation, 'CreateExternalCanister')) {
    return { CreateExternalCanister: null };
  } else if (variantIs(requestOperation, 'ConfigureExternalCanister')) {
    return { ConfigureExternalCanister: [] };
  } else if (variantIs(requestOperation, 'CallExternalCanister')) {
    return { CallExternalCanister: [] };
  } else if (variantIs(requestOperation, 'FundExternalCanister')) {
    return { FundExternalCanister: [] };
  } else if (variantIs(requestOperation, 'PruneExternalCanister')) {
    return { PruneExternalCanister: [] };
  } else if (variantIs(requestOperation, 'RestoreExternalCanister')) {
    return { RestoreExternalCanister: [] };
  } else if (variantIs(requestOperation, 'SnapshotExternalCanister')) {
    return { SnapshotExternalCanister: [] };
  } else if (variantIs(requestOperation, 'SetDisasterRecovery')) {
    return { SetDisasterRecovery: null };
  } else if (variantIs(requestOperation, 'AddAsset')) {
    return { AddAsset: null };
  } else if (variantIs(requestOperation, 'EditAsset')) {
    return { EditAsset: null };
  } else if (variantIs(requestOperation, 'RemoveAsset')) {
    return { RemoveAsset: null };
  } else {
    return unreachable(requestOperation);
  }
};

export const mapListRequestsOperationTypeGroupToCsvHeaders = (
  group: ListRequestsOperationTypeGroup,
): CsvRow => {
  const headers: CsvRow = {};
  if (group === ListRequestsOperationTypeGroup.Account) {
    headers.account_id = 'Account ID';
    headers.account_name = 'Account Name';
    headers.token = 'Token';
    headers.address = 'Address';
  }

  if (group === ListRequestsOperationTypeGroup.User) {
    headers.user_id = 'User ID';
    headers.user_name = 'User Name';
  }

  if (group === ListRequestsOperationTypeGroup.UserGroup) {
    headers.group_id = 'Group ID';
    headers.group_name = 'Group Name';
  }

  if (group === ListRequestsOperationTypeGroup.AddressBook) {
    headers.address_book_id = 'Address Book ID';
    headers.blockchain = 'Blockchain';
    headers.address_owner = 'Address Owner';
    headers.address = 'Address';
  }

  if (group === ListRequestsOperationTypeGroup.RequestPolicy) {
    headers.policy_id = 'Policy ID';
  }

  if (group === ListRequestsOperationTypeGroup.SystemUpgrade) {
    headers.change_target = 'Change Target';
    headers.wasm_checksum = 'Wasm Checksum';
  }

  if (group === ListRequestsOperationTypeGroup.Transfer) {
    headers.from_account = 'From Account';
    headers.to = 'To';
    headers.amount = 'Amount';
    headers.fee = 'Fee';
    headers.comment = 'Comment';
    headers.from_account_address = 'From Account Address';
  }

  return headers;
};

const mapRequestToUserGroupCsvRow = (request: Request): CsvRow => {
  if (variantIs(request.operation, 'AddUserGroup')) {
    return {
      group_id: request.operation.AddUserGroup.user_group?.[0]?.id ?? '',
      group_name: request.operation.AddUserGroup.input.name,
    };
  }

  if (variantIs(request.operation, 'EditUserGroup')) {
    return {
      group_id: request.operation.EditUserGroup.input.user_group_id,
      group_name: request.operation.EditUserGroup.input.name,
    };
  }

  if (variantIs(request.operation, 'RemoveUserGroup')) {
    return {
      group_id: request.operation.RemoveUserGroup.input.user_group_id,
    };
  }

  return {};
};

const mapRequestToUserCsvRow = (request: Request): CsvRow => {
  if (variantIs(request.operation, 'AddUser')) {
    return {
      user_id: request.operation.AddUser.user?.[0]?.id ?? '',
      user_name: request.operation.AddUser.input.name,
      details: stringify({
        identities: request.operation.AddUser.input.identities?.map(i => i.toText()),
        status: request.operation.AddUser.input.status,
        groups: request.operation.AddUser.input.groups,
      }),
    };
  }

  if (variantIs(request.operation, 'EditUser')) {
    return {
      user_id: request.operation.EditUser.input.id,
      user_name: request.operation.EditUser.input.name?.[0] ?? '',
      details: stringify({
        identities: request.operation.EditUser.input.identities?.[0]?.map(i => i.toText()),
        status: request.operation.EditUser.input.status?.[0] ?? '',
        groups: request.operation.EditUser.input.groups?.[0] ?? '',
      }),
    };
  }

  return {};
};

const mapRequestToAccountCsvRow = (request: Request): CsvRow => {
  if (variantIs(request.operation, 'AddAccount')) {
    return {
      account_id: request.operation.AddAccount.account?.[0]?.id ?? '',
      account_name: request.operation.AddAccount.input.name,
      details: stringify({
        metadata: request.operation.AddAccount.input.metadata,
        assets: request.operation.AddAccount.input.assets,
        configs_request_policy: request.operation.AddAccount.input.configs_request_policy,
        transfer_request_policy: request.operation.AddAccount.input.transfer_request_policy,
      }),
    };
  }

  if (variantIs(request.operation, 'EditAccount')) {
    return {
      account_id: request.operation.EditAccount.input.account_id,
      account_name: request.operation.EditAccount.input.name?.[0] ?? '',
      details: stringify({
        configs_request_policy: request.operation.EditAccount.input.configs_request_policy,
        transfer_request_policy: request.operation.EditAccount.input.transfer_request_policy,
      }),
    };
  }
  return {};
};

const mapRequestToAddressBookCsvRow = (request: Request): CsvRow => {
  if (variantIs(request.operation, 'AddAddressBookEntry')) {
    return {
      address_book_id: request.operation.AddAddressBookEntry.address_book_entry?.[0]?.id ?? '',
      blockchain: request.operation.AddAddressBookEntry.input.blockchain,
      address_owner: request.operation.AddAddressBookEntry.input.address_owner,
      address: request.operation.AddAddressBookEntry.input.address,
      details: stringify({
        metadata: request.operation.AddAddressBookEntry.input.metadata,
      }),
    };
  }

  if (variantIs(request.operation, 'EditAddressBookEntry')) {
    return {
      address_book_id: request.operation.EditAddressBookEntry.input.address_book_entry_id,
      address_owner: request.operation.EditAddressBookEntry.input.address_owner?.[0] ?? '',
      details: stringify({
        change_metadata: request.operation.EditAddressBookEntry.input.change_metadata,
      }),
    };
  }

  if (variantIs(request.operation, 'RemoveAddressBookEntry')) {
    return {
      address_book_id: request.operation.RemoveAddressBookEntry.input.address_book_entry_id,
    };
  }

  return {};
};

const mapRequestToTransferCsvRow = (request: Request): CsvRow => {
  if (variantIs(request.operation, 'Transfer') && request.operation.Transfer.from_account?.[0]) {
    const account = request.operation.Transfer.from_account[0];

    const asset = request.operation.Transfer.from_asset;

    // to determine the `from address` we find a matching address to the format of the `to address`
    const maybeToAddressFormat = detectAddressFormat(
      asset.blockchain,
      request.operation.Transfer.input.to,
    );

    const fallbackAddress = account.addresses[0]?.address ?? '-';

    const fromAddress = maybeToAddressFormat
      ? (account.addresses.find(accountAddress => accountAddress.format === maybeToAddressFormat)
          ?.address ?? fallbackAddress)
      : fallbackAddress;

    return {
      from_account: account.name,
      from_account_address: fromAddress,
      from_asset: `${asset.name} (${asset.blockchain} / ${asset.name})`,
      to: request.operation.Transfer.input.to,
      amount:
        formatBalance(request.operation.Transfer.input.amount, asset.decimals) + ' ' + asset.symbol,
      fee: request.operation.Transfer.fee[0]
        ? formatBalance(request.operation.Transfer.fee[0], asset.decimals) + ' ' + asset.symbol
        : '',
      comment: request.summary[0] ?? '',
    };
  }

  return {};
};

const mapRequestToRequestPolicyCsvRow = (request: Request): CsvRow => {
  if (variantIs(request.operation, 'AddRequestPolicy')) {
    return {
      policy_id: request.operation.AddRequestPolicy.policy_id?.[0] ?? '',
      details: stringify(request.operation.AddRequestPolicy.input),
    };
  }

  if (variantIs(request.operation, 'EditRequestPolicy')) {
    return {
      policy_id: request.operation.EditRequestPolicy.input.policy_id,
      details: stringify(request.operation.EditRequestPolicy.input),
    };
  }

  if (variantIs(request.operation, 'RemoveRequestPolicy')) {
    return {
      policy_id: request.operation.RemoveRequestPolicy.input.policy_id,
    };
  }

  return {};
};

const mapRequestToPermissionCsvRow = (request: Request): CsvRow => {
  if (variantIs(request.operation, 'EditPermission')) {
    return {
      details: stringify(request.operation.EditPermission.input),
    };
  }

  return {};
};

const mapRequestToSystemUpgradeCsvRow = (request: Request): CsvRow => {
  if (variantIs(request.operation, 'SystemUpgrade')) {
    const args = request.operation.SystemUpgrade.arg_checksum[0]
      ? request.operation.SystemUpgrade.arg_checksum[0]
      : '';

    if (variantIs(request.operation.SystemUpgrade.target, 'UpgradeStation')) {
      return {
        change_target: 'station',
        wasm_checksum: request.operation.SystemUpgrade.module_checksum,
        details: stringify({ args }),
      };
    }

    if (variantIs(request.operation.SystemUpgrade.target, 'UpgradeUpgrader')) {
      return {
        change_target: 'upgrader',
        wasm_checksum: request.operation.SystemUpgrade.module_checksum,
        details: stringify({ args }),
      };
    }
  }

  return {};
};

export const mapRequestToCsvRow = (
  group: ListRequestsOperationTypeGroup,
  request: Request,
): CsvRow => {
  switch (group) {
    case ListRequestsOperationTypeGroup.Account:
      return mapRequestToAccountCsvRow(request);
    case ListRequestsOperationTypeGroup.User:
      return mapRequestToUserCsvRow(request);
    case ListRequestsOperationTypeGroup.UserGroup:
      return mapRequestToUserGroupCsvRow(request);
    case ListRequestsOperationTypeGroup.AddressBook:
      return mapRequestToAddressBookCsvRow(request);
    case ListRequestsOperationTypeGroup.RequestPolicy:
      return mapRequestToRequestPolicyCsvRow(request);
    case ListRequestsOperationTypeGroup.Permission:
      return mapRequestToPermissionCsvRow(request);
    case ListRequestsOperationTypeGroup.SystemUpgrade:
      return mapRequestToSystemUpgradeCsvRow(request);
    case ListRequestsOperationTypeGroup.Transfer:
      return mapRequestToTransferCsvRow(request);
  }

  return {};
};

export const mapRequestsToCsvTable = (
  group: ListRequestsOperationTypeGroup,
  requests: RequestWithDetails[],
): CsvTable => {
  const table: CsvTable = { headers: {}, rows: [] };
  const headers: CsvRow = {
    id: 'ID',
    requester: 'Requester',
    status: 'Status',
    status_reason: 'Status Reason',
    created: 'Created',
    expires: 'Expires',
    operation_type: 'Operation Type',
    ...mapListRequestsOperationTypeGroupToCsvHeaders(group),
    details: 'Details',
  };

  for (const key in headers) {
    headers[key] = mapRequestCsvHeaderToTranslation(key);
  }

  const rows = requests.map(entry => {
    const row: CsvRow = {
      id: entry.request.id,
      requester: entry.additionalInfo?.requester_name ?? entry.request.requested_by,
      status: mapRequestStatusToText(entry.request.status),
      status_reason: mapRequestStatusToReason(entry.request.status),
      created: entry.request.created_at,
      expires: entry.request.expiration_dt,
      operation_type: mapRequestOperationEnumToTranslation(
        mapRequestOperationToTypeEnum(entry.request.operation),
      ),
      ...mapRequestToCsvRow(group, entry.request),
    };

    return row;
  });

  table.headers = headers;
  table.rows = rows;

  return table;
};

export const mapRequestApprovalStatusEnumToVariant = (
  status: RequestApprovalStatusEnum,
): RequestApprovalStatus => {
  switch (status) {
    case RequestApprovalStatusEnum.Approved:
      return { Approved: null };

    case RequestApprovalStatusEnum.Rejected:
      return { Rejected: null };

    default:
      return unreachable(status);
  }
};
