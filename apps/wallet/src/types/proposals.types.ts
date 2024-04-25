import { Proposal, ProposalAdditionalInfo } from '~/generated/wallet/wallet.did';

export enum ListProposalsOperationTypeGroup {
  User = 'user',
  UserGroup = 'user_group',
  Account = 'account',
  Transfer = 'transfer',
  AddressBook = 'address_book_entry',
  ProposalPolicy = 'proposal_policy',
  AccessPolicy = 'access_policy',
  ChangeCanister = 'change_canister',
}

export interface ProposalWithDetails {
  proposal: Proposal;
  additionalInfo?: ProposalAdditionalInfo;
}
