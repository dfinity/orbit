export const idlFactory = ({ IDL }) => {
  const WasmModuleRegistryEntryDependency = IDL.Record({
    'name' : IDL.Text,
    'version' : IDL.Text,
  });
  const WasmModuleRegistryEntryValueInput = IDL.Record({
    'wasm_module' : IDL.Vec(IDL.Nat8),
    'version' : IDL.Text,
    'dependencies' : IDL.Vec(WasmModuleRegistryEntryDependency),
  });
  const RegistryEntryValueInput = IDL.Variant({
    'WasmModule' : WasmModuleRegistryEntryValueInput,
  });
  const Metadata = IDL.Record({ 'key' : IDL.Text, 'value' : IDL.Text });
  const RegistryEntryInput = IDL.Record({
    'categories' : IDL.Vec(IDL.Text),
    'value' : RegistryEntryValueInput,
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'tags' : IDL.Vec(IDL.Text),
    'description' : IDL.Text,
  });
  const AddRegistryEntryInput = IDL.Record({ 'entry' : RegistryEntryInput });
  const UUID = IDL.Text;
  const TimestampRFC3339 = IDL.Text;
  const WasmModuleRegistryEntryValue = IDL.Record({
    'version' : IDL.Text,
    'dependencies' : IDL.Vec(WasmModuleRegistryEntryDependency),
    'wasm_artifact_id' : UUID,
  });
  const RegistryEntryValue = IDL.Variant({
    'WasmModule' : WasmModuleRegistryEntryValue,
  });
  const RegistryEntry = IDL.Record({
    'id' : UUID,
    'categories' : IDL.Vec(IDL.Text),
    'updated_at' : IDL.Opt(TimestampRFC3339),
    'value' : RegistryEntryValue,
    'metadata' : IDL.Vec(Metadata),
    'name' : IDL.Text,
    'tags' : IDL.Vec(IDL.Text),
    'description' : IDL.Text,
    'created_at' : TimestampRFC3339,
  });
  const AddRegistryEntryResponse = IDL.Record({ 'entry' : RegistryEntry });
  const ApiError = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const AddRegistryEntryResult = IDL.Variant({
    'Ok' : AddRegistryEntryResponse,
    'Err' : ApiError,
  });
  const UserSubscriptionStatus = IDL.Variant({
    'Unsubscribed' : IDL.Null,
    'Approved' : IDL.Null,
    'Denylisted' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const CanDeployStationResponse = IDL.Variant({
    'NotAllowed' : UserSubscriptionStatus,
    'Allowed' : IDL.Nat64,
    'QuotaExceeded' : IDL.Null,
  });
  const CanDeployStationResult = IDL.Variant({
    'Ok' : CanDeployStationResponse,
    'Err' : ApiError,
  });
  const DeleteRegistryEntryInput = IDL.Record({ 'id' : UUID });
  const DeleteRegistryEntryResponse = IDL.Record({ 'entry' : RegistryEntry });
  const DeleteRegistryEntryResult = IDL.Variant({
    'Ok' : DeleteRegistryEntryResponse,
    'Err' : ApiError,
  });
  const User = IDL.Record({
    'last_active' : TimestampRFC3339,
    'subscription_status' : UserSubscriptionStatus,
    'identity' : IDL.Principal,
  });
  const RemoveUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const DeployStationAdminUserInput = IDL.Record({
    'username' : IDL.Text,
    'identity' : IDL.Principal,
  });
  const DeployStationInput = IDL.Record({
    'name' : IDL.Text,
    'admins' : IDL.Vec(DeployStationAdminUserInput),
    'associate_with_caller' : IDL.Opt(
      IDL.Record({ 'labels' : IDL.Vec(IDL.Text) })
    ),
  });
  const StationID = IDL.Principal;
  const DeployStationResult = IDL.Variant({
    'Ok' : IDL.Record({ 'canister_id' : StationID }),
    'Err' : ApiError,
  });
  const RegistryEntryUpdateInput = IDL.Record({
    'categories' : IDL.Opt(IDL.Vec(IDL.Text)),
    'value' : IDL.Opt(RegistryEntryValueInput),
    'metadata' : IDL.Opt(IDL.Vec(Metadata)),
    'tags' : IDL.Opt(IDL.Vec(IDL.Text)),
    'description' : IDL.Opt(IDL.Text),
  });
  const EditRegistryEntryInput = IDL.Record({
    'id' : UUID,
    'entry' : RegistryEntryUpdateInput,
  });
  const EditRegistryEntryResponse = IDL.Record({ 'entry' : RegistryEntry });
  const EditRegistryEntryResult = IDL.Variant({
    'Ok' : EditRegistryEntryResponse,
    'Err' : ApiError,
  });
  const GetArtifactInput = IDL.Record({ 'artifact_id' : UUID });
  const Sha256Hex = IDL.Text;
  const Artifact = IDL.Record({
    'id' : UUID,
    'hash' : Sha256Hex,
    'artifact' : IDL.Vec(IDL.Nat8),
    'size' : IDL.Nat64,
    'created_at' : TimestampRFC3339,
  });
  const GetArtifactResponse = IDL.Record({ 'artifact' : Artifact });
  const GetArtifactResult = IDL.Variant({
    'Ok' : GetArtifactResponse,
    'Err' : ApiError,
  });
  const GetRegistryEntryInput = IDL.Record({ 'id' : UUID });
  const GetRegistryEntryResponse = IDL.Record({ 'entry' : RegistryEntry });
  const GetRegistryEntryResult = IDL.Variant({
    'Ok' : GetRegistryEntryResponse,
    'Err' : ApiError,
  });
  const GetUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const SubscribedUser = IDL.Record({
    'user_principal' : IDL.Principal,
    'email' : IDL.Text,
  });
  const GetWaitingListResponse = IDL.Record({
    'subscribed_users' : IDL.Vec(SubscribedUser),
  });
  const GetWaitingListResult = IDL.Variant({
    'Ok' : GetWaitingListResponse,
    'Err' : ApiError,
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
  const ListUserStationsInput = IDL.Record({
    'filter_by_labels' : IDL.Opt(IDL.Vec(IDL.Text)),
  });
  const UserStation = IDL.Record({
    'name' : IDL.Text,
    'labels' : IDL.Vec(IDL.Text),
    'canister_id' : StationID,
  });
  const ListUserStationsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'stations' : IDL.Vec(UserStation) }),
    'Err' : ApiError,
  });
  const ManageUserStationsInput = IDL.Variant({
    'Add' : IDL.Vec(UserStation),
    'Remove' : IDL.Vec(StationID),
    'Update' : IDL.Vec(
      IDL.Record({ 'station' : UserStation, 'index' : IDL.Opt(IDL.Nat64) })
    ),
  });
  const ManageUserStationsResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : ApiError,
  });
  const NextModuleVersionInput = IDL.Record({
    'name' : IDL.Text,
    'current_version' : IDL.Text,
  });
  const NextModuleVersionResponse = IDL.Record({
    'entry' : IDL.Opt(RegistryEntry),
  });
  const NextModuleVersionResult = IDL.Variant({
    'Ok' : NextModuleVersionResponse,
    'Err' : ApiError,
  });
  const RegisterUserInput = IDL.Record({ 'station' : IDL.Opt(UserStation) });
  const RegisterUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const SortDirection = IDL.Variant({ 'Asc' : IDL.Null, 'Desc' : IDL.Null });
  const RegistryEntrySortBy = IDL.Variant({
    'Version' : SortDirection,
    'CreatedAt' : SortDirection,
  });
  const PaginationInput = IDL.Record({
    'offset' : IDL.Opt(IDL.Nat64),
    'limit' : IDL.Opt(IDL.Nat16),
  });
  const RegistryEntryValueKind = IDL.Variant({ 'WasmModule' : IDL.Null });
  const SearchRegistryFilterKind = IDL.Variant({
    'Kind' : RegistryEntryValueKind,
    'Name' : IDL.Text,
    'Namespace' : IDL.Text,
  });
  const SearchRegistryInput = IDL.Record({
    'sort_by' : IDL.Opt(RegistryEntrySortBy),
    'pagination' : IDL.Opt(PaginationInput),
    'filter_by' : IDL.Vec(SearchRegistryFilterKind),
  });
  const SearchRegistryResponse = IDL.Record({
    'total' : IDL.Nat64,
    'entries' : IDL.Vec(RegistryEntry),
    'next_offset' : IDL.Opt(IDL.Nat64),
  });
  const SearchRegistryResult = IDL.Variant({
    'Ok' : SearchRegistryResponse,
    'Err' : ApiError,
  });
  const SetUserActiveResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : ApiError,
  });
  const SubscribeToWaitingListResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : ApiError,
  });
  const UpdateWaitingListInput = IDL.Record({
    'users' : IDL.Vec(IDL.Principal),
    'new_status' : UserSubscriptionStatus,
  });
  const UpdateWaitingListResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : ApiError,
  });
  const UploadCanisterModulesInput = IDL.Record({
    'station_wasm_module' : IDL.Vec(IDL.Nat8),
    'upgrader_wasm_module' : IDL.Vec(IDL.Nat8),
  });
  const UploadUploadCanisterModulesInputResult = IDL.Variant({
    'Ok' : IDL.Null,
    'Err' : ApiError,
  });
  return IDL.Service({
    'add_registry_entry' : IDL.Func(
        [AddRegistryEntryInput],
        [AddRegistryEntryResult],
        [],
      ),
    'can_deploy_station' : IDL.Func([], [CanDeployStationResult], ['query']),
    'delete_registry_entry' : IDL.Func(
        [DeleteRegistryEntryInput],
        [DeleteRegistryEntryResult],
        [],
      ),
    'delete_user' : IDL.Func([], [RemoveUserResult], []),
    'deploy_station' : IDL.Func(
        [DeployStationInput],
        [DeployStationResult],
        [],
      ),
    'edit_registry_entry' : IDL.Func(
        [EditRegistryEntryInput],
        [EditRegistryEntryResult],
        [],
      ),
    'get_artifact' : IDL.Func(
        [GetArtifactInput],
        [GetArtifactResult],
        ['query'],
      ),
    'get_registry_entry' : IDL.Func(
        [GetRegistryEntryInput],
        [GetRegistryEntryResult],
        ['query'],
      ),
    'get_user' : IDL.Func([], [GetUserResult], ['query']),
    'get_waiting_list' : IDL.Func([], [GetWaitingListResult], []),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'list_user_stations' : IDL.Func(
        [ListUserStationsInput],
        [ListUserStationsResult],
        ['query'],
      ),
    'manage_user_stations' : IDL.Func(
        [ManageUserStationsInput],
        [ManageUserStationsResult],
        [],
      ),
    'next_module_version' : IDL.Func(
        [NextModuleVersionInput],
        [NextModuleVersionResult],
        ['query'],
      ),
    'register_user' : IDL.Func([RegisterUserInput], [RegisterUserResult], []),
    'search_registry' : IDL.Func(
        [SearchRegistryInput],
        [SearchRegistryResult],
        ['query'],
      ),
    'set_user_active' : IDL.Func([], [SetUserActiveResult], []),
    'subscribe_to_waiting_list' : IDL.Func(
        [IDL.Text],
        [SubscribeToWaitingListResult],
        [],
      ),
    'update_waiting_list' : IDL.Func(
        [UpdateWaitingListInput],
        [UpdateWaitingListResult],
        [],
      ),
    'upload_canister_modules' : IDL.Func(
        [UploadCanisterModulesInput],
        [UploadUploadCanisterModulesInputResult],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
