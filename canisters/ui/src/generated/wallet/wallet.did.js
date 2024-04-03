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
  const AssetMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const AssetSymbol = IDL.Text;
  const WalletAsset = IDL.Record({
    'metadata' : IDL.Vec(AssetMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'standard' : IDL.Text,
    'symbol' : AssetSymbol,
  });
  const Config = IDL.Record({ 'supported_assets' : IDL.Vec(WalletAsset) });
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
  const UUID = IDL.Text;
  const ResourceId = IDL.Variant({ 'Id' : UUID, 'Any' : IDL.Null });
  const UserResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Read' : ResourceId,
    'Create' : IDL.Null,
    'Update' : ResourceId,
  });
  const ResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Read' : ResourceId,
    'Delete' : ResourceId,
    'Create' : IDL.Null,
    'Update' : ResourceId,
  });
  const SettingsResourceAction = IDL.Variant({
    'Read' : IDL.Null,
    'ReadConfig' : IDL.Null,
  });
  const AccountResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Read' : ResourceId,
    'Create' : IDL.Null,
    'Transfer' : ResourceId,
    'Update' : ResourceId,
  });
  const ProposalResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Read' : ResourceId,
  });
  const ChangeCanisterResourceAction = IDL.Variant({ 'Create' : IDL.Null });
  const AccessPolicyResourceAction = IDL.Variant({
    'Read' : IDL.Null,
    'Update' : IDL.Null,
  });
  const Resource = IDL.Variant({
    'User' : UserResourceAction,
    'ProposalPolicy' : ResourceAction,
    'Settings' : SettingsResourceAction,
    'Account' : AccountResourceAction,
    'AddressBook' : ResourceAction,
    'Proposal' : ProposalResourceAction,
    'ChangeCanister' : ChangeCanisterResourceAction,
    'AccessPolicy' : AccessPolicyResourceAction,
    'UserGroup' : ResourceAction,
  });
  const AuthScope = IDL.Variant({
    'Authenticated' : IDL.Null,
    'Public' : IDL.Null,
    'Restricted' : IDL.Null,
  });
  const EditAccessPolicyOperationInput = IDL.Record({
    'resource' : Resource,
    'user_groups' : IDL.Opt(IDL.Vec(UUID)),
    'auth_scope' : IDL.Opt(AuthScope),
    'users' : IDL.Opt(IDL.Vec(UUID)),
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
  const RemoveAddressBookEntryOperationInput = IDL.Record({
    'address_book_entry_id' : UUID,
  });
  const AddressBookMetadata = IDL.Record({
    'key' : IDL.Text,
    'value' : IDL.Text,
  });
  const ChangeAddressBookMetadata = IDL.Variant({
    'OverrideSpecifiedBy' : IDL.Vec(AddressBookMetadata),
    'RemoveKeys' : IDL.Vec(IDL.Text),
    'ReplaceAllBy' : IDL.Vec(AddressBookMetadata),
  });
  const EditAddressBookEntryOperationInput = IDL.Record({
    'change_metadata' : IDL.Opt(ChangeAddressBookMetadata),
    'address_book_entry_id' : UUID,
    'address_owner' : IDL.Opt(IDL.Text),
  });
  const ResourceSpecifier = IDL.Variant({
    'Any' : IDL.Null,
    'Resource' : Resource,
  });
  const ResourceIds = IDL.Variant({ 'Any' : IDL.Null, 'Ids' : IDL.Vec(UUID) });
  const ProposalSpecifier = IDL.Variant({
    'EditAccessPolicy' : ResourceSpecifier,
    'AddUserGroup' : IDL.Null,
    'RemoveProposalPolicy' : ResourceIds,
    'AddUser' : IDL.Null,
    'EditUserGroup' : ResourceIds,
    'RemoveAddressBookEntry' : ResourceIds,
    'EditAddressBookEntry' : ResourceIds,
    'AddProposalPolicy' : IDL.Null,
    'ChangeCanister' : IDL.Null,
    'EditProposalPolicy' : ResourceIds,
    'EditUser' : ResourceIds,
    'Transfer' : ResourceIds,
    'EditAccount' : ResourceIds,
    'AddAddressBookEntry' : IDL.Null,
    'RemoveUserGroup' : ResourceIds,
    'AddAccount' : IDL.Null,
  });
  const UserSpecifier = IDL.Variant({
    'Id' : IDL.Vec(UUID),
    'Any' : IDL.Null,
    'Group' : IDL.Vec(UUID),
    'Proposer' : IDL.Null,
    'Owner' : IDL.Null,
  });
  const MinimumVotes = IDL.Record({
    'minimum' : IDL.Nat16,
    'voters' : UserSpecifier,
  });
  const ApprovalThreshold = IDL.Record({
    'threshold' : IDL.Nat16,
    'voters' : UserSpecifier,
  });
  ProposalPolicyCriteria.fill(
    IDL.Variant({
      'Or' : IDL.Vec(ProposalPolicyCriteria),
      'And' : IDL.Vec(ProposalPolicyCriteria),
      'Not' : ProposalPolicyCriteria,
      'HasAddressBookMetadata' : AddressBookMetadata,
      'MinimumVotes' : MinimumVotes,
      'ApprovalThreshold' : ApprovalThreshold,
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
    'module' : IDL.Vec(IDL.Nat8),
  });
  const EditProposalPolicyOperationInput = IDL.Record({
    'specifier' : IDL.Opt(ProposalSpecifier),
    'criteria' : IDL.Opt(ProposalPolicyCriteria),
    'policy_id' : UUID,
  });
  const EditUserOperationInput = IDL.Record({
    'id' : UUID,
    'status' : IDL.Opt(UserStatus),
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
  const AddAddressBookEntryOperationInput = IDL.Record({
    'metadata' : IDL.Vec(AddressBookMetadata),
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'address_owner' : IDL.Text,
    'standard' : IDL.Text,
  });
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
    'RemoveAddressBookEntry' : RemoveAddressBookEntryOperationInput,
    'EditAddressBookEntry' : EditAddressBookEntryOperationInput,
    'AddProposalPolicy' : AddProposalPolicyOperationInput,
    'ChangeCanister' : ChangeCanisterOperationInput,
    'EditProposalPolicy' : EditProposalPolicyOperationInput,
    'EditUser' : EditUserOperationInput,
    'Transfer' : TransferOperationInput,
    'EditAccount' : EditAccountOperationInput,
    'AddAddressBookEntry' : AddAddressBookEntryOperationInput,
    'RemoveUserGroup' : RemoveUserGroupOperationInput,
    'AddAccount' : AddAccountOperationInput,
  });
  const CreateProposalInput = IDL.Record({
    'title' : IDL.Opt(IDL.Text),
    'execution_plan' : IDL.Opt(ProposalExecutionSchedule),
    'summary' : IDL.Opt(IDL.Text),
    'operation' : ProposalOperationInput,
  });
  const ProposalCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_vote' : IDL.Bool,
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
  const UserGroup = IDL.Record({ 'id' : UUID, 'name' : IDL.Text });
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
  const RemoveAddressBookEntryOperation = IDL.Record({
    'input' : RemoveAddressBookEntryOperationInput,
  });
  const EditAddressBookEntryOperation = IDL.Record({
    'input' : EditAddressBookEntryOperationInput,
  });
  const AddProposalPolicyOperation = IDL.Record({
    'input' : AddProposalPolicyOperationInput,
    'policy_id' : IDL.Opt(UUID),
  });
  const Sha256Hash = IDL.Text;
  const ChangeCanisterOperation = IDL.Record({
    'module_checksum' : Sha256Hash,
    'target' : ChangeCanisterTarget,
    'arg_checksum' : IDL.Opt(Sha256Hash),
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
    'transfer_id' : IDL.Opt(UUID),
    'from_account' : IDL.Opt(Account),
    'input' : TransferOperationInput,
  });
  const EditAccountOperation = IDL.Record({
    'input' : EditAccountOperationInput,
  });
  const AddressBookEntry = IDL.Record({
    'id' : UUID,
    'metadata' : IDL.Vec(AddressBookMetadata),
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'last_modification_timestamp' : IDL.Text,
    'address_owner' : IDL.Text,
    'standard' : IDL.Text,
  });
  const AddAddressBookEntryOperation = IDL.Record({
    'address_book_entry' : IDL.Opt(AddressBookEntry),
    'input' : AddAddressBookEntryOperationInput,
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
    'RemoveAddressBookEntry' : RemoveAddressBookEntryOperation,
    'EditAddressBookEntry' : EditAddressBookEntryOperation,
    'AddProposalPolicy' : AddProposalPolicyOperation,
    'ChangeCanister' : ChangeCanisterOperation,
    'EditProposalPolicy' : EditProposalPolicyOperation,
    'EditUser' : EditUserOperation,
    'Transfer' : TransferOperation,
    'EditAccount' : EditAccountOperation,
    'AddAddressBookEntry' : AddAddressBookEntryOperation,
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
  const DisplayUser = IDL.Record({ 'id' : UUID, 'name' : IDL.Opt(IDL.Text) });
  const ProposalAdditionalInfo = IDL.Record({
    'id' : UUID,
    'voters' : IDL.Vec(DisplayUser),
    'proposer_name' : IDL.Opt(IDL.Text),
  });
  const CreateProposalResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : ProposalCallerPrivileges,
      'proposal' : Proposal,
      'additional_info' : ProposalAdditionalInfo,
    }),
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
  const GetAccessPolicyInput = IDL.Record({ 'resource' : Resource });
  const AccessPolicyCallerPrivileges = IDL.Record({
    'resource' : Resource,
    'can_edit' : IDL.Bool,
  });
  const Allow = IDL.Record({
    'user_groups' : IDL.Vec(UUID),
    'auth_scope' : AuthScope,
    'users' : IDL.Vec(UUID),
  });
  const AccessPolicy = IDL.Record({ 'resource' : Resource, 'allow' : Allow });
  const GetAccessPolicyResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : AccessPolicyCallerPrivileges,
      'policy' : AccessPolicy,
    }),
    'Err' : Error,
  });
  const GetAccountInput = IDL.Record({ 'account_id' : UUID });
  const AccountCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_transfer' : IDL.Bool,
    'can_edit' : IDL.Bool,
  });
  const GetAccountResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : AccountCallerPrivileges,
      'account' : Account,
    }),
    'Err' : Error,
  });
  const GetAddressBookEntryInput = IDL.Record({
    'address_book_entry_id' : UUID,
  });
  const AddressBookEntryCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_delete' : IDL.Bool,
    'can_edit' : IDL.Bool,
  });
  const GetAddressBookEntryResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : AddressBookEntryCallerPrivileges,
      'address_book_entry' : AddressBookEntry,
    }),
    'Err' : Error,
  });
  const ListProposalsOperationType = IDL.Variant({
    'EditAccessPolicy' : IDL.Null,
    'AddUserGroup' : IDL.Null,
    'RemoveProposalPolicy' : IDL.Null,
    'AddUser' : IDL.Null,
    'EditUserGroup' : IDL.Null,
    'RemoveAddressBookEntry' : IDL.Null,
    'EditAddressBookEntry' : IDL.Null,
    'AddProposalPolicy' : IDL.Null,
    'ChangeCanister' : IDL.Null,
    'EditProposalPolicy' : IDL.Null,
    'EditUser' : IDL.Null,
    'Transfer' : IDL.Opt(UUID),
    'EditAccount' : IDL.Null,
    'AddAddressBookEntry' : IDL.Null,
    'RemoveUserGroup' : IDL.Null,
    'AddAccount' : IDL.Null,
  });
  const GetNextVotableProposalInput = IDL.Record({
    'excluded_proposal_ids' : IDL.Vec(UUID),
    'operation_types' : IDL.Opt(IDL.Vec(ListProposalsOperationType)),
  });
  const GetProposalResultData = IDL.Record({
    'privileges' : ProposalCallerPrivileges,
    'proposal' : Proposal,
    'additional_info' : ProposalAdditionalInfo,
  });
  const GetNextVotableProposalResponse = IDL.Variant({
    'Ok' : IDL.Opt(GetProposalResultData),
    'Err' : Error,
  });
  const GetProposalInput = IDL.Record({ 'proposal_id' : UUID });
  const GetProposalResult = IDL.Variant({
    'Ok' : GetProposalResultData,
    'Err' : Error,
  });
  const GetProposalPolicyInput = IDL.Record({ 'id' : UUID });
  const ProposalPolicyCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_delete' : IDL.Bool,
    'can_edit' : IDL.Bool,
  });
  const ProposalPolicy = IDL.Record({
    'id' : UUID,
    'specifier' : ProposalSpecifier,
    'criteria' : ProposalPolicyCriteria,
  });
  const GetProposalPolicyResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : ProposalPolicyCallerPrivileges,
      'policy' : ProposalPolicy,
    }),
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
    'proposal_id' : UUID,
    'amount' : IDL.Nat,
  });
  const GetTransfersResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfers' : IDL.Vec(Transfer) }),
    'Err' : Error,
  });
  const GetUserInput = IDL.Record({ 'user_id' : UUID });
  const UserCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_edit' : IDL.Bool,
  });
  const GetUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'privileges' : UserCallerPrivileges, 'user' : User }),
    'Err' : Error,
  });
  const GetUserGroupInput = IDL.Record({ 'user_group_id' : UUID });
  const UserGroupCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_delete' : IDL.Bool,
    'can_edit' : IDL.Bool,
  });
  const GetUserGroupResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : UserGroupCallerPrivileges,
      'user_group' : UserGroup,
    }),
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
  const ListAccessPoliciesInput = IDL.Record({
    'resources' : IDL.Opt(IDL.Vec(Resource)),
    'paginate' : IDL.Opt(PaginationInput),
  });
  const BasicUser = IDL.Record({
    'id' : UUID,
    'status' : UserStatus,
    'name' : IDL.Text,
  });
  const ListAccessPoliciesResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(AccessPolicyCallerPrivileges),
      'user_groups' : IDL.Vec(UserGroup),
      'users' : IDL.Vec(BasicUser),
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
    'proposal_id' : UUID,
    'amount' : IDL.Nat,
  });
  const ListAccountTransfersResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfers' : IDL.Vec(TransferListItem) }),
    'Err' : Error,
  });
  const ListAccountsInput = IDL.Record({
    'paginate' : IDL.Opt(PaginationInput),
    'search_term' : IDL.Opt(IDL.Text),
  });
  const ListAccountsResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(AccountCallerPrivileges),
      'accounts' : IDL.Vec(Account),
      'next_offset' : IDL.Opt(IDL.Nat64),
    }),
    'Err' : Error,
  });
  const ListAddressBookEntriesInput = IDL.Record({
    'ids' : IDL.Opt(IDL.Vec(UUID)),
    'addresses' : IDL.Opt(IDL.Vec(IDL.Text)),
    'paginate' : IDL.Opt(PaginationInput),
    'address_chain' : IDL.Opt(
      IDL.Record({ 'blockchain' : IDL.Text, 'standard' : IDL.Text })
    ),
  });
  const ListAddressBookEntriesResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(AddressBookEntryCallerPrivileges),
      'address_book_entries' : IDL.Vec(AddressBookEntry),
      'next_offset' : IDL.Opt(IDL.Nat64),
    }),
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
    'RemoveAddressBookEntry' : IDL.Null,
    'EditAddressBookEntry' : IDL.Null,
    'AddProposalPolicy' : IDL.Null,
    'ChangeCanister' : IDL.Null,
    'EditProposalPolicy' : IDL.Null,
    'EditUser' : IDL.Null,
    'Transfer' : IDL.Null,
    'EditAccount' : IDL.Null,
    'AddAddressBookEntry' : IDL.Null,
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
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(ProposalPolicyCallerPrivileges),
      'next_offset' : IDL.Opt(IDL.Nat64),
      'policies' : IDL.Vec(ProposalPolicy),
    }),
    'Err' : Error,
  });
  const SortByDirection = IDL.Variant({ 'Asc' : IDL.Null, 'Desc' : IDL.Null });
  const ListProposalsSortBy = IDL.Variant({
    'ExpirationDt' : SortByDirection,
    'LastModificationDt' : SortByDirection,
    'CreatedAt' : SortByDirection,
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
  const ListProposalsInput = IDL.Record({
    'sort_by' : IDL.Opt(ListProposalsSortBy),
    'voter_ids' : IDL.Opt(IDL.Vec(UUID)),
    'expiration_from_dt' : IDL.Opt(TimestampRFC3339),
    'created_to_dt' : IDL.Opt(TimestampRFC3339),
    'statuses' : IDL.Opt(IDL.Vec(ProposalStatusCode)),
    'only_votable' : IDL.Bool,
    'proposer_ids' : IDL.Opt(IDL.Vec(UUID)),
    'expiration_to_dt' : IDL.Opt(TimestampRFC3339),
    'paginate' : IDL.Opt(PaginationInput),
    'operation_types' : IDL.Opt(IDL.Vec(ListProposalsOperationType)),
    'created_from_dt' : IDL.Opt(TimestampRFC3339),
  });
  const ListProposalsResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(ProposalCallerPrivileges),
      'proposals' : IDL.Vec(Proposal),
      'next_offset' : IDL.Opt(IDL.Nat64),
      'additional_info' : IDL.Vec(ProposalAdditionalInfo),
    }),
    'Err' : Error,
  });
  const ListUserGroupsInput = IDL.Record({
    'paginate' : IDL.Opt(PaginationInput),
    'search_term' : IDL.Opt(IDL.Text),
  });
  const ListUserGroupsResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(UserGroupCallerPrivileges),
      'user_groups' : IDL.Vec(UserGroup),
      'next_offset' : IDL.Opt(IDL.Nat64),
    }),
    'Err' : Error,
  });
  const ListUsersInput = IDL.Record({
    'statuses' : IDL.Opt(IDL.Vec(UserStatus)),
    'paginate' : IDL.Opt(PaginationInput),
    'search_term' : IDL.Opt(IDL.Text),
  });
  const ListUsersResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(UserCallerPrivileges),
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
    'ListProposals' : IDL.Null,
    'ListUserGroups' : IDL.Null,
    'AddUser' : IDL.Null,
    'ListUsers' : IDL.Null,
    'AddProposalPolicy' : IDL.Null,
    'ChangeCanister' : IDL.Null,
    'ListProposalPolicies' : IDL.Null,
    'AddAddressBookEntry' : IDL.Null,
    'ListAccounts' : IDL.Null,
    'ListAccessPolicies' : IDL.Null,
    'ListAddressBookEntries' : IDL.Null,
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
    'Ok' : IDL.Record({
      'privileges' : ProposalCallerPrivileges,
      'proposal' : Proposal,
      'additional_info' : ProposalAdditionalInfo,
    }),
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
    'get_address_book_entry' : IDL.Func(
        [GetAddressBookEntryInput],
        [GetAddressBookEntryResult],
        ['query'],
      ),
    'get_next_votable_proposal' : IDL.Func(
        [GetNextVotableProposalInput],
        [GetNextVotableProposalResponse],
        ['query'],
      ),
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
    'list_accounts' : IDL.Func(
        [ListAccountsInput],
        [ListAccountsResult],
        ['query'],
      ),
    'list_address_book_entries' : IDL.Func(
        [ListAddressBookEntriesInput],
        [ListAddressBookEntriesResult],
        ['query'],
      ),
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
    'list_user_groups' : IDL.Func(
        [ListUserGroupsInput],
        [ListUserGroupsResult],
        ['query'],
      ),
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
