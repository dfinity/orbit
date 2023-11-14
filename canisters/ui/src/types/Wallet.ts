export enum PolicyType {
  VariableApprovalThreshold = 'VariableApprovalThreshold',
  FixedApprovalThreshold = 'FixedApprovalThreshold',
}

export enum AccountTransferStatus {
  Pending = 'pending',
  Rejected = 'rejected',
  Failed = 'failed',
  Approved = 'approved',
  Cancelled = 'cancelled',
  Submitted = 'submitted',
  Processing = 'processing',
  Completed = 'completed',
  Unknown = 'unknown',
}

export enum WalletProposalType {
  Transfer = 'Transfer',
  Unknown = 'Unknown',
}
