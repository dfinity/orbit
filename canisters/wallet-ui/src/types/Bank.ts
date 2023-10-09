export enum WalletPolicyType {
  VariableApprovalThreshold = 'VariableApprovalThreshold',
  FixedApprovalThreshold = 'FixedApprovalThreshold',
}

export enum WalletTransferStatus {
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

export enum BankOperationType {
  ApproveTransfer = 'approve-transfer',
  Unknown = 'unknown',
}

