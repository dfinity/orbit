use super::{
    setup::{dfx_orbit_test, setup_dfx_user, DfxOrbitTestConfig},
    util::{permit_call_operation, set_auto_approve_on_call, set_four_eyes_on_call},
};
use crate::{
    dfx_orbit::{setup::setup_dfx_orbit, util::fetch_asset},
    setup::{create_canister, get_canister_wasm, setup_new_env, WALLET_ADMIN_USER},
    utils::{
        add_external_canister_call_any_method_permission_and_approval, add_user, execute_request,
        user_test_id, ADMIN_GROUP_ID,
    },
    CanisterIds, TestEnv,
};
use candid::{Nat, Principal};
use dfx_orbit::{
    args::{RequestArgs, RequestArgsActions, VerifyArgs, VerifyArgsAction},
    asset::{
        RequestAssetActionArgs, RequestAssetArgs, VerifyAssetActionArgs, VerifyAssetArgs,
        {RequestAssetUploadArgs, VerifyAssetUploadArgs},
    },
};
use ic_certified_assets::types::{GrantPermissionArguments, Permission};
use pocket_ic::PocketIc;
use rand::{thread_rng, Rng};
use station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, GetRequestInput, RequestOperationInput,
};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::Builder;

const ASSET_CANISTER_NAME: &str = "test_asset_upload";

#[test]
fn asset_upload() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // add the permissions for admins to call any external canister
    add_external_canister_call_any_method_permission_and_approval(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        station_api::QuorumDTO {
            approvers: station_api::UserSpecifierDTO::Any,
            min_approved: 1,
        },
    );

    let asset_canister = setup_asset_canister(&mut env, &canister_ids);

    let (dfx_principal, _dfx_user) = setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    // As admin: Grant the user the call and prepare permissions
    permit_call_operation(&env, &canister_ids);
    set_auto_approve_on_call(&env, &canister_ids);
    grant_prepare_permission(&env, &canister_ids, asset_canister, dfx_principal);

    let (asset_dir, assets) = setup_assets();

    let mut asset_canisters = BTreeMap::new();
    asset_canisters.insert(
        ASSET_CANISTER_NAME.into(),
        vec![asset_dir.to_str().unwrap().to_string()],
    );
    let config = DfxOrbitTestConfig {
        asset_canisters,
        canister_ids: vec![(ASSET_CANISTER_NAME.into(), asset_canister)],
    };

    dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        let request_args = RequestAssetUploadArgs {
            canister: ASSET_CANISTER_NAME.into(),
            ignore_evidence: false,
            files: vec![],
        };
        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Asset(RequestAssetArgs {
                action: RequestAssetActionArgs::Upload(request_args),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();
        let _ = dfx_orbit.station.request(request.clone()).await.unwrap();

        // NOTE: We need to wait until the certified state becomes available.
        // Since we are in live mode, we can not simply advance pocketIC by some
        // ticks, but actially need to wait.
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Test that we can fetch the assets
        for (asset_path, expected_asset) in assets {
            let req = fetch_asset(asset_canister, &asset_path).await;
            let test_asset_a = String::from_utf8_lossy(&req);
            assert_eq!(expected_asset, test_asset_a);
        }
    });
}

#[test]
fn asset_validation() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    add_external_canister_call_any_method_permission_and_approval(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        station_api::QuorumDTO {
            approvers: station_api::UserSpecifierDTO::Group(vec![ADMIN_GROUP_ID
                .hyphenated()
                .to_string()]),
            min_approved: 2,
        },
    );

    let asset_canister = setup_asset_canister(&mut env, &canister_ids);

    let (dfx_principal, _dfx_user) = setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    // As admin: Grant the user the call and prepare permissions
    permit_call_operation(&env, &canister_ids);
    set_four_eyes_on_call(&env, &canister_ids);
    grant_prepare_permission(&env, &canister_ids, asset_canister, dfx_principal);

    let (asset_dir, _) = setup_assets();

    let mut asset_canisters = BTreeMap::new();
    asset_canisters.insert(
        ASSET_CANISTER_NAME.into(),
        vec![asset_dir.to_str().unwrap().to_string()],
    );
    let config = DfxOrbitTestConfig {
        asset_canisters,
        canister_ids: vec![(ASSET_CANISTER_NAME.into(), asset_canister)],
    };

    dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        let request_args = RequestAssetUploadArgs {
            canister: ASSET_CANISTER_NAME.into(),
            ignore_evidence: false,
            files: vec![],
        };

        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Asset(RequestAssetArgs {
                action: RequestAssetActionArgs::Upload(request_args),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();
        let request = dfx_orbit.station.request(request.clone()).await.unwrap();

        // Check that the request verifies
        let verify_args = VerifyAssetUploadArgs {
            canister: ASSET_CANISTER_NAME.into(),
            batch_id: Nat::from(1u64),
            files: vec![],
        };
        let req_response = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: request.request.id.clone(),
                with_full_info: Some(false),
            })
            .await
            .unwrap();
        VerifyArgs {
            request_id: request.request.id,
            and_approve: false,
            or_reject: false,
            action: VerifyArgsAction::Asset(VerifyAssetArgs {
                action: VerifyAssetActionArgs::Upload(verify_args),
            }),
        }
        .verify(&dfx_orbit, &req_response)
        .await
        .unwrap();
    });
}

/// Setup a tmpdir, and store assets in it
///
/// We generate the assets dynamically, since we want to make sure we are not
/// fetching old assets
/// NOTE: Currently, the local asset computation skips hidden files while the
/// remote version does not. This creates an issue if we just used tempdir(), as that
/// uses `.` prefix.
fn setup_assets() -> (PathBuf, BTreeMap<String, String>) {
    let asset_dir = Builder::new().prefix("asset").tempdir().unwrap();
    let mut assets = BTreeMap::new();

    let asset_a = format!(
        "This is the current time: {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let asset_a_path = PathBuf::from("system_time");
    std::fs::write(asset_dir.path().join(&asset_a_path), &asset_a).unwrap();
    assets.insert(
        asset_a_path.into_os_string().into_string().unwrap(),
        asset_a,
    );

    let asset_b = format!("This is a random number: {}", thread_rng().gen::<u64>());
    let asset_b_path = PathBuf::from("subdir").join("random_number");
    std::fs::create_dir_all(asset_dir.path().join("subdir")).unwrap();
    std::fs::write(asset_dir.path().join(&asset_b_path), &asset_b).unwrap();
    assets.insert(
        asset_b_path.into_os_string().into_string().unwrap(),
        asset_b,
    );

    (asset_dir.into_path(), assets)
}

/// Install the assets canister under orbit control
fn setup_asset_canister(env: &mut PocketIc, canister_ids: &CanisterIds) -> Principal {
    let asset_canister = create_canister(env, canister_ids.station);
    let asset_canister_wasm = get_canister_wasm("assetstorage");
    env.install_canister(
        asset_canister,
        asset_canister_wasm,
        candid::encode_args(()).unwrap(),
        Some(canister_ids.station),
    );
    asset_canister
}

fn grant_prepare_permission(
    env: &PocketIc,
    canister_ids: &CanisterIds,
    asset_canister: Principal,
    to_principal: Principal,
) {
    let arg = GrantPermissionArguments {
        to_principal,
        permission: Permission::Prepare,
    };
    let arg = candid::encode_one(arg).unwrap();

    let request = RequestOperationInput::CallExternalCanister(CallExternalCanisterOperationInput {
        validation_method: None,
        execution_method: CanisterMethodDTO {
            canister_id: asset_canister,
            method_name: String::from("grant_permission"),
        },
        arg: Some(arg),
        execution_method_cycles: None,
    });

    execute_request(env, WALLET_ADMIN_USER, canister_ids.station, request).unwrap();
}
