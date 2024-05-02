export const idlFactory = ({ IDL }) => {
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
  const ApiError = IDL.Record({
    'code' : IDL.Text,
    'message' : IDL.Opt(IDL.Text),
    'details' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
  });
  const CanDeployStationResult = IDL.Variant({
    'Ok' : CanDeployStationResponse,
    'Err' : ApiError,
  });
  const StationID = IDL.Principal;
  const UserStation = IDL.Record({
    'name' : IDL.Text,
    'canister_id' : StationID,
  });
  const TimestampRFC3339 = IDL.Text;
  const User = IDL.Record({
    'stations' : IDL.Vec(UserStation),
    'last_active' : TimestampRFC3339,
    'subscription_status' : UserSubscriptionStatus,
    'identity' : IDL.Principal,
    'main_station' : IDL.Opt(StationID),
  });
  const RemoveUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const DeployStationInput = IDL.Record({
    'admin_name' : IDL.Text,
    'station_name' : IDL.Text,
  });
  const DeployStationResult = IDL.Variant({
    'Ok' : IDL.Record({ 'canister_id' : StationID }),
    'Err' : ApiError,
  });
  const GetMainStationResult = IDL.Variant({
    'Ok' : IDL.Record({ 'station' : IDL.Opt(UserStation) }),
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
  const ListStationsResult = IDL.Variant({
    'Ok' : IDL.Record({ 'stations' : IDL.Vec(UserStation) }),
    'Err' : ApiError,
  });
  const ManageUserInput = IDL.Record({
    'stations' : IDL.Opt(IDL.Vec(UserStation)),
    'main_station' : IDL.Opt(StationID),
  });
  const ManageUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
    'Err' : ApiError,
  });
  const RegisterUserInput = IDL.Record({ 'station' : IDL.Opt(UserStation) });
  const RegisterUserResult = IDL.Variant({
    'Ok' : IDL.Record({ 'user' : User }),
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
    'can_deploy_station' : IDL.Func([], [CanDeployStationResult], ['query']),
    'delete_user' : IDL.Func([], [RemoveUserResult], []),
    'deploy_station' : IDL.Func(
        [DeployStationInput],
        [DeployStationResult],
        [],
      ),
    'get_main_station' : IDL.Func([], [GetMainStationResult], ['query']),
    'get_user' : IDL.Func([], [GetUserResult], ['query']),
    'get_waiting_list' : IDL.Func([], [GetWaitingListResult], []),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'list_stations' : IDL.Func([], [ListStationsResult], ['query']),
    'manage_user' : IDL.Func([ManageUserInput], [ManageUserResult], []),
    'register_user' : IDL.Func([RegisterUserInput], [RegisterUserResult], []),
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
