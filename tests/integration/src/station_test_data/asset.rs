use super::next_unique_id;
use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use orbit_essentials::api::ApiResult;
use pocket_ic::{query_candid_as, PocketIc, RejectResponse};
use station_api::{GetAssetInput, GetAssetResponse, ListAssetsInput, ListAssetsResponse};

pub fn add_asset_with_input(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    input: station_api::AddAssetOperationInput,
) -> station_api::AssetDTO {
    let add_asset_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::AddAsset(input),
    );

    let request = wait_for_request(env, requester, station_canister_id, add_asset_request)
        .expect("Failed to add asset");

    match request.operation {
        station_api::RequestOperationDTO::AddAsset(add_asset) => add_asset.asset.unwrap(),
        _ => panic!("invalid request operation"),
    }
}

pub fn add_asset(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> station_api::AssetDTO {
    let next_id = next_unique_id();

    add_asset_with_input(
        env,
        station_canister_id,
        requester,
        station_api::AddAssetOperationInput {
            name: format!("asset-{}", next_id),
            blockchain: "icp".to_string(),
            standards: vec!["icp_native".to_string()],
            metadata: Vec::new(),
            symbol: format!("SYM{}", next_id),
            decimals: 8,
        },
    )
}

pub fn edit_asset_name(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    asset_id: station_api::UuidDTO,
    name: String,
) {
    let edit_asset_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::EditAsset(station_api::EditAssetOperationInput {
            asset_id,
            name: Some(name),
            blockchain: None,
            standards: None,
            symbol: None,
            change_metadata: None,
        }),
    );

    wait_for_request(env, requester, station_canister_id, edit_asset_request)
        .expect("Failed to edit asset name");
}

pub fn remove_asset(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    asset_id: station_api::UuidDTO,
) {
    let remove_asset_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::RemoveAsset(station_api::RemoveAssetOperationInput {
            asset_id,
        }),
    );

    wait_for_request(env, requester, station_canister_id, remove_asset_request)
        .expect("Failed to remove asset");
}

pub fn list_assets(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> Result<(ApiResult<ListAssetsResponse>,), RejectResponse> {
    query_candid_as::<(ListAssetsInput,), (ApiResult<ListAssetsResponse>,)>(
        env,
        station_canister_id,
        requester,
        "list_assets",
        (ListAssetsInput { paginate: None },),
    )
}

pub fn get_asset(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    asset_id: station_api::UuidDTO,
) -> Result<(ApiResult<GetAssetResponse>,), RejectResponse> {
    query_candid_as::<(GetAssetInput,), (ApiResult<GetAssetResponse>,)>(
        env,
        station_canister_id,
        requester,
        "get_asset",
        (GetAssetInput { asset_id },),
    )
}
