export enum PolicyType {
  VariableApprovalThreshold = 'VariableApprovalThreshold',
  FixedApprovalThreshold = 'FixedApprovalThreshold',
}

export enum AccountTransferStatus {
  Created = 'created',
  Failed = 'failed',
  Cancelled = 'cancelled',
  Processing = 'processing',
  Completed = 'completed',
  Unknown = 'unknown',
}

export enum WalletProposalType {
  Transfer = 'Transfer',
  AccountEdit = 'AccountEdit',
  Unknown = 'Unknown',
}
