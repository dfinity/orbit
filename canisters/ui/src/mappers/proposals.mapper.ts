import {
  ListProposalsOperationType,
  Proposal,
  ProposalOperation,
  ProposalStatus,
} from '~/generated/wallet/wallet.did';
import { i18n } from '~/plugins/i18n.plugin';
import { CsvRow, CsvTable } from '~/types/app.types';
import { ListProposalsOperationTypeGroup, ProposalWithDetails } from '~/types/proposals.types';
import { ProposalOperationEnum, ProposalStatusEnum } from '~/types/wallet.types';
import { formatBalance, stringify, unreachable, variantIs } from '~/utils/helper.utils';

export const mapProposalsOperationTypeToGroup = (
  operationType: ListProposalsOperationType,
): ListProposalsOperationTypeGroup => {
  if (variantIs(operationType, 'AddAccount') || variantIs(operationType, 'EditAccount')) {
    return ListProposalsOperationTypeGroup.Account;
  }

  if (variantIs(operationType, 'Transfer')) {
    return ListProposalsOperationTypeGroup.Transfer;
  }

  if (variantIs(operationType, 'AddUser') || variantIs(operationType, 'EditUser')) {
    return ListProposalsOperationTypeGroup.User;
  }

  if (
    variantIs(operationType, 'AddAddressBookEntry') ||
    variantIs(operationType, 'EditAddressBookEntry') ||
    variantIs(operationType, 'RemoveAddressBookEntry')
  ) {
    return ListProposalsOperationTypeGroup.AddressBook;
  }

  if (
    variantIs(operationType, 'AddProposalPolicy') ||
    variantIs(operationType, 'EditProposalPolicy') ||
    variantIs(operationType, 'RemoveProposalPolicy')
  ) {
    return ListProposalsOperationTypeGroup.ProposalPolicy;
  }

  if (
    variantIs(operationType, 'AddAccessPolicy') ||
    variantIs(operationType, 'EditAccessPolicy') ||
    variantIs(operationType, 'RemoveAccessPolicy')
  ) {
    return ListProposalsOperationTypeGroup.AccessPolicy;
  }

  if (variantIs(operationType, 'ChangeCanister')) {
    return ListProposalsOperationTypeGroup.ChangeCanister;
  }

  if (
    variantIs(operationType, 'AddUserGroup') ||
    variantIs(operationType, 'EditUserGroup') ||
    variantIs(operationType, 'RemoveUserGroup')
  ) {
    return ListProposalsOperationTypeGroup.UserGroup;
  }

  return unreachable(operationType);
};

export const mapProposalStatusToText = (status: ProposalStatus): ProposalStatusEnum => {
  if (variantIs(status, 'Adopted')) {
    return ProposalStatusEnum.Adopted;
  }

  if (variantIs(status, 'Cancelled')) {
    return ProposalStatusEnum.Cancelled;
  }

  if (variantIs(status, 'Completed')) {
    return ProposalStatusEnum.Completed;
  }

  if (variantIs(status, 'Created')) {
    return ProposalStatusEnum.Created;
  }

  if (variantIs(status, 'Failed')) {
    return ProposalStatusEnum.Failed;
  }

  if (variantIs(status, 'Processing')) {
    return ProposalStatusEnum.Processing;
  }

  if (variantIs(status, 'Rejected')) {
    return ProposalStatusEnum.Rejected;
  }

  if (variantIs(status, 'Scheduled')) {
    return ProposalStatusEnum.Scheduled;
  }

  return unreachable(status);
};

export const mapProposalStatusToReason = (status: ProposalStatus): string => {
  if (
    variantIs(status, 'Adopted') ||
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

export const mapProposalOperationEnumToTranslation = (operation: ProposalOperationEnum): string => {
  const translationKey = `proposals.types.${operation.toLowerCase()}.title`;
  const translation = i18n.global.t(translationKey);

  if (translation === translationKey) {
    return operation;
  }

  return translation;
};

export const mapProposalCsvHeaderToTranslation = (header: string): string => {
  const translationKey = `proposals.headers.${header}`;
  const translation = i18n.global.t(translationKey);

  if (translation === translationKey) {
    return header;
  }

  return translation;
};

export const mapProposalOperationToTypeEnum = (
  operation: ProposalOperation,
): ProposalOperationEnum => {
  if (variantIs(operation, 'AddAccount')) {
    return ProposalOperationEnum.AddAccount;
  }
  if (variantIs(operation, 'EditAccount')) {
    return ProposalOperationEnum.EditAccount;
  }
  if (variantIs(operation, 'Transfer')) {
    return ProposalOperationEnum.Transfer;
  }
  if (variantIs(operation, 'AddUser')) {
    return ProposalOperationEnum.AddUser;
  }
  if (variantIs(operation, 'EditUser')) {
    return ProposalOperationEnum.EditUser;
  }
  if (variantIs(operation, 'AddAddressBookEntry')) {
    return ProposalOperationEnum.AddAddressBookEntry;
  }
  if (variantIs(operation, 'EditAddressBookEntry')) {
    return ProposalOperationEnum.EditAddressBookEntry;
  }
  if (variantIs(operation, 'RemoveAddressBookEntry')) {
    return ProposalOperationEnum.RemoveAddressBookEntry;
  }
  if (variantIs(operation, 'AddProposalPolicy')) {
    return ProposalOperationEnum.AddProposalPolicy;
  }
  if (variantIs(operation, 'EditProposalPolicy')) {
    return ProposalOperationEnum.EditProposalPolicy;
  }
  if (variantIs(operation, 'RemoveProposalPolicy')) {
    return ProposalOperationEnum.RemoveProposalPolicy;
  }
  if (variantIs(operation, 'AddAccessPolicy')) {
    return ProposalOperationEnum.AddAccessPolicy;
  }
  if (variantIs(operation, 'EditAccessPolicy')) {
    return ProposalOperationEnum.EditAccessPolicy;
  }
  if (variantIs(operation, 'RemoveAccessPolicy')) {
    return ProposalOperationEnum.RemoveAccessPolicy;
  }
  if (variantIs(operation, 'ChangeCanister')) {
    return ProposalOperationEnum.ChangeCanister;
  }
  if (variantIs(operation, 'AddUserGroup')) {
    return ProposalOperationEnum.AddUserGroup;
  }
  if (variantIs(operation, 'EditUserGroup')) {
    return ProposalOperationEnum.EditUserGroup;
  }
  if (variantIs(operation, 'RemoveUserGroup')) {
    return ProposalOperationEnum.RemoveUserGroup;
  }

  return unreachable(operation);
};

export const mapListProposalsOperationTypeToGroups = (
  types: ListProposalsOperationType[],
): Map<ListProposalsOperationTypeGroup, ListProposalsOperationType[]> => {
  const groups = new Map<ListProposalsOperationTypeGroup, ListProposalsOperationType[]>();

  for (const type of types) {
    const group = mapProposalsOperationTypeToGroup(type);
    const groupTypes = groups.get(group) ?? [];
    groupTypes.push(type);

    groups.set(group, Array.from(new Set(groupTypes)));
  }

  return groups;
};

export const mapListProposalsOperationTypeGroupToCsvHeaders = (
  group: ListProposalsOperationTypeGroup,
): CsvRow => {
  const headers: CsvRow = {};
  if (group === ListProposalsOperationTypeGroup.Account) {
    headers.account_id = 'Account ID';
    headers.account_name = 'Account Name';
    headers.token = 'Token';
    headers.address = 'Address';
  }

  if (group === ListProposalsOperationTypeGroup.User) {
    headers.user_id = 'User ID';
    headers.user_name = 'User Name';
  }

  if (group === ListProposalsOperationTypeGroup.UserGroup) {
    headers.group_id = 'Group ID';
    headers.group_name = 'Group Name';
  }

  if (group === ListProposalsOperationTypeGroup.AddressBook) {
    headers.address_book_id = 'Address Book ID';
    headers.blockchain = 'Blockchain';
    headers.address_owner = 'Address Owner';
    headers.address = 'Address';
  }

  if (group === ListProposalsOperationTypeGroup.ProposalPolicy) {
    headers.policy_id = 'Policy ID';
  }

  if (group === ListProposalsOperationTypeGroup.AccessPolicy) {
    headers.policy_id = 'Policy ID';
  }

  if (group === ListProposalsOperationTypeGroup.ChangeCanister) {
    headers.change_target = 'Change Target';
    headers.wasm_checksum = 'Wasm Checksum';
  }

  if (group === ListProposalsOperationTypeGroup.Transfer) {
    headers.from_account = 'From Account';
    headers.to = 'To';
    headers.amount = 'Amount';
    headers.fee = 'Fee';
  }

  return headers;
};

const mapProposalToUserGroupCsvRow = (proposal: Proposal): CsvRow => {
  if (variantIs(proposal.operation, 'AddUserGroup')) {
    return {
      group_id: proposal.operation.AddUserGroup.user_group?.[0]?.id ?? '',
      group_name: proposal.operation.AddUserGroup.input.name,
    };
  }

  if (variantIs(proposal.operation, 'EditUserGroup')) {
    return {
      group_id: proposal.operation.EditUserGroup.input.user_group_id,
      group_name: proposal.operation.EditUserGroup.input.name,
    };
  }

  if (variantIs(proposal.operation, 'RemoveUserGroup')) {
    return {
      group_id: proposal.operation.RemoveUserGroup.input.user_group_id,
    };
  }

  return {};
};

const mapProposalToUserCsvRow = (proposal: Proposal): CsvRow => {
  if (variantIs(proposal.operation, 'AddUser')) {
    return {
      user_id: proposal.operation.AddUser.user?.[0]?.id ?? '',
      user_name: proposal.operation.AddUser.input.name?.[0] ?? '',
      details: stringify({
        identities: proposal.operation.AddUser.input.identities?.map(i => i.toText()),
        status: proposal.operation.AddUser.input.status,
        groups: proposal.operation.AddUser.input.groups,
      }),
    };
  }

  if (variantIs(proposal.operation, 'EditUser')) {
    return {
      user_id: proposal.operation.EditUser.input.id,
      user_name: proposal.operation.EditUser.input.name?.[0] ?? '',
      details: stringify({
        identities: proposal.operation.EditUser.input.identities?.[0]?.map(i => i.toText()),
        status: proposal.operation.EditUser.input.status?.[0] ?? '',
        groups: proposal.operation.EditUser.input.groups?.[0] ?? '',
      }),
    };
  }

  return {};
};

const mapProposalToAccountCsvRow = (proposal: Proposal): CsvRow => {
  if (variantIs(proposal.operation, 'AddAccount')) {
    return {
      account_id: proposal.operation.AddAccount.account?.[0]?.id ?? '',
      account_name: proposal.operation.AddAccount.input.name,
      blockchain: proposal.operation.AddAccount.input.blockchain,
      details: stringify({
        metadata: proposal.operation.AddAccount.input.metadata,
        standard: proposal.operation.AddAccount.input.standard,
        policies: proposal.operation.AddAccount.input.policies,
      }),
    };
  }

  if (variantIs(proposal.operation, 'EditAccount')) {
    return {
      account_id: proposal.operation.EditAccount.input.account_id,
      account_name: proposal.operation.EditAccount.input.name?.[0] ?? '',
      details: stringify({
        policies: proposal.operation.EditAccount.input.policies,
      }),
    };
  }
  return {};
};

const mapProposalToAddressBookCsvRow = (proposal: Proposal): CsvRow => {
  if (variantIs(proposal.operation, 'AddAddressBookEntry')) {
    return {
      address_book_id: proposal.operation.AddAddressBookEntry.address_book_entry?.[0]?.id ?? '',
      blockchain: proposal.operation.AddAddressBookEntry.input.blockchain,
      address_owner: proposal.operation.AddAddressBookEntry.input.address_owner,
      address: proposal.operation.AddAddressBookEntry.input.address,
      details: stringify({
        metadata: proposal.operation.AddAddressBookEntry.input.metadata,
      }),
    };
  }

  if (variantIs(proposal.operation, 'EditAddressBookEntry')) {
    return {
      address_book_id: proposal.operation.EditAddressBookEntry.input.address_book_entry_id,
      address_owner: proposal.operation.EditAddressBookEntry.input.address_owner?.[0] ?? '',
      details: stringify({
        change_metadata: proposal.operation.EditAddressBookEntry.input.change_metadata,
      }),
    };
  }

  if (variantIs(proposal.operation, 'RemoveAddressBookEntry')) {
    return {
      address_book_id: proposal.operation.RemoveAddressBookEntry.input.address_book_entry_id,
    };
  }

  return {};
};

const mapProposalToTransferCsvRow = (proposal: Proposal): CsvRow => {
  if (variantIs(proposal.operation, 'Transfer') && proposal.operation.Transfer.from_account?.[0]) {
    const account = proposal.operation.Transfer.from_account[0];

    return {
      from_account: account.name,
      to: proposal.operation.Transfer.input.to,
      amount:
        formatBalance(proposal.operation.Transfer.input.amount, account.decimals) +
        ' ' +
        account.symbol,
      fee: proposal.operation.Transfer.input.fee[0]
        ? formatBalance(proposal.operation.Transfer.input.fee[0], account.decimals)
        : '',
    };
  }

  return {};
};

const mapProposalToProposalPolicyCsvRow = (proposal: Proposal): CsvRow => {
  if (variantIs(proposal.operation, 'AddProposalPolicy')) {
    return {
      policy_id: proposal.operation.AddProposalPolicy.policy_id?.[0] ?? '',
      details: stringify(proposal.operation.AddProposalPolicy.input),
    };
  }

  if (variantIs(proposal.operation, 'EditProposalPolicy')) {
    return {
      policy_id: proposal.operation.EditProposalPolicy.input.policy_id,
      details: stringify(proposal.operation.EditProposalPolicy.input),
    };
  }

  if (variantIs(proposal.operation, 'RemoveProposalPolicy')) {
    return {
      policy_id: proposal.operation.RemoveProposalPolicy.input.policy_id,
    };
  }

  return {};
};

const mapProposalToAccessPolicyCsvRow = (proposal: Proposal): CsvRow => {
  if (variantIs(proposal.operation, 'AddAccessPolicy')) {
    return {
      policy_id: proposal.operation.AddAccessPolicy.policy_id?.[0] ?? '',
      details: stringify(proposal.operation.AddAccessPolicy.input),
    };
  }

  if (variantIs(proposal.operation, 'EditAccessPolicy')) {
    return {
      policy_id: proposal.operation.EditAccessPolicy.input.policy_id,
      details: stringify(proposal.operation.EditAccessPolicy.input),
    };
  }

  if (variantIs(proposal.operation, 'RemoveAccessPolicy')) {
    return {
      policy_id: proposal.operation.RemoveAccessPolicy.input.policy_id,
    };
  }

  return {};
};

const mapProposalToChangeCanisterCsvRow = (proposal: Proposal): CsvRow => {
  if (variantIs(proposal.operation, 'ChangeCanister')) {
    const checksum = Buffer.from(proposal.operation.ChangeCanister.checksum).toString('hex');
    const args = proposal.operation.ChangeCanister.arg_checksum[0]
      ? Buffer.from(proposal.operation.ChangeCanister.arg_checksum[0]).toString('hex')
      : '';

    if (variantIs(proposal.operation.ChangeCanister.target, 'UpgradeWallet')) {
      return {
        change_target: 'wallet',
        wasm_checksum: checksum,
        details: stringify({ args }),
      };
    }

    if (variantIs(proposal.operation.ChangeCanister.target, 'UpgradeUpgrader')) {
      return {
        change_target: 'upgrader',
        wasm_checksum: checksum,
        details: stringify({ args }),
      };
    }

    if (variantIs(proposal.operation.ChangeCanister.target, 'UpgradeCanister')) {
      return {
        change_target: proposal.operation.ChangeCanister.target.UpgradeCanister.toText(),
        wasm_checksum: checksum,
        details: stringify({ args }),
      };
    }
  }

  return {};
};

export const mapProposalToCsvRow = (
  group: ListProposalsOperationTypeGroup,
  proposal: Proposal,
): CsvRow => {
  switch (group) {
    case ListProposalsOperationTypeGroup.Account:
      return mapProposalToAccountCsvRow(proposal);
    case ListProposalsOperationTypeGroup.User:
      return mapProposalToUserCsvRow(proposal);
    case ListProposalsOperationTypeGroup.UserGroup:
      return mapProposalToUserGroupCsvRow(proposal);
    case ListProposalsOperationTypeGroup.AddressBook:
      return mapProposalToAddressBookCsvRow(proposal);
    case ListProposalsOperationTypeGroup.ProposalPolicy:
      return mapProposalToProposalPolicyCsvRow(proposal);
    case ListProposalsOperationTypeGroup.AccessPolicy:
      return mapProposalToAccessPolicyCsvRow(proposal);
    case ListProposalsOperationTypeGroup.ChangeCanister:
      return mapProposalToChangeCanisterCsvRow(proposal);
    case ListProposalsOperationTypeGroup.Transfer:
      return mapProposalToTransferCsvRow(proposal);
  }

  return {};
};

export const mapProposalsToCsvTable = (
  group: ListProposalsOperationTypeGroup,
  proposals: ProposalWithDetails[],
): CsvTable => {
  const table: CsvTable = { headers: {}, rows: [] };
  const headers: CsvRow = {
    id: 'ID',
    proposer: 'Proposer',
    status: 'Status',
    status_reason: 'Status Reason',
    created: 'Created',
    expires: 'Expires',
    operation_type: 'Operation Type',
    ...mapListProposalsOperationTypeGroupToCsvHeaders(group),
    details: 'Details',
  };

  for (const key in headers) {
    headers[key] = mapProposalCsvHeaderToTranslation(key);
  }

  const rows = proposals.map(entry => {
    const row: CsvRow = {
      id: entry.proposal.id,
      proposer: entry.additionalInfo?.proposer_name[0] ?? entry.proposal.proposed_by,
      status: mapProposalStatusToText(entry.proposal.status),
      status_reason: mapProposalStatusToReason(entry.proposal.status),
      created: entry.proposal.created_at,
      expires: entry.proposal.expiration_dt,
      operation_type: mapProposalOperationEnumToTranslation(
        mapProposalOperationToTypeEnum(entry.proposal.operation),
      ),
      ...mapProposalToCsvRow(group, entry.proposal),
    };

    return row;
  });

  table.headers = headers;
  table.rows = rows;

  return table;
};
