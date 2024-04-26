import { Proposal, ProposalAdditionalInfo } from '~/generated/station/station.did';

export enum ListProposalsOperationTypeGroup {
  User = 'user',
  UserGroup = 'user_group',
  Account = 'account',
  Transfer = 'transfer',
  AddressBook = 'address_book_entry',
  ProposalPolicy = 'proposal_policy',
  Permission = 'permission',
  ChangeCanister = 'change_canister',
}

export interface ProposalWithDetails {
  proposal: Proposal;
  additionalInfo?: ProposalAdditionalInfo;
}
