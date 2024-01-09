import { User, UserPrivilege } from '~/generated/wallet/wallet.did';

export enum PolicyType {
  VariableApprovalThreshold = 'VariableApprovalThreshold',
  FixedApprovalThreshold = 'FixedApprovalThreshold',
}

export enum AccountTransferStatus {
  Created = 'created',
  Failed = 'failed',
  Processing = 'processing',
  Completed = 'completed',
  Unknown = 'unknown',
}

export enum WalletProposalType {
  Transfer = 'Transfer',
  AccountEdit = 'AccountEdit',
  Unknown = 'Unknown',
}

export interface AuthenticatedUser {
  me: User;
  privileges: UserPrivilege[];
}
