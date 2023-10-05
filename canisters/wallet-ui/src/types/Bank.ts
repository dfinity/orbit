export enum WalletPolicyType {
  VariableApprovalThreshold = 'VariableApprovalThreshold',
  FixedApprovalThreshold = 'FixedApprovalThreshold',
}

export enum WalletTransferStatus {
  Pending = 'pending',
  Rejected = 'rejected',
  Approved = 'approved',
  Cancelled = 'cancelled',
  Submitted = 'submitted',
  Processing = 'processing',
  Completed = 'completed',
  Unknown = 'unknown',
}
