export const idlFactory = ({ IDL }) => {
  const ProposalPolicyCriteria = IDL.Rec();
  const WalletUpgrade = IDL.Record({
    'owners' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  const WalletInit = IDL.Record({
    'owners' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'upgrader_wasm_module' : IDL.Vec(IDL.Nat8),
  });
  const WalletInstall = IDL.Variant({
    'Upgrade' : WalletUpgrade,
    'Init' : WalletInit,
  });
  const UUID = IDL.Text;
  const UserGroupId = UUID;
  const UserGroup = IDL.Record({ 'id' : UserGroupId, 'name' : IDL.Text });
  const AssetMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const AssetSymbol = IDL.Text;
  const WalletAsset = IDL.Record({
    'standards' : IDL.Vec(IDL.Text),
    'metadata' : IDL.Vec(AssetMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'symbol' : AssetSymbol,
  });
  const Config = IDL.Record({
    'user_groups' : IDL.Vec(UserGroup),
    'supported_assets' : IDL.Vec(WalletAsset),
  });
  const Error = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const GetConfigResult = IDL.Variant({
    'Ok' : IDL.Record({ 'config' : Config }),
    'Err' : Error,
  });
  const TimestampRFC3339 = IDL.Text;
  const ProposalExecutionSchedule = IDL.Variant({
    'Immediate' : IDL.Null,
    'Scheduled' : IDL.Record({ 'execution_time' : TimestampRFC3339 }),
  });
  const CommonSpecifier = IDL.Variant({
    'Id' : IDL.Vec(UUID),
    'Any' : IDL.Null,
    'Group' : IDL.Vec(UUID),
  });
  const ProposalActionSpecifier = IDL.Variant({
    'List' : IDL.Null,
    'Read' : CommonSpecifier,
  });
  const ChangeCanisterActionSpecifier = IDL.Variant({ 'Create' : IDL.Null });
  const TransferSpecifier = IDL.Record({ 'account' : CommonSpecifier });
  const TransferActionSpecifier = IDL.Variant({
    'Read' : TransferSpecifier,
    'Delete' : TransferSpecifier,
    'Create' : TransferSpecifier,
  });
  const CanisterSettingsActionSpecifier = IDL.Variant({
    'Read' : IDL.Null,
    'ReadConfig' : IDL.Null,
  });
  const CommonActionSpecifier = IDL.Variant({
    'List' : IDL.Null,
    'Read' : CommonSpecifier,
    'Delete' : CommonSpecifier,
    'Create' : IDL.Null,
    'Update' : CommonSpecifier,
  });
  const ResourceType = IDL.Variant({
    'User' : IDL.Null,
    'ProposalPolicy' : IDL.Null,
    'Account' : IDL.Null,
    'AddressBook' : IDL.Null,
    'AccessPolicy' : IDL.Null,
    'UserGroup' : IDL.Null,
  });
  const ResourceSpecifier = IDL.Variant({
    'Proposal' : ProposalActionSpecifier,
    'ChangeCanister' : ChangeCanisterActionSpecifier,
    'Transfer' : TransferActionSpecifier,
    'CanisterSettings' : CanisterSettingsActionSpecifier,
    'Common' : IDL.Record({
      'action' : CommonActionSpecifier,
      'resource_type' : ResourceType,
    }),
  });
  const AccessControlUserSpecifier = CommonSpecifier;
  const EditAccessPolicyOperationInput = IDL.Record({
    'resource' : IDL.Opt(ResourceSpecifier),
    'user' : IDL.Opt(AccessControlUserSpecifier),
    'policy_id' : UUID,
  });
  const AddUserGroupOperationInput = IDL.Record({ 'name' : IDL.Text });
  const RemoveProposalPolicyOperationInput = IDL.Record({ 'policy_id' : UUID });
  const UserStatus = IDL.Variant({
    'Inactive' : IDL.Null,
    'Active' : IDL.Null,
  });
  const AddUserOperationInput = IDL.Record({
    'status' : UserStatus,
    'groups' : IDL.Vec(UUID),
    'name' : IDL.Opt(IDL.Text),
    'identities' : IDL.Vec(IDL.Principal),
  });
  const EditUserGroupOperationInput = IDL.Record({
    'name' : IDL.Text,
    'user_group_id' : UUID,
  });
  const UserSpecifier = IDL.Variant({
    'Id' : IDL.Vec(UUID),
    'Any' : IDL.Null,
    'Group' : IDL.Vec(UserGroupId),
    'Proposer' : IDL.Null,
    'Owner' : IDL.Null,
  });
  const AccountSpecifier = CommonSpecifier;
  const ProposalSpecifier = IDL.Variant({
    'EditAccessPolicy' : CommonSpecifier,
    'AddUserGroup' : IDL.Null,
    'RemoveProposalPolicy' : CommonSpecifier,
    'AddUser' : IDL.Null,
    'EditUserGroup' : CommonSpecifier,
    'AddProposalPolicy' : IDL.Null,
    'ChangeCanister' : IDL.Null,
    'EditProposalPolicy' : CommonSpecifier,
    'EditUser' : UserSpecifier,
    'Transfer' : TransferSpecifier,
    'EditAccount' : AccountSpecifier,
    'AddAccessPolicy' : IDL.Null,
    'RemoveAccessPolicy' : CommonSpecifier,
    'RemoveUserGroup' : CommonSpecifier,
    'AddAccount' : IDL.Null,
  });
  ProposalPolicyCriteria.fill(
    IDL.Variant({
      'Or' : IDL.Vec(ProposalPolicyCriteria),
      'And' : IDL.Vec(ProposalPolicyCriteria),
      'Not' : ProposalPolicyCriteria,
      'MinimumVotes' : IDL.Tuple(UserSpecifier, IDL.Nat16),
      'ApprovalThreshold' : IDL.Tuple(UserSpecifier, IDL.Nat16),
      'AutoAdopted' : IDL.Null,
    })
  );
  const AddProposalPolicyOperationInput = IDL.Record({
    'specifier' : ProposalSpecifier,
    'criteria' : ProposalPolicyCriteria,
  });
  const ChangeCanisterTarget = IDL.Variant({
    'UpgradeUpgrader' : IDL.Null,
    'UpgradeCanister' : IDL.Principal,
    'UpgradeWallet' : IDL.Null,
  });
  const ChangeCanisterOperationInput = IDL.Record({
    'arg' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'target' : ChangeCanisterTarget,
    'checksum' : IDL.Vec(IDL.Nat8),
    'module' : IDL.Vec(IDL.Nat8),
  });
  const EditProposalPolicyOperationInput = IDL.Record({
    'specifier' : IDL.Opt(ProposalSpecifier),
    'criteria' : IDL.Opt(ProposalPolicyCriteria),
    'policy_id' : UUID,
  });
  const EditUserOperationInput = IDL.Record({
    'id' : UUID,
    'groups' : IDL.Opt(IDL.Vec(UUID)),
    'name' : IDL.Opt(IDL.Text),
    'identities' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  const TransferMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const NetworkId = IDL.Text;
  const Network = IDL.Record({ 'id' : NetworkId, 'name' : IDL.Text });
  const TransferOperationInput = IDL.Record({
    'to' : IDL.Text,
    'fee' : IDL.Opt(IDL.Nat),
    'from_account_id' : UUID,
    'metadata' : IDL.Vec(TransferMetadata),
    'network' : IDL.Opt(Network),
    'amount' : IDL.Nat,
  });
  const AccountPolicies = IDL.Record({
    'edit' : IDL.Opt(ProposalPolicyCriteria),
    'transfer' : IDL.Opt(ProposalPolicyCriteria),
  });
  const EditAccountOperationInput = IDL.Record({
    'account_id' : UUID,
    'owners' : IDL.Opt(IDL.Vec(UUID)),
    'name' : IDL.Opt(IDL.Text),
    'policies' : IDL.Opt(AccountPolicies),
  });
  const AddAccessPolicyOperationInput = IDL.Record({
    'resource' : ResourceSpecifier,
    'user' : AccessControlUserSpecifier,
  });
  const RemoveAccessPolicyOperationInput = IDL.Record({ 'policy_id' : UUID });
  const RemoveUserGroupOperationInput = IDL.Record({ 'user_group_id' : UUID });
  const AccountMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const AddAccountOperationInput = IDL.Record({
    'owners' : IDL.Vec(UUID),
    'metadata' : IDL.Vec(AccountMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'standard' : IDL.Text,
    'policies' : AccountPolicies,
  });
  const ProposalOperationInput = IDL.Variant({
    'EditAccessPolicy' : EditAccessPolicyOperationInput,
    'AddUserGroup' : AddUserGroupOperationInput,
    'RemoveProposalPolicy' : RemoveProposalPolicyOperationInput,
    'AddUser' : AddUserOperationInput,
    'EditUserGroup' : EditUserGroupOperationInput,
    'AddProposalPolicy' : AddProposalPolicyOperationInput,
    'ChangeCanister' : ChangeCanisterOperationInput,
    'EditProposalPolicy' : EditProposalPolicyOperationInput,
    'EditUser' : EditUserOperationInput,
    'Transfer' : TransferOperationInput,
    'EditAccount' : EditAccountOperationInput,
    'AddAccessPolicy' : AddAccessPolicyOperationInput,
    'RemoveAccessPolicy' : RemoveAccessPolicyOperationInput,
    'RemoveUserGroup' : RemoveUserGroupOperationInput,
    'AddAccount' : AddAccountOperationInput,
  });
  const CreateProposalInput = IDL.Record({
    'title' : IDL.Opt(IDL.Text),
    'execution_plan' : IDL.Opt(ProposalExecutionSchedule),
    'summary' : IDL.Opt(IDL.Text),
    'operation' : ProposalOperationInput,
  });
  const ProposalStatus = IDL.Variant({
    'Failed' : IDL.Record({ 'reason' : IDL.Opt(IDL.Text) }),
    'Rejected' : IDL.Null,
    'Scheduled' : IDL.Record({ 'scheduled_at' : TimestampRFC3339 }),
    'Adopted' : IDL.Null,
    'Cancelled' : IDL.Record({ 'reason' : IDL.Opt(IDL.Text) }),
    'Processing' : IDL.Record({ 'started_at' : TimestampRFC3339 }),
    'Created' : IDL.Null,
    'Completed' : IDL.Record({ 'completed_at' : TimestampRFC3339 }),
  });
  const ProposalVoteStatus = IDL.Variant({
    'Rejected' : IDL.Null,
    'Accepted' : IDL.Null,
  });
  const ProposalVote = IDL.Record({
    'status' : ProposalVoteStatus,
    'user_id' : UUID,
    'status_reason' : IDL.Opt(IDL.Text),
    'decided_at' : TimestampRFC3339,
  });
  const EditAccessPolicyOperation = IDL.Record({
    'input' : EditAccessPolicyOperationInput,
  });
  const AddUserGroupOperation = IDL.Record({
    'user_group' : IDL.Opt(UserGroup),
    'input' : AddUserGroupOperationInput,
  });
  const RemoveProposalPolicyOperation = IDL.Record({
    'input' : RemoveProposalPolicyOperationInput,
  });
  const User = IDL.Record({
    'id' : UUID,
    'status' : UserStatus,
    'groups' : IDL.Vec(UserGroup),
    'name' : IDL.Opt(IDL.Text),
    'last_modification_timestamp' : TimestampRFC3339,
    'identities' : IDL.Vec(IDL.Principal),
  });
  const AddUserOperation = IDL.Record({
    'user' : IDL.Opt(User),
    'input' : AddUserOperationInput,
  });
  const EditUserGroupOperation = IDL.Record({
    'input' : EditUserGroupOperationInput,
  });
  const ProposalPolicy = IDL.Record({
    'id' : UUID,
    'specifier' : ProposalSpecifier,
    'criteria' : ProposalPolicyCriteria,
  });
  const AddProposalPolicyOperation = IDL.Record({
    'input' : AddProposalPolicyOperationInput,
    'policy' : IDL.Opt(ProposalPolicy),
  });
  const ChangeCanisterOperation = IDL.Record({
    'input' : ChangeCanisterOperationInput,
  });
  const EditProposalPolicyOperation = IDL.Record({
    'input' : EditProposalPolicyOperationInput,
  });
  const EditUserOperation = IDL.Record({ 'input' : EditUserOperationInput });
  const AccountBalanceInfo = IDL.Record({
    'decimals' : IDL.Nat32,
    'balance' : IDL.Nat,
    'last_update_timestamp' : TimestampRFC3339,
  });
  const Account = IDL.Record({
    'id' : UUID,
    'decimals' : IDL.Nat32,
    'balance' : IDL.Opt(AccountBalanceInfo),
    'owners' : IDL.Vec(UUID),
    'metadata' : IDL.Vec(AccountMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'last_modification_timestamp' : TimestampRFC3339,
    'standard' : IDL.Text,
    'symbol' : AssetSymbol,
    'policies' : AccountPolicies,
  });
  const TransferOperation = IDL.Record({
    'network' : Network,
    'from_account' : IDL.Opt(Account),
    'input' : TransferOperationInput,
  });
  const EditAccountOperation = IDL.Record({
    'input' : EditAccountOperationInput,
  });
  const AccessPolicy = IDL.Record({
    'id' : UUID,
    'resource' : ResourceSpecifier,
    'user' : AccessControlUserSpecifier,
  });
  const AddAccessPolicyOperation = IDL.Record({
    'input' : AddAccessPolicyOperationInput,
    'policy' : IDL.Opt(AccessPolicy),
  });
  const RemoveAccessPolicyOperation = IDL.Record({
    'input' : RemoveAccessPolicyOperationInput,
  });
  const RemoveUserGroupOperation = IDL.Record({
    'input' : RemoveUserGroupOperationInput,
  });
  const AddAccountOperation = IDL.Record({
    'account' : IDL.Opt(Account),
    'input' : AddAccountOperationInput,
  });
  const ProposalOperation = IDL.Variant({
    'EditAccessPolicy' : EditAccessPolicyOperation,
    'AddUserGroup' : AddUserGroupOperation,
    'RemoveProposalPolicy' : RemoveProposalPolicyOperation,
    'AddUser' : AddUserOperation,
    'EditUserGroup' : EditUserGroupOperation,
    'AddProposalPolicy' : AddProposalPolicyOperation,
    'ChangeCanister' : ChangeCanisterOperation,
    'EditProposalPolicy' : EditProposalPolicyOperation,
    'EditUser' : EditUserOperation,
    'Transfer' : TransferOperation,
    'EditAccount' : EditAccountOperation,
    'AddAccessPolicy' : AddAccessPolicyOperation,
    'RemoveAccessPolicy' : RemoveAccessPolicyOperation,
    'RemoveUserGroup' : RemoveUserGroupOperation,
    'AddAccount' : AddAccountOperation,
  });
  const Proposal = IDL.Record({
    'id' : UUID,
    'status' : ProposalStatus,
    'title' : IDL.Text,
    'execution_plan' : ProposalExecutionSchedule,
    'expiration_dt' : TimestampRFC3339,
    'votes' : IDL.Vec(ProposalVote),
    'created_at' : TimestampRFC3339,
    'summary' : IDL.Opt(IDL.Text),
    'operation' : ProposalOperation,
    'proposed_by' : UUID,
  });
  const CreateProposalResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposal' : Proposal }),
    'Err' : Error,
  });
  const FetchAccountBalancesInput = IDL.Record({
    'account_ids' : IDL.Vec(UUID),
  });
  const AccountBalance = IDL.Record({
    'account_id' : UUID,
    'decimals' : IDL.Nat32,
    'balance' : IDL.Nat,
    'last_update_timestamp' : TimestampRFC3339,
  });
  const FetchAccountBalancesResult = IDL.Variant({
    'Ok' : IDL.Record({ 'balances' : IDL.Vec(AccountBalance) }),
    'Err' : Error,
  });
  const GetAccessPolicyInput = IDL.Record({ 'id' : UUID });
  const GetAccessPolicyResult = IDL.Variant({
    'Ok' : IDL.Record({ 'policy' : AccessPolicy }),
    'Err' : Error,
  });
  const GetAccountInput = IDL.Record({ 'account_id' : UUID });
  const GetAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'account' : Account }),
    'Err' : Error,
  });
  const GetProposalInput = IDL.Record({ 'proposal_id' : UUID });
  const GetProposalResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposal' : Proposal }),
    'Err' : Error,
  });
  const GetProposalPolicyInput = IDL.Record({ 'id' : UUID });
  const GetProposalPolicyResult = IDL.Variant({
    'Ok' : IDL.Record({ 'policy' : ProposalPolicy }),
    'Err' : Error,
  });
  const GetTransfersInput = IDL.Record({ 'transfer_ids' : IDL.Vec(UUID) });
  const TransferStatus = IDL.Variant({
    'Failed' : IDL.Record({ 'reason' : IDL.Text }),
    'Processing' : IDL.Record({ 'started_at' : TimestampRFC3339 }),
    'Created' : IDL.Null,
    'Completed' : IDL.Record({
      'signature' : IDL.Opt(IDL.Text),
      'hash' : IDL.Opt(IDL.Text),
      'completed_at' : TimestampRFC3339,
    }),
  });
  const Transfer = IDL.Record({
    'id' : UUID,
    'to' : IDL.Text,
    'fee' : IDL.Nat,
    'status' : TransferStatus,
    'from_account_id' : UUID,
    'metadata' : IDL.Vec(TransferMetadata),
    'network' : Network,
    'amount' : IDL.Nat,
  });
  const GetTransfersResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfers' : IDL.Vec(Transfer) }),
    'Err' : Error,
  });
  const GetUserInput = IDL.Record({ 'user_id' : UUID });
  const GetUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : Error,
  });
  const GetUserGroupInput = IDL.Record({ 'user_group_id' : UUID });
  const GetUserGroupResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user_group' : UserGroup }),
    'Err' : Error,
  });
  const HealthStatus = IDL.Variant({
    'Healthy' : IDL.Null,
    'Uninitialized' : IDL.Null,
  });
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'status_code' : IDL.Nat16,
  });
  const PaginationInput = IDL.Record({
    'offset' : IDL.Opt(IDL.Nat64),
    'limit' : IDL.Opt(IDL.Nat16),
  });
  const ListAccessPoliciesInput = PaginationInput;
  const ListAccessPoliciesResult = IDL.Variant({
    'Ok' : IDL.Record({
      'next_offset' : IDL.Opt(IDL.Nat64),
      'policies' : IDL.Vec(AccessPolicy),
    }),
    'Err' : Error,
  });
  const TransferStatusType = IDL.Variant({
    'Failed' : IDL.Null,
    'Processing' : IDL.Null,
    'Created' : IDL.Null,
    'Completed' : IDL.Null,
  });
  const ListAccountTransfersInput = IDL.Record({
    'account_id' : UUID,
    'status' : IDL.Opt(TransferStatusType),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'from_dt' : IDL.Opt(TimestampRFC3339),
  });
  const TransferListItem = IDL.Record({
    'to' : IDL.Text,
    'status' : TransferStatus,
    'created_at' : TimestampRFC3339,
    'transfer_id' : UUID,
    'amount' : IDL.Nat,
  });
  const ListAccountTransfersResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfers' : IDL.Vec(TransferListItem) }),
    'Err' : Error,
  });
  const ListAccountResult = IDL.Variant({
    'Ok' : IDL.Record({ 'accounts' : IDL.Vec(Account) }),
    'Err' : Error,
  });
  const NotificationStatus = IDL.Variant({
    'Read' : IDL.Null,
    'Sent' : IDL.Null,
  });
  const NotificationTypeInput = IDL.Variant({
    'ProposalCreated' : IDL.Null,
    'SystemMessage' : IDL.Null,
  });
  const ListNotificationsInput = IDL.Record({
    'status' : IDL.Opt(NotificationStatus),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'from_dt' : IDL.Opt(TimestampRFC3339),
    'notification_type' : IDL.Opt(NotificationTypeInput),
  });
  const ProposalOperationType = IDL.Variant({
    'EditAccessPolicy' : IDL.Null,
    'AddUserGroup' : IDL.Null,
    'RemoveProposalPolicy' : IDL.Null,
    'AddUser' : IDL.Null,
    'EditUserGroup' : IDL.Null,
    'AddProposalPolicy' : IDL.Null,
    'ChangeCanister' : IDL.Null,
    'EditProposalPolicy' : IDL.Null,
    'EditUser' : IDL.Null,
    'Transfer' : IDL.Null,
    'EditAccount' : IDL.Null,
    'AddAccessPolicy' : IDL.Null,
    'RemoveAccessPolicy' : IDL.Null,
    'RemoveUserGroup' : IDL.Null,
    'AddAccount' : IDL.Null,
  });
  const NotificationType = IDL.Variant({
    'ProposalCreated' : IDL.Record({
      'account_id' : IDL.Opt(UUID),
      'operation_type' : ProposalOperationType,
      'user_id' : IDL.Opt(UUID),
      'proposal_id' : UUID,
    }),
    'SystemMessage' : IDL.Null,
  });
  const Notification = IDL.Record({
    'id' : UUID,
    'status' : NotificationStatus,
    'title' : IDL.Text,
    'created_at' : TimestampRFC3339,
    'notification_type' : NotificationType,
    'message' : IDL.Opt(IDL.Text),
    'target_user_id' : UUID,
  });
  const ListNotificationsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'notifications' : IDL.Vec(Notification) }),
    'Err' : Error,
  });
  const ListProposalPoliciesInput = PaginationInput;
  const ListProposalPoliciesResult = IDL.Variant({
    'Ok' : IDL.Record({
      'next_offset' : IDL.Opt(IDL.Nat64),
      'policies' : IDL.Vec(ProposalPolicy),
    }),
    'Err' : Error,
  });
  const ProposalStatusCode = IDL.Variant({
    'Failed' : IDL.Null,
    'Rejected' : IDL.Null,
    'Scheduled' : IDL.Null,
    'Adopted' : IDL.Null,
    'Cancelled' : IDL.Null,
    'Processing' : IDL.Null,
    'Created' : IDL.Null,
    'Completed' : IDL.Null,
  });
  const ListProposalsOperationType = IDL.Variant({
    'EditAccessPolicy' : IDL.Null,
    'AddUserGroup' : IDL.Null,
    'RemoveProposalPolicy' : IDL.Null,
    'AddUser' : IDL.Null,
    'EditUserGroup' : IDL.Null,
    'AddProposalPolicy' : IDL.Null,
    'ChangeCanister' : IDL.Null,
    'EditProposalPolicy' : IDL.Null,
    'EditUser' : IDL.Null,
    'Transfer' : IDL.Opt(UUID),
    'EditAccount' : IDL.Null,
    'AddAccessPolicy' : IDL.Null,
    'RemoveAccessPolicy' : IDL.Null,
    'RemoveUserGroup' : IDL.Null,
    'AddAccount' : IDL.Null,
  });
  const ListProposalsInput = IDL.Record({
    'voter_ids' : IDL.Opt(IDL.Vec(UUID)),
    'expiration_from_dt' : IDL.Opt(TimestampRFC3339),
    'created_to_dt' : IDL.Opt(TimestampRFC3339),
    'statuses' : IDL.Opt(IDL.Vec(ProposalStatusCode)),
    'proposer_ids' : IDL.Opt(IDL.Vec(UUID)),
    'expiration_to_dt' : IDL.Opt(TimestampRFC3339),
    'paginate' : IDL.Opt(PaginationInput),
    'operation_types' : IDL.Opt(IDL.Vec(ListProposalsOperationType)),
    'created_from_dt' : IDL.Opt(TimestampRFC3339),
  });
  const PaginationInfo = IDL.Record({
    'total' : IDL.Nat64,
    'next_offset' : IDL.Opt(IDL.Nat64),
  });
  const ListProposalsResult = IDL.Variant({
    'Ok' : IDL.Record({
      'pagination' : PaginationInfo,
      'proposals' : IDL.Vec(Proposal),
    }),
    'Err' : Error,
  });
  const ListUserGroupResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user_groups' : IDL.Vec(UserGroup) }),
    'Err' : Error,
  });
  const ListUsersInput = PaginationInput;
  const ListUsersResult = IDL.Variant({
    'Ok' : IDL.Record({
      'users' : IDL.Vec(User),
      'next_offset' : IDL.Opt(IDL.Nat64),
    }),
    'Err' : Error,
  });
  const MarkNotificationsReadInput = IDL.Record({
    'notification_ids' : IDL.Vec(UUID),
    'read' : IDL.Bool,
  });
  const MarkNotificationReadResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : Error,
  });
  const UserPrivilege = IDL.Variant({
    'AddUserGroup' : IDL.Null,
    'ListUserGroups' : IDL.Null,
    'AddUser' : IDL.Null,
    'ListUsers' : IDL.Null,
    'AddProposalPolicy' : IDL.Null,
    'ListProposalPolicies' : IDL.Null,
    'ListAccounts' : IDL.Null,
    'ListAccessPolicies' : IDL.Null,
    'AddAccessPolicy' : IDL.Null,
    'AddAccount' : IDL.Null,
  });
  const MeResult = IDL.Variant({
    'Ok' : IDL.Record({ 'me' : User, 'privileges' : IDL.Vec(UserPrivilege) }),
    'Err' : Error,
  });
  const VoteOnProposalInput = IDL.Record({
    'approve' : IDL.Bool,
    'proposal_id' : UUID,
    'reason' : IDL.Opt(IDL.Text),
  });
  const VoteOnProposalResult = IDL.Variant({
    'Ok' : IDL.Record({ 'proposal' : Proposal }),
    'Err' : Error,
  });
  const WalletSettings = IDL.Record({
    'owners' : IDL.Vec(User),
    'last_upgrade_timestamp' : TimestampRFC3339,
  });
  const WalletSettingsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'settings' : WalletSettings }),
    'Err' : Error,
  });
  return IDL.Service({
    'config' : IDL.Func([], [GetConfigResult], ['query']),
    'create_proposal' : IDL.Func(
        [CreateProposalInput],
        [CreateProposalResult],
        [],
      ),
    'fetch_account_balances' : IDL.Func(
        [FetchAccountBalancesInput],
        [FetchAccountBalancesResult],
        [],
      ),
    'get_access_policy' : IDL.Func(
        [GetAccessPolicyInput],
        [GetAccessPolicyResult],
        ['query'],
      ),
    'get_account' : IDL.Func([GetAccountInput], [GetAccountResult], ['query']),
    'get_proposal' : IDL.Func(
        [GetProposalInput],
        [GetProposalResult],
        ['query'],
      ),
    'get_proposal_policy' : IDL.Func(
        [GetProposalPolicyInput],
        [GetProposalPolicyResult],
        ['query'],
      ),
    'get_transfers' : IDL.Func(
        [GetTransfersInput],
        [GetTransfersResult],
        ['query'],
      ),
    'get_user' : IDL.Func([GetUserInput], [GetUserResult], ['query']),
    'get_user_group' : IDL.Func(
        [GetUserGroupInput],
        [GetUserGroupResult],
        ['query'],
      ),
    'health_status' : IDL.Func([], [HealthStatus], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'list_access_policies' : IDL.Func(
        [ListAccessPoliciesInput],
        [ListAccessPoliciesResult],
        ['query'],
      ),
    'list_account_transfers' : IDL.Func(
        [ListAccountTransfersInput],
        [ListAccountTransfersResult],
        ['query'],
      ),
    'list_accounts' : IDL.Func([], [ListAccountResult], ['query']),
    'list_notifications' : IDL.Func(
        [ListNotificationsInput],
        [ListNotificationsResult],
        ['query'],
      ),
    'list_proposal_policies' : IDL.Func(
        [ListProposalPoliciesInput],
        [ListProposalPoliciesResult],
        ['query'],
      ),
    'list_proposals' : IDL.Func(
        [ListProposalsInput],
        [ListProposalsResult],
        ['query'],
      ),
    'list_user_groups' : IDL.Func([], [ListUserGroupResult], ['query']),
    'list_users' : IDL.Func([ListUsersInput], [ListUsersResult], ['query']),
    'mark_notifications_read' : IDL.Func(
        [MarkNotificationsReadInput],
        [MarkNotificationReadResult],
        [],
      ),
    'me' : IDL.Func([], [MeResult], ['query']),
    'vote_on_proposal' : IDL.Func(
        [VoteOnProposalInput],
        [VoteOnProposalResult],
        [],
      ),
    'wallet_settings' : IDL.Func([], [WalletSettingsResult], ['query']),
  });
};
export const init = ({ IDL }) => {
  const WalletUpgrade = IDL.Record({
    'owners' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  const WalletInit = IDL.Record({
    'owners' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'upgrader_wasm_module' : IDL.Vec(IDL.Nat8),
  });
  const WalletInstall = IDL.Variant({
    'Upgrade' : WalletUpgrade,
    'Init' : WalletInit,
  });
  return [IDL.Opt(WalletInstall)];
};
