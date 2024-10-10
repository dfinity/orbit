export const idlFactory = ({ IDL }) => {
  const RequestPolicyRule = IDL.Rec();
  const RequestPolicyRuleResult = IDL.Rec();
  const SystemUpgrade = IDL.Record({ 'name' : IDL.Opt(IDL.Text) });
  const SystemUpgraderInput = IDL.Variant({
    'Id' : IDL.Principal,
    'WasmModule' : IDL.Vec(IDL.Nat8),
  });
  const UUID = IDL.Text;
  const AccountMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const InitAccountInput = IDL.Record({
    'id' : IDL.Opt(UUID),
    'metadata' : IDL.Vec(AccountMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'standard' : IDL.Text,
  });
  const AdminInitInput = IDL.Record({
    'name' : IDL.Text,
    'identity' : IDL.Principal,
  });
  const SystemInit = IDL.Record({
    'name' : IDL.Text,
    'fallback_controller' : IDL.Opt(IDL.Principal),
    'upgrader' : SystemUpgraderInput,
    'accounts' : IDL.Opt(IDL.Vec(InitAccountInput)),
    'admins' : IDL.Vec(AdminInitInput),
    'quorum' : IDL.Opt(IDL.Nat16),
  });
  const SystemInstall = IDL.Variant({
    'Upgrade' : SystemUpgrade,
    'Init' : SystemInit,
  });
  const CanisterStatusInput = IDL.Record({ 'canister_id' : IDL.Principal });
  const DefiniteCanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Nat,
    'controllers' : IDL.Vec(IDL.Principal),
    'reserved_cycles_limit' : IDL.Nat,
    'memory_allocation' : IDL.Nat,
    'compute_allocation' : IDL.Nat,
  });
  const CanisterStatusResponse = IDL.Record({
    'status' : IDL.Variant({
      'stopped' : IDL.Null,
      'stopping' : IDL.Null,
      'running' : IDL.Null,
    }),
    'memory_size' : IDL.Nat,
    'cycles' : IDL.Nat,
    'settings' : DefiniteCanisterSettings,
    'query_stats' : IDL.Record({
      'response_payload_bytes_total' : IDL.Nat,
      'num_instructions_total' : IDL.Nat,
      'num_calls_total' : IDL.Nat,
      'request_payload_bytes_total' : IDL.Nat,
    }),
    'idle_cycles_burned_per_day' : IDL.Nat,
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'reserved_cycles' : IDL.Nat,
  });
  const Error = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const CanisterStatusResult = IDL.Variant({
    'Ok' : CanisterStatusResponse,
    'Err' : Error,
  });
  const AssetMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const AssetSymbol = IDL.Text;
  const Asset = IDL.Record({
    'metadata' : IDL.Vec(AssetMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'standard' : IDL.Text,
    'symbol' : AssetSymbol,
  });
  const Capabilities = IDL.Record({
    'name' : IDL.Text,
    'version' : IDL.Text,
    'supported_assets' : IDL.Vec(Asset),
  });
  const CapabilitiesResult = IDL.Variant({
    'Ok' : IDL.Record({ 'capabilities' : Capabilities }),
    'Err' : Error,
  });
  const TimestampRFC3339 = IDL.Text;
  const RequestExecutionSchedule = IDL.Variant({
    'Immediate' : IDL.Null,
    'Scheduled' : IDL.Record({ 'execution_time' : TimestampRFC3339 }),
  });
  const AddUserGroupOperationInput = IDL.Record({ 'name' : IDL.Text });
  const ResourceId = IDL.Variant({ 'Id' : UUID, 'Any' : IDL.Null });
  const RequestResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Read' : ResourceId,
  });
  const NotificationResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Update' : ResourceId,
  });
  const SystemResourceAction = IDL.Variant({
    'Upgrade' : IDL.Null,
    'ManageSystemInfo' : IDL.Null,
    'SystemInfo' : IDL.Null,
    'Capabilities' : IDL.Null,
  });
  const UserResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Read' : ResourceId,
    'Create' : IDL.Null,
    'Update' : ResourceId,
  });
  const CanisterMethod = IDL.Record({
    'canister_id' : IDL.Principal,
    'method_name' : IDL.Text,
  });
  const ExecutionMethodResourceTarget = IDL.Variant({
    'Any' : IDL.Null,
    'ExecutionMethod' : CanisterMethod,
  });
  const ValidationMethodResourceTarget = IDL.Variant({
    'No' : IDL.Null,
    'ValidationMethod' : CanisterMethod,
  });
  const CallExternalCanisterResourceTarget = IDL.Record({
    'execution_method' : ExecutionMethodResourceTarget,
    'validation_method' : ValidationMethodResourceTarget,
  });
  const ExternalCanisterId = IDL.Variant({
    'Any' : IDL.Null,
    'Canister' : IDL.Principal,
  });
  const ExternalCanisterResourceAction = IDL.Variant({
    'Call' : CallExternalCanisterResourceTarget,
    'Fund' : ExternalCanisterId,
    'List' : IDL.Null,
    'Read' : ExternalCanisterId,
    'Create' : IDL.Null,
    'Change' : ExternalCanisterId,
  });
  const AccountResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Read' : ResourceId,
    'Create' : IDL.Null,
    'Transfer' : ResourceId,
    'Update' : ResourceId,
  });
  const ResourceAction = IDL.Variant({
    'List' : IDL.Null,
    'Read' : ResourceId,
    'Delete' : ResourceId,
    'Create' : IDL.Null,
    'Update' : ResourceId,
  });
  const PermissionResourceAction = IDL.Variant({
    'Read' : IDL.Null,
    'Update' : IDL.Null,
  });
  const Resource = IDL.Variant({
    'Request' : RequestResourceAction,
    'Notification' : NotificationResourceAction,
    'System' : SystemResourceAction,
    'User' : UserResourceAction,
    'ExternalCanister' : ExternalCanisterResourceAction,
    'Account' : AccountResourceAction,
    'AddressBook' : ResourceAction,
    'UserGroup' : ResourceAction,
    'Permission' : PermissionResourceAction,
    'RequestPolicy' : ResourceAction,
  });
  const AuthScope = IDL.Variant({
    'Authenticated' : IDL.Null,
    'Public' : IDL.Null,
    'Restricted' : IDL.Null,
  });
  const EditPermissionOperationInput = IDL.Record({
    'resource' : Resource,
    'user_groups' : IDL.Opt(IDL.Vec(UUID)),
    'auth_scope' : IDL.Opt(AuthScope),
    'users' : IDL.Opt(IDL.Vec(UUID)),
  });
  const Allow = IDL.Record({
    'user_groups' : IDL.Vec(UUID),
    'auth_scope' : AuthScope,
    'users' : IDL.Vec(UUID),
  });
  const CanisterExecutionAndValidationMethodPair = IDL.Record({
    'execution_method' : IDL.Text,
    'validation_method' : ValidationMethodResourceTarget,
  });
  const ExternalCanisterCallPermission = IDL.Record({
    'execution_method' : IDL.Text,
    'allow' : Allow,
    'validation_method' : ValidationMethodResourceTarget,
  });
  const ExternalCanisterChangeCallPermissionsInput = IDL.Variant({
    'OverrideSpecifiedByExecutionMethods' : IDL.Vec(
      IDL.Record({
        'execution_method' : IDL.Text,
        'permissions' : IDL.Vec(
          IDL.Record({
            'allow' : Allow,
            'validation_method' : ValidationMethodResourceTarget,
          })
        ),
      })
    ),
    'OverrideSpecifiedByExecutionValidationMethodPairs' : IDL.Vec(
      IDL.Record({
        'allow' : IDL.Opt(Allow),
        'method_configuration' : CanisterExecutionAndValidationMethodPair,
      })
    ),
    'ReplaceAllBy' : IDL.Vec(ExternalCanisterCallPermission),
  });
  const ExternalCanisterPermissionsUpdateInput = IDL.Record({
    'calls' : IDL.Opt(ExternalCanisterChangeCallPermissionsInput),
    'read' : IDL.Opt(Allow),
    'change' : IDL.Opt(Allow),
  });
  const UserSpecifier = IDL.Variant({
    'Id' : IDL.Vec(UUID),
    'Any' : IDL.Null,
    'Group' : IDL.Vec(UUID),
  });
  const Quorum = IDL.Record({
    'min_approved' : IDL.Nat16,
    'approvers' : UserSpecifier,
  });
  const QuorumPercentage = IDL.Record({
    'min_approved' : IDL.Nat16,
    'approvers' : UserSpecifier,
  });
  const AddressBookMetadata = IDL.Record({
    'key' : IDL.Text,
    'value' : IDL.Text,
  });
  RequestPolicyRule.fill(
    IDL.Variant({
      'Not' : RequestPolicyRule,
      'Quorum' : Quorum,
      'AllowListed' : IDL.Null,
      'QuorumPercentage' : QuorumPercentage,
      'AutoApproved' : IDL.Null,
      'AllOf' : IDL.Vec(RequestPolicyRule),
      'AnyOf' : IDL.Vec(RequestPolicyRule),
      'AllowListedByMetadata' : AddressBookMetadata,
    })
  );
  const ExternalCanisterChangeRequestPolicyRuleInput = IDL.Record({
    'rule' : RequestPolicyRule,
    'policy_id' : IDL.Opt(UUID),
  });
  const ExternalCanisterCallRequestPolicyRuleInput = IDL.Record({
    'execution_method' : IDL.Text,
    'rule' : RequestPolicyRule,
    'validation_method' : ValidationMethodResourceTarget,
    'policy_id' : IDL.Opt(UUID),
  });
  const ExternalCanisterChangeCallRequestPoliciesInput = IDL.Variant({
    'RemoveByPolicyIds' : IDL.Vec(UUID),
    'OverrideSpecifiedByExecutionMethods' : IDL.Vec(
      IDL.Record({
        'execution_method' : IDL.Text,
        'policies' : IDL.Vec(
          IDL.Record({
            'rule' : RequestPolicyRule,
            'validation_method' : ValidationMethodResourceTarget,
            'policy_id' : IDL.Opt(UUID),
          })
        ),
      })
    ),
    'OverrideSpecifiedByExecutionValidationMethodPairs' : IDL.Vec(
      IDL.Record({
        'method_configuration' : CanisterExecutionAndValidationMethodPair,
        'policies' : IDL.Vec(ExternalCanisterChangeRequestPolicyRuleInput),
      })
    ),
    'ReplaceAllBy' : IDL.Vec(ExternalCanisterCallRequestPolicyRuleInput),
  });
  const ExternalCanisterRequestPoliciesUpdateInput = IDL.Record({
    'calls' : IDL.Opt(ExternalCanisterChangeCallRequestPoliciesInput),
    'change' : IDL.Opt(IDL.Vec(ExternalCanisterChangeRequestPolicyRuleInput)),
  });
  const ExternalCanisterState = IDL.Variant({
    'Active' : IDL.Null,
    'Archived' : IDL.Null,
  });
  const ConfigureExternalCanisterSettingsInput = IDL.Record({
    'permissions' : IDL.Opt(ExternalCanisterPermissionsUpdateInput),
    'name' : IDL.Opt(IDL.Text),
    'labels' : IDL.Opt(IDL.Vec(IDL.Text)),
    'description' : IDL.Opt(IDL.Text),
    'request_policies' : IDL.Opt(ExternalCanisterRequestPoliciesUpdateInput),
    'state' : IDL.Opt(ExternalCanisterState),
  });
  const DefiniteCanisterSettingsInput = IDL.Record({
    'freezing_threshold' : IDL.Opt(IDL.Nat),
    'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'reserved_cycles_limit' : IDL.Opt(IDL.Nat),
    'memory_allocation' : IDL.Opt(IDL.Nat),
    'compute_allocation' : IDL.Opt(IDL.Nat),
  });
  const ConfigureExternalCanisterOperationKind = IDL.Variant({
    'SoftDelete' : IDL.Null,
    'Settings' : ConfigureExternalCanisterSettingsInput,
    'Delete' : IDL.Null,
    'NativeSettings' : DefiniteCanisterSettingsInput,
  });
  const ConfigureExternalCanisterOperationInput = IDL.Record({
    'kind' : ConfigureExternalCanisterOperationKind,
    'canister_id' : IDL.Principal,
  });
  const WasmModuleExtraChunks = IDL.Record({
    'wasm_module_hash' : IDL.Vec(IDL.Nat8),
    'chunk_hashes_list' : IDL.Vec(IDL.Vec(IDL.Nat8)),
    'store_canister' : IDL.Principal,
  });
  const CanisterInstallMode = IDL.Variant({
    'reinstall' : IDL.Null,
    'upgrade' : IDL.Null,
    'install' : IDL.Null,
  });
  const ChangeExternalCanisterOperationInput = IDL.Record({
    'arg' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'module_extra_chunks' : IDL.Opt(WasmModuleExtraChunks),
    'mode' : CanisterInstallMode,
    'canister_id' : IDL.Principal,
    'module' : IDL.Vec(IDL.Nat8),
  });
  const UserStatus = IDL.Variant({
    'Inactive' : IDL.Null,
    'Active' : IDL.Null,
  });
  const AddUserOperationInput = IDL.Record({
    'status' : UserStatus,
    'groups' : IDL.Vec(UUID),
    'name' : IDL.Text,
    'identities' : IDL.Vec(IDL.Principal),
  });
  const EditUserGroupOperationInput = IDL.Record({
    'name' : IDL.Text,
    'user_group_id' : UUID,
  });
  const DisasterRecoveryCommittee = IDL.Record({
    'user_group_id' : UUID,
    'quorum' : IDL.Nat16,
  });
  const SetDisasterRecoveryOperationInput = IDL.Record({
    'committee' : IDL.Opt(DisasterRecoveryCommittee),
  });
  const ResourceSpecifier = IDL.Variant({
    'Any' : IDL.Null,
    'Resource' : Resource,
  });
  const ResourceIds = IDL.Variant({ 'Any' : IDL.Null, 'Ids' : IDL.Vec(UUID) });
  const RequestSpecifier = IDL.Variant({
    'AddUserGroup' : IDL.Null,
    'EditPermission' : ResourceSpecifier,
    'ChangeExternalCanister' : ExternalCanisterId,
    'AddUser' : IDL.Null,
    'EditUserGroup' : ResourceIds,
    'SetDisasterRecovery' : IDL.Null,
    'EditRequestPolicy' : ResourceIds,
    'RemoveRequestPolicy' : ResourceIds,
    'SystemUpgrade' : IDL.Null,
    'RemoveAddressBookEntry' : ResourceIds,
    'CreateExternalCanister' : IDL.Null,
    'EditAddressBookEntry' : ResourceIds,
    'FundExternalCanister' : ExternalCanisterId,
    'EditUser' : ResourceIds,
    'ManageSystemInfo' : IDL.Null,
    'Transfer' : ResourceIds,
    'EditAccount' : ResourceIds,
    'AddAddressBookEntry' : IDL.Null,
    'AddRequestPolicy' : IDL.Null,
    'RemoveUserGroup' : ResourceIds,
    'CallExternalCanister' : CallExternalCanisterResourceTarget,
    'AddAccount' : IDL.Null,
  });
  const EditRequestPolicyOperationInput = IDL.Record({
    'rule' : IDL.Opt(RequestPolicyRule),
    'specifier' : IDL.Opt(RequestSpecifier),
    'policy_id' : UUID,
  });
  const RemoveRequestPolicyOperationInput = IDL.Record({ 'policy_id' : UUID });
  const SystemUpgradeTarget = IDL.Variant({
    'UpgradeUpgrader' : IDL.Null,
    'UpgradeStation' : IDL.Null,
  });
  const SystemUpgradeOperationInput = IDL.Record({
    'arg' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'module_extra_chunks' : IDL.Opt(WasmModuleExtraChunks),
    'target' : SystemUpgradeTarget,
    'module' : IDL.Vec(IDL.Nat8),
  });
  const RemoveAddressBookEntryOperationInput = IDL.Record({
    'address_book_entry_id' : UUID,
  });
  const ExternalCanisterPermissions = IDL.Record({
    'calls' : IDL.Vec(ExternalCanisterCallPermission),
    'read' : Allow,
    'change' : Allow,
  });
  const ExternalCanisterPermissionsCreateInput = ExternalCanisterPermissions;
  const CreateExternalCanisterOperationKindAddExisting = IDL.Record({
    'canister_id' : IDL.Principal,
  });
  const CreateExternalCanisterOperationKindCreateNew = IDL.Record({
    'initial_cycles' : IDL.Opt(IDL.Nat64),
  });
  const CreateExternalCanisterOperationKind = IDL.Variant({
    'AddExisting' : CreateExternalCanisterOperationKindAddExisting,
    'CreateNew' : CreateExternalCanisterOperationKindCreateNew,
  });
  const ExternalCanisterRequestPoliciesCreateInput = IDL.Record({
    'calls' : IDL.Vec(ExternalCanisterCallRequestPolicyRuleInput),
    'change' : IDL.Vec(ExternalCanisterChangeRequestPolicyRuleInput),
  });
  const CreateExternalCanisterOperationInput = IDL.Record({
    'permissions' : ExternalCanisterPermissionsCreateInput,
    'kind' : CreateExternalCanisterOperationKind,
    'name' : IDL.Text,
    'labels' : IDL.Opt(IDL.Vec(IDL.Text)),
    'description' : IDL.Opt(IDL.Text),
    'request_policies' : ExternalCanisterRequestPoliciesCreateInput,
  });
  const ChangeAddressBookMetadata = IDL.Variant({
    'OverrideSpecifiedBy' : IDL.Vec(AddressBookMetadata),
    'RemoveKeys' : IDL.Vec(IDL.Text),
    'ReplaceAllBy' : IDL.Vec(AddressBookMetadata),
  });
  const EditAddressBookEntryOperationInput = IDL.Record({
    'labels' : IDL.Opt(IDL.Vec(IDL.Text)),
    'change_metadata' : IDL.Opt(ChangeAddressBookMetadata),
    'address_book_entry_id' : UUID,
    'address_owner' : IDL.Opt(IDL.Text),
  });
  const FundExternalCanisterSendCyclesInput = IDL.Record({
    'cycles' : IDL.Nat64,
  });
  const FundExternalCanisterOperationKind = IDL.Variant({
    'Send' : FundExternalCanisterSendCyclesInput,
  });
  const FundExternalCanisterOperationInput = IDL.Record({
    'kind' : FundExternalCanisterOperationKind,
    'canister_id' : IDL.Principal,
  });
  const EditUserOperationInput = IDL.Record({
    'id' : UUID,
    'status' : IDL.Opt(UserStatus),
    'groups' : IDL.Opt(IDL.Vec(UUID)),
    'cancel_pending_requests' : IDL.Opt(IDL.Bool),
    'name' : IDL.Opt(IDL.Text),
    'identities' : IDL.Opt(IDL.Vec(IDL.Principal)),
  });
  const CycleObtainStrategyInput = IDL.Variant({
    'Disabled' : IDL.Null,
    'MintFromNativeToken' : IDL.Record({ 'account_id' : UUID }),
  });
  const ManageSystemInfoOperationInput = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'cycle_obtain_strategy' : IDL.Opt(CycleObtainStrategyInput),
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
  const RequestPolicyRuleInput = IDL.Variant({
    'Set' : RequestPolicyRule,
    'Remove' : IDL.Null,
  });
  const EditAccountOperationInput = IDL.Record({
    'account_id' : UUID,
    'configs_request_policy' : IDL.Opt(RequestPolicyRuleInput),
    'read_permission' : IDL.Opt(Allow),
    'configs_permission' : IDL.Opt(Allow),
    'name' : IDL.Opt(IDL.Text),
    'transfer_request_policy' : IDL.Opt(RequestPolicyRuleInput),
    'transfer_permission' : IDL.Opt(Allow),
  });
  const AddAddressBookEntryOperationInput = IDL.Record({
    'metadata' : IDL.Vec(AddressBookMetadata),
    'labels' : IDL.Vec(IDL.Text),
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'address_owner' : IDL.Text,
  });
  const AddRequestPolicyOperationInput = IDL.Record({
    'rule' : RequestPolicyRule,
    'specifier' : RequestSpecifier,
  });
  const RemoveUserGroupOperationInput = IDL.Record({ 'user_group_id' : UUID });
  const CallExternalCanisterOperationInput = IDL.Record({
    'arg' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'execution_method' : CanisterMethod,
    'validation_method' : IDL.Opt(CanisterMethod),
    'execution_method_cycles' : IDL.Opt(IDL.Nat64),
  });
  const AddAccountOperationInput = IDL.Record({
    'configs_request_policy' : IDL.Opt(RequestPolicyRule),
    'read_permission' : Allow,
    'configs_permission' : Allow,
    'metadata' : IDL.Vec(AccountMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'transfer_request_policy' : IDL.Opt(RequestPolicyRule),
    'transfer_permission' : Allow,
    'standard' : IDL.Text,
  });
  const RequestOperationInput = IDL.Variant({
    'AddUserGroup' : AddUserGroupOperationInput,
    'EditPermission' : EditPermissionOperationInput,
    'ConfigureExternalCanister' : ConfigureExternalCanisterOperationInput,
    'ChangeExternalCanister' : ChangeExternalCanisterOperationInput,
    'AddUser' : AddUserOperationInput,
    'EditUserGroup' : EditUserGroupOperationInput,
    'SetDisasterRecovery' : SetDisasterRecoveryOperationInput,
    'EditRequestPolicy' : EditRequestPolicyOperationInput,
    'RemoveRequestPolicy' : RemoveRequestPolicyOperationInput,
    'SystemUpgrade' : SystemUpgradeOperationInput,
    'RemoveAddressBookEntry' : RemoveAddressBookEntryOperationInput,
    'CreateExternalCanister' : CreateExternalCanisterOperationInput,
    'EditAddressBookEntry' : EditAddressBookEntryOperationInput,
    'FundExternalCanister' : FundExternalCanisterOperationInput,
    'EditUser' : EditUserOperationInput,
    'ManageSystemInfo' : ManageSystemInfoOperationInput,
    'Transfer' : TransferOperationInput,
    'EditAccount' : EditAccountOperationInput,
    'AddAddressBookEntry' : AddAddressBookEntryOperationInput,
    'AddRequestPolicy' : AddRequestPolicyOperationInput,
    'RemoveUserGroup' : RemoveUserGroupOperationInput,
    'CallExternalCanister' : CallExternalCanisterOperationInput,
    'AddAccount' : AddAccountOperationInput,
  });
  const CreateRequestInput = IDL.Record({
    'title' : IDL.Opt(IDL.Text),
    'execution_plan' : IDL.Opt(RequestExecutionSchedule),
    'summary' : IDL.Opt(IDL.Text),
    'operation' : RequestOperationInput,
  });
  const RequestCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_approve' : IDL.Bool,
  });
  const RequestStatus = IDL.Variant({
    'Failed' : IDL.Record({ 'reason' : IDL.Opt(IDL.Text) }),
    'Approved' : IDL.Null,
    'Rejected' : IDL.Null,
    'Scheduled' : IDL.Record({ 'scheduled_at' : TimestampRFC3339 }),
    'Cancelled' : IDL.Record({ 'reason' : IDL.Opt(IDL.Text) }),
    'Processing' : IDL.Record({ 'started_at' : TimestampRFC3339 }),
    'Created' : IDL.Null,
    'Completed' : IDL.Record({ 'completed_at' : TimestampRFC3339 }),
  });
  const UserGroup = IDL.Record({ 'id' : UUID, 'name' : IDL.Text });
  const AddUserGroupOperation = IDL.Record({
    'user_group' : IDL.Opt(UserGroup),
    'input' : AddUserGroupOperationInput,
  });
  const EditPermissionOperation = IDL.Record({
    'input' : EditPermissionOperationInput,
  });
  const ConfigureExternalCanisterOperation = ConfigureExternalCanisterOperationInput;
  const Sha256Hash = IDL.Text;
  const ChangeExternalCanisterOperation = IDL.Record({
    'mode' : CanisterInstallMode,
    'canister_id' : IDL.Principal,
    'module_checksum' : Sha256Hash,
    'arg_checksum' : IDL.Opt(Sha256Hash),
  });
  const User = IDL.Record({
    'id' : UUID,
    'status' : UserStatus,
    'groups' : IDL.Vec(UserGroup),
    'name' : IDL.Text,
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
  const SetDisasterRecoveryOperation = IDL.Record({
    'committee' : IDL.Opt(DisasterRecoveryCommittee),
  });
  const EditRequestPolicyOperation = IDL.Record({
    'input' : EditRequestPolicyOperationInput,
  });
  const RemoveRequestPolicyOperation = IDL.Record({
    'input' : RemoveRequestPolicyOperationInput,
  });
  const SystemUpgradeOperation = IDL.Record({
    'module_checksum' : Sha256Hash,
    'target' : SystemUpgradeTarget,
    'arg_checksum' : IDL.Opt(Sha256Hash),
  });
  const RemoveAddressBookEntryOperation = IDL.Record({
    'input' : RemoveAddressBookEntryOperationInput,
  });
  const CreateExternalCanisterOperation = IDL.Record({
    'canister_id' : IDL.Opt(IDL.Principal),
    'input' : CreateExternalCanisterOperationInput,
  });
  const EditAddressBookEntryOperation = IDL.Record({
    'input' : EditAddressBookEntryOperationInput,
  });
  const FundExternalCanisterOperation = FundExternalCanisterOperationInput;
  const EditUserOperation = IDL.Record({ 'input' : EditUserOperationInput });
  const ManageSystemInfoOperation = IDL.Record({
    'input' : ManageSystemInfoOperationInput,
  });
  const AccountBalanceInfo = IDL.Record({
    'decimals' : IDL.Nat32,
    'balance' : IDL.Nat,
    'last_update_timestamp' : TimestampRFC3339,
  });
  const Account = IDL.Record({
    'id' : UUID,
    'configs_request_policy' : IDL.Opt(RequestPolicyRule),
    'decimals' : IDL.Nat32,
    'balance' : IDL.Opt(AccountBalanceInfo),
    'metadata' : IDL.Vec(AccountMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'transfer_request_policy' : IDL.Opt(RequestPolicyRule),
    'last_modification_timestamp' : TimestampRFC3339,
    'standard' : IDL.Text,
    'symbol' : AssetSymbol,
  });
  const TransferOperation = IDL.Record({
    'fee' : IDL.Opt(IDL.Nat),
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
    'labels' : IDL.Vec(IDL.Text),
    'blockchain' : IDL.Text,
    'address' : IDL.Text,
    'last_modification_timestamp' : IDL.Text,
    'address_owner' : IDL.Text,
  });
  const AddAddressBookEntryOperation = IDL.Record({
    'address_book_entry' : IDL.Opt(AddressBookEntry),
    'input' : AddAddressBookEntryOperationInput,
  });
  const AddRequestPolicyOperation = IDL.Record({
    'input' : AddRequestPolicyOperationInput,
    'policy_id' : IDL.Opt(UUID),
  });
  const RemoveUserGroupOperation = IDL.Record({
    'input' : RemoveUserGroupOperationInput,
  });
  const CallExternalCanisterOperation = IDL.Record({
    'arg' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'execution_method' : CanisterMethod,
    'validation_method' : IDL.Opt(CanisterMethod),
    'arg_checksum' : IDL.Opt(Sha256Hash),
    'execution_method_cycles' : IDL.Opt(IDL.Nat64),
    'arg_rendering' : IDL.Opt(IDL.Text),
    'execution_method_reply' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const AddAccountOperation = IDL.Record({
    'account' : IDL.Opt(Account),
    'input' : AddAccountOperationInput,
  });
  const RequestOperation = IDL.Variant({
    'AddUserGroup' : AddUserGroupOperation,
    'EditPermission' : EditPermissionOperation,
    'ConfigureExternalCanister' : ConfigureExternalCanisterOperation,
    'ChangeExternalCanister' : ChangeExternalCanisterOperation,
    'AddUser' : AddUserOperation,
    'EditUserGroup' : EditUserGroupOperation,
    'SetDisasterRecovery' : SetDisasterRecoveryOperation,
    'EditRequestPolicy' : EditRequestPolicyOperation,
    'RemoveRequestPolicy' : RemoveRequestPolicyOperation,
    'SystemUpgrade' : SystemUpgradeOperation,
    'RemoveAddressBookEntry' : RemoveAddressBookEntryOperation,
    'CreateExternalCanister' : CreateExternalCanisterOperation,
    'EditAddressBookEntry' : EditAddressBookEntryOperation,
    'FundExternalCanister' : FundExternalCanisterOperation,
    'EditUser' : EditUserOperation,
    'ManageSystemInfo' : ManageSystemInfoOperation,
    'Transfer' : TransferOperation,
    'EditAccount' : EditAccountOperation,
    'AddAddressBookEntry' : AddAddressBookEntryOperation,
    'AddRequestPolicy' : AddRequestPolicyOperation,
    'RemoveUserGroup' : RemoveUserGroupOperation,
    'CallExternalCanister' : CallExternalCanisterOperation,
    'AddAccount' : AddAccountOperation,
  });
  const RequestApprovalStatus = IDL.Variant({
    'Approved' : IDL.Null,
    'Rejected' : IDL.Null,
  });
  const RequestApproval = IDL.Record({
    'status' : RequestApprovalStatus,
    'approver_id' : UUID,
    'status_reason' : IDL.Opt(IDL.Text),
    'decided_at' : TimestampRFC3339,
  });
  const Request = IDL.Record({
    'id' : UUID,
    'status' : RequestStatus,
    'title' : IDL.Text,
    'execution_plan' : RequestExecutionSchedule,
    'expiration_dt' : TimestampRFC3339,
    'created_at' : TimestampRFC3339,
    'requested_by' : UUID,
    'summary' : IDL.Opt(IDL.Text),
    'operation' : RequestOperation,
    'approvals' : IDL.Vec(RequestApproval),
  });
  const EvaluationStatus = IDL.Variant({
    'Approved' : IDL.Null,
    'Rejected' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const EvaluationSummaryReason = IDL.Variant({
    'AllowList' : IDL.Null,
    'AllowListMetadata' : IDL.Null,
    'AutoApproved' : IDL.Null,
    'ApprovalQuorum' : IDL.Null,
  });
  const EvaluatedRequestPolicyRule = IDL.Variant({
    'Not' : RequestPolicyRuleResult,
    'Quorum' : IDL.Record({
      'total_possible_approvers' : IDL.Nat64,
      'min_approved' : IDL.Nat64,
      'approvers' : IDL.Vec(UUID),
    }),
    'AllowListed' : IDL.Null,
    'QuorumPercentage' : IDL.Record({
      'total_possible_approvers' : IDL.Nat64,
      'min_approved' : IDL.Nat64,
      'approvers' : IDL.Vec(UUID),
    }),
    'AutoApproved' : IDL.Null,
    'AllOf' : IDL.Vec(RequestPolicyRuleResult),
    'AnyOf' : IDL.Vec(RequestPolicyRuleResult),
    'AllowListedByMetadata' : IDL.Record({ 'metadata' : AddressBookMetadata }),
  });
  RequestPolicyRuleResult.fill(
    IDL.Record({
      'status' : EvaluationStatus,
      'evaluated_rule' : EvaluatedRequestPolicyRule,
    })
  );
  const RequestEvaluationResult = IDL.Record({
    'request_id' : UUID,
    'status' : EvaluationStatus,
    'result_reasons' : IDL.Opt(IDL.Vec(EvaluationSummaryReason)),
    'policy_results' : IDL.Vec(RequestPolicyRuleResult),
  });
  const DisplayUser = IDL.Record({ 'id' : UUID, 'name' : IDL.Text });
  const RequestAdditionalInfo = IDL.Record({
    'id' : UUID,
    'evaluation_result' : IDL.Opt(RequestEvaluationResult),
    'requester_name' : IDL.Text,
    'approvers' : IDL.Vec(DisplayUser),
  });
  const CreateRequestResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : RequestCallerPrivileges,
      'request' : Request,
      'additional_info' : RequestAdditionalInfo,
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
  const GetExternalCanisterInput = IDL.Record({
    'canister_id' : IDL.Principal,
  });
  const ExternalCanisterCallerMethodsPrivileges = IDL.Record({
    'execution_method' : IDL.Text,
    'validation_method' : ValidationMethodResourceTarget,
  });
  const ExternalCanisterCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_change' : IDL.Bool,
    'canister_id' : IDL.Principal,
    'can_call' : IDL.Vec(ExternalCanisterCallerMethodsPrivileges),
    'can_fund' : IDL.Bool,
  });
  const ExternalCanisterCallRequestPolicyRule = IDL.Record({
    'execution_method' : IDL.Text,
    'rule' : RequestPolicyRule,
    'validation_method' : ValidationMethodResourceTarget,
    'policy_id' : UUID,
  });
  const ExternalCanisterChangeRequestPolicyRule = IDL.Record({
    'rule' : RequestPolicyRule,
    'policy_id' : UUID,
  });
  const ExternalCanisterRequestPolicies = IDL.Record({
    'calls' : IDL.Vec(ExternalCanisterCallRequestPolicyRule),
    'change' : IDL.Vec(ExternalCanisterChangeRequestPolicyRule),
  });
  const ExternalCanister = IDL.Record({
    'id' : UUID,
    'permissions' : ExternalCanisterPermissions,
    'modified_at' : IDL.Opt(TimestampRFC3339),
    'name' : IDL.Text,
    'labels' : IDL.Vec(IDL.Text),
    'canister_id' : IDL.Principal,
    'description' : IDL.Opt(IDL.Text),
    'created_at' : TimestampRFC3339,
    'request_policies' : ExternalCanisterRequestPolicies,
    'state' : ExternalCanisterState,
  });
  const GetExternalCanisterResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : ExternalCanisterCallerPrivileges,
      'canister' : ExternalCanister,
    }),
    'Err' : Error,
  });
  const GetExternalCanisterFiltersInput = IDL.Record({
    'with_labels' : IDL.Opt(IDL.Bool),
    'with_name' : IDL.Opt(IDL.Record({ 'prefix' : IDL.Opt(IDL.Text) })),
  });
  const GetExternalCanisterFiltersResult = IDL.Variant({
    'Ok' : IDL.Record({
      'labels' : IDL.Opt(IDL.Vec(IDL.Text)),
      'names' : IDL.Opt(
        IDL.Vec(
          IDL.Record({ 'name' : IDL.Text, 'canister_id' : IDL.Principal })
        )
      ),
    }),
    'Err' : Error,
  });
  const ListRequestsOperationType = IDL.Variant({
    'AddUserGroup' : IDL.Null,
    'EditPermission' : IDL.Null,
    'ConfigureExternalCanister' : IDL.Opt(IDL.Principal),
    'ChangeExternalCanister' : IDL.Opt(IDL.Principal),
    'AddUser' : IDL.Null,
    'EditUserGroup' : IDL.Null,
    'SetDisasterRecovery' : IDL.Null,
    'EditRequestPolicy' : IDL.Null,
    'RemoveRequestPolicy' : IDL.Null,
    'SystemUpgrade' : IDL.Null,
    'RemoveAddressBookEntry' : IDL.Null,
    'CreateExternalCanister' : IDL.Null,
    'EditAddressBookEntry' : IDL.Null,
    'FundExternalCanister' : IDL.Opt(IDL.Principal),
    'EditUser' : IDL.Null,
    'ManageSystemInfo' : IDL.Null,
    'Transfer' : IDL.Opt(UUID),
    'EditAccount' : IDL.Null,
    'AddAddressBookEntry' : IDL.Null,
    'AddRequestPolicy' : IDL.Null,
    'RemoveUserGroup' : IDL.Null,
    'CallExternalCanister' : IDL.Opt(IDL.Principal),
    'AddAccount' : IDL.Null,
  });
  const GetNextApprovableRequestInput = IDL.Record({
    'excluded_request_ids' : IDL.Vec(UUID),
    'operation_types' : IDL.Opt(IDL.Vec(ListRequestsOperationType)),
  });
  const GetRequestResultData = IDL.Record({
    'privileges' : RequestCallerPrivileges,
    'request' : Request,
    'additional_info' : RequestAdditionalInfo,
  });
  const GetNextApprovableRequestResult = IDL.Variant({
    'Ok' : IDL.Opt(GetRequestResultData),
    'Err' : Error,
  });
  const GetPermissionInput = IDL.Record({ 'resource' : Resource });
  const Permission = IDL.Record({ 'resource' : Resource, 'allow' : Allow });
  const PermissionCallerPrivileges = IDL.Record({
    'resource' : Resource,
    'can_edit' : IDL.Bool,
  });
  const GetPermissionResult = IDL.Variant({
    'Ok' : IDL.Record({
      'permission' : Permission,
      'privileges' : PermissionCallerPrivileges,
    }),
    'Err' : Error,
  });
  const GetRequestInput = IDL.Record({
    'request_id' : UUID,
    'with_full_info' : IDL.Opt(IDL.Bool),
  });
  const GetRequestResult = IDL.Variant({
    'Ok' : GetRequestResultData,
    'Err' : Error,
  });
  const GetRequestPolicyInput = IDL.Record({ 'id' : UUID });
  const RequestPolicyCallerPrivileges = IDL.Record({
    'id' : UUID,
    'can_delete' : IDL.Bool,
    'can_edit' : IDL.Bool,
  });
  const RequestPolicy = IDL.Record({
    'id' : UUID,
    'rule' : RequestPolicyRule,
    'specifier' : RequestSpecifier,
  });
  const GetRequestPolicyResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : RequestPolicyCallerPrivileges,
      'policy' : RequestPolicy,
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
    'request_id' : UUID,
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
    'request_id' : UUID,
    'status' : TransferStatus,
    'created_at' : TimestampRFC3339,
    'transfer_id' : UUID,
    'amount' : IDL.Nat,
  });
  const ListAccountTransfersResult = IDL.Variant({
    'Ok' : IDL.Record({ 'transfers' : IDL.Vec(TransferListItem) }),
    'Err' : Error,
  });
  const PaginationInput = IDL.Record({
    'offset' : IDL.Opt(IDL.Nat64),
    'limit' : IDL.Opt(IDL.Nat16),
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
    'labels' : IDL.Opt(IDL.Vec(IDL.Text)),
    'blockchain' : IDL.Opt(IDL.Text),
    'addresses' : IDL.Opt(IDL.Vec(IDL.Text)),
    'paginate' : IDL.Opt(PaginationInput),
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
  const SortByDirection = IDL.Variant({ 'Asc' : IDL.Null, 'Desc' : IDL.Null });
  const ListExternalCanistersSortInput = IDL.Variant({
    'Name' : SortByDirection,
  });
  const ListExternalCanistersInput = IDL.Record({
    'sort_by' : IDL.Opt(ListExternalCanistersSortInput),
    'states' : IDL.Opt(IDL.Vec(ExternalCanisterState)),
    'canister_ids' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'labels' : IDL.Opt(IDL.Vec(IDL.Text)),
    'paginate' : IDL.Opt(PaginationInput),
  });
  const ListExternalCanistersResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(ExternalCanisterCallerPrivileges),
      'canisters' : IDL.Vec(ExternalCanister),
      'next_offset' : IDL.Opt(IDL.Nat64),
    }),
    'Err' : Error,
  });
  const NotificationStatus = IDL.Variant({
    'Read' : IDL.Null,
    'Sent' : IDL.Null,
  });
  const NotificationTypeInput = IDL.Variant({
    'RequestCreated' : IDL.Null,
    'SystemMessage' : IDL.Null,
  });
  const ListNotificationsInput = IDL.Record({
    'status' : IDL.Opt(NotificationStatus),
    'to_dt' : IDL.Opt(TimestampRFC3339),
    'from_dt' : IDL.Opt(TimestampRFC3339),
    'notification_type' : IDL.Opt(NotificationTypeInput),
  });
  const RequestOperationType = IDL.Variant({
    'AddUserGroup' : IDL.Null,
    'EditPermission' : IDL.Null,
    'ConfigureExternalCanister' : IDL.Null,
    'ChangeExternalCanister' : IDL.Null,
    'AddUser' : IDL.Null,
    'EditUserGroup' : IDL.Null,
    'SetDisasterRecovery' : IDL.Null,
    'EditRequestPolicy' : IDL.Null,
    'RemoveRequestPolicy' : IDL.Null,
    'SystemUpgrade' : IDL.Null,
    'RemoveAddressBookEntry' : IDL.Null,
    'CreateExternalCanister' : IDL.Null,
    'EditAddressBookEntry' : IDL.Null,
    'FundExternalCanister' : IDL.Null,
    'EditUser' : IDL.Null,
    'ManageSystemInfo' : IDL.Null,
    'Transfer' : IDL.Null,
    'EditAccount' : IDL.Null,
    'AddAddressBookEntry' : IDL.Null,
    'AddRequestPolicy' : IDL.Null,
    'RemoveUserGroup' : IDL.Null,
    'CallExternalCanister' : IDL.Null,
    'AddAccount' : IDL.Null,
  });
  const NotificationType = IDL.Variant({
    'RequestCreated' : IDL.Record({
      'account_id' : IDL.Opt(UUID),
      'request_id' : UUID,
      'operation_type' : RequestOperationType,
      'user_id' : IDL.Opt(UUID),
    }),
    'RequestRejected' : IDL.Record({
      'request_id' : UUID,
      'reasons' : IDL.Opt(IDL.Vec(EvaluationSummaryReason)),
      'operation_type' : RequestOperationType,
    }),
    'SystemMessage' : IDL.Null,
    'RequestFailed' : IDL.Record({
      'request_id' : UUID,
      'operation_type' : RequestOperationType,
      'reason' : IDL.Opt(IDL.Text),
    }),
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
  const ListPermissionsInput = IDL.Record({
    'resources' : IDL.Opt(IDL.Vec(Resource)),
    'paginate' : IDL.Opt(PaginationInput),
  });
  const BasicUser = IDL.Record({
    'id' : UUID,
    'status' : UserStatus,
    'name' : IDL.Text,
  });
  const ListPermissionsResult = IDL.Variant({
    'Ok' : IDL.Record({
      'permissions' : IDL.Vec(Permission),
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(PermissionCallerPrivileges),
      'user_groups' : IDL.Vec(UserGroup),
      'users' : IDL.Vec(BasicUser),
      'next_offset' : IDL.Opt(IDL.Nat64),
    }),
    'Err' : Error,
  });
  const ListRequestPoliciesInput = PaginationInput;
  const ListRequestPoliciesResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(RequestPolicyCallerPrivileges),
      'next_offset' : IDL.Opt(IDL.Nat64),
      'policies' : IDL.Vec(RequestPolicy),
    }),
    'Err' : Error,
  });
  const ListRequestsSortBy = IDL.Variant({
    'ExpirationDt' : SortByDirection,
    'LastModificationDt' : SortByDirection,
    'CreatedAt' : SortByDirection,
  });
  const RequestStatusCode = IDL.Variant({
    'Failed' : IDL.Null,
    'Approved' : IDL.Null,
    'Rejected' : IDL.Null,
    'Scheduled' : IDL.Null,
    'Cancelled' : IDL.Null,
    'Processing' : IDL.Null,
    'Created' : IDL.Null,
    'Completed' : IDL.Null,
  });
  const ListRequestsInput = IDL.Record({
    'sort_by' : IDL.Opt(ListRequestsSortBy),
    'with_evaluation_results' : IDL.Bool,
    'expiration_from_dt' : IDL.Opt(TimestampRFC3339),
    'created_to_dt' : IDL.Opt(TimestampRFC3339),
    'statuses' : IDL.Opt(IDL.Vec(RequestStatusCode)),
    'approver_ids' : IDL.Opt(IDL.Vec(UUID)),
    'expiration_to_dt' : IDL.Opt(TimestampRFC3339),
    'paginate' : IDL.Opt(PaginationInput),
    'requester_ids' : IDL.Opt(IDL.Vec(UUID)),
    'operation_types' : IDL.Opt(IDL.Vec(ListRequestsOperationType)),
    'only_approvable' : IDL.Bool,
    'created_from_dt' : IDL.Opt(TimestampRFC3339),
  });
  const ListRequestsResult = IDL.Variant({
    'Ok' : IDL.Record({
      'total' : IDL.Nat64,
      'privileges' : IDL.Vec(RequestCallerPrivileges),
      'requests' : IDL.Vec(Request),
      'next_offset' : IDL.Opt(IDL.Nat64),
      'additional_info' : IDL.Vec(RequestAdditionalInfo),
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
    'groups' : IDL.Opt(IDL.Vec(UUID)),
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
    'ListRequestPolicies' : IDL.Null,
    'ListPermissions' : IDL.Null,
    'ListUserGroups' : IDL.Null,
    'AddUser' : IDL.Null,
    'ListUsers' : IDL.Null,
    'SystemUpgrade' : IDL.Null,
    'CreateExternalCanister' : IDL.Null,
    'ManageSystemInfo' : IDL.Null,
    'AddAddressBookEntry' : IDL.Null,
    'ListAccounts' : IDL.Null,
    'AddRequestPolicy' : IDL.Null,
    'ListAddressBookEntries' : IDL.Null,
    'ListExternalCanisters' : IDL.Null,
    'ListRequests' : IDL.Null,
    'SystemInfo' : IDL.Null,
    'Capabilities' : IDL.Null,
    'AddAccount' : IDL.Null,
  });
  const MeResult = IDL.Variant({
    'Ok' : IDL.Record({ 'me' : User, 'privileges' : IDL.Vec(UserPrivilege) }),
    'Err' : Error,
  });
  const NotifyFailedStationUpgradeInput = IDL.Record({ 'reason' : IDL.Text });
  const NotifyFailedStationUpgradeResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : Error,
  });
  const SubmitRequestApprovalInput = IDL.Record({
    'request_id' : UUID,
    'decision' : RequestApprovalStatus,
    'reason' : IDL.Opt(IDL.Text),
  });
  const SubmitRequestApprovalResult = IDL.Variant({
    'Ok' : IDL.Record({
      'privileges' : RequestCallerPrivileges,
      'request' : Request,
      'additional_info' : RequestAdditionalInfo,
    }),
    'Err' : Error,
  });
  const DisasterRecovery = IDL.Record({
    'user_group_name' : IDL.Opt(IDL.Text),
    'committee' : DisasterRecoveryCommittee,
  });
  const CycleObtainStrategy = IDL.Variant({
    'Disabled' : IDL.Null,
    'MintFromNativeToken' : IDL.Record({
      'account_id' : UUID,
      'account_name' : IDL.Opt(IDL.Text),
    }),
  });
  const SystemInfo = IDL.Record({
    'disaster_recovery' : IDL.Opt(DisasterRecovery),
    'name' : IDL.Text,
    'last_upgrade_timestamp' : TimestampRFC3339,
    'raw_rand_successful' : IDL.Bool,
    'version' : IDL.Text,
    'cycles' : IDL.Nat64,
    'upgrader_id' : IDL.Principal,
    'cycle_obtain_strategy' : CycleObtainStrategy,
  });
  const SystemInfoResult = IDL.Variant({
    'Ok' : IDL.Record({ 'system' : SystemInfo }),
    'Err' : Error,
  });
  return IDL.Service({
    'canister_status' : IDL.Func(
        [CanisterStatusInput],
        [CanisterStatusResult],
        [],
      ),
    'capabilities' : IDL.Func([], [CapabilitiesResult], ['query']),
    'create_request' : IDL.Func(
        [CreateRequestInput],
        [CreateRequestResult],
        [],
      ),
    'fetch_account_balances' : IDL.Func(
        [FetchAccountBalancesInput],
        [FetchAccountBalancesResult],
        [],
      ),
    'get_account' : IDL.Func([GetAccountInput], [GetAccountResult], ['query']),
    'get_address_book_entry' : IDL.Func(
        [GetAddressBookEntryInput],
        [GetAddressBookEntryResult],
        ['query'],
      ),
    'get_external_canister' : IDL.Func(
        [GetExternalCanisterInput],
        [GetExternalCanisterResult],
        ['query'],
      ),
    'get_external_canister_filters' : IDL.Func(
        [GetExternalCanisterFiltersInput],
        [GetExternalCanisterFiltersResult],
        ['query'],
      ),
    'get_next_approvable_request' : IDL.Func(
        [GetNextApprovableRequestInput],
        [GetNextApprovableRequestResult],
        ['query'],
      ),
    'get_permission' : IDL.Func(
        [GetPermissionInput],
        [GetPermissionResult],
        ['query'],
      ),
    'get_request' : IDL.Func([GetRequestInput], [GetRequestResult], ['query']),
    'get_request_policy' : IDL.Func(
        [GetRequestPolicyInput],
        [GetRequestPolicyResult],
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
    'list_external_canisters' : IDL.Func(
        [ListExternalCanistersInput],
        [ListExternalCanistersResult],
        ['query'],
      ),
    'list_notifications' : IDL.Func(
        [ListNotificationsInput],
        [ListNotificationsResult],
        ['query'],
      ),
    'list_permissions' : IDL.Func(
        [ListPermissionsInput],
        [ListPermissionsResult],
        ['query'],
      ),
    'list_request_policies' : IDL.Func(
        [ListRequestPoliciesInput],
        [ListRequestPoliciesResult],
        ['query'],
      ),
    'list_requests' : IDL.Func(
        [ListRequestsInput],
        [ListRequestsResult],
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
    'notify_failed_station_upgrade' : IDL.Func(
        [NotifyFailedStationUpgradeInput],
        [NotifyFailedStationUpgradeResult],
        [],
      ),
    'submit_request_approval' : IDL.Func(
        [SubmitRequestApprovalInput],
        [SubmitRequestApprovalResult],
        [],
      ),
    'system_info' : IDL.Func([], [SystemInfoResult], ['query']),
  });
};
export const init = ({ IDL }) => {
  const SystemUpgrade = IDL.Record({ 'name' : IDL.Opt(IDL.Text) });
  const SystemUpgraderInput = IDL.Variant({
    'Id' : IDL.Principal,
    'WasmModule' : IDL.Vec(IDL.Nat8),
  });
  const UUID = IDL.Text;
  const AccountMetadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const InitAccountInput = IDL.Record({
    'id' : IDL.Opt(UUID),
    'metadata' : IDL.Vec(AccountMetadata),
    'name' : IDL.Text,
    'blockchain' : IDL.Text,
    'standard' : IDL.Text,
  });
  const AdminInitInput = IDL.Record({
    'name' : IDL.Text,
    'identity' : IDL.Principal,
  });
  const SystemInit = IDL.Record({
    'name' : IDL.Text,
    'fallback_controller' : IDL.Opt(IDL.Principal),
    'upgrader' : SystemUpgraderInput,
    'accounts' : IDL.Opt(IDL.Vec(InitAccountInput)),
    'admins' : IDL.Vec(AdminInitInput),
    'quorum' : IDL.Opt(IDL.Nat16),
  });
  const SystemInstall = IDL.Variant({
    'Upgrade' : SystemUpgrade,
    'Init' : SystemInit,
  });
  return [IDL.Opt(SystemInstall)];
};
