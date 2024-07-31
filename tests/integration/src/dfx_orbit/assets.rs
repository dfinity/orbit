use dfx_orbit::StationAgent;
use pocket_ic::PocketIc;
use rand::{thread_rng, Rng};
use station_api::{
    AddRequestPolicyOperationInput, CallExternalCanisterOperationInput,
    CallExternalCanisterResourceTargetDTO, CanisterMethodDTO, CreateRequestInput,
    ExecutionMethodResourceTargetDTO, RequestOperationInput, RequestPolicyRuleDTO,
    RequestSpecifierDTO, ValidationMethodResourceTargetDTO,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tempfile::Builder;

use crate::{
    dfx_orbit::{
        canister_call::permit_call_operation, dfx_orbit_test, fetch_asset, setup_dfx_orbit,
        setup_dfx_user,
    },
    setup::{create_canister, get_canister_wasm, setup_new_env, WALLET_ADMIN_USER},
    utils::execute_request,
    CanisterIds, TestEnv,
};

#[test]
fn assets_update() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let (dfx_principal, _dfx_user) = setup_dfx_user(&env, &canister_ids);

    // Install the assets canister under orbit control
    let asset_canister = create_canister(&mut env, canister_ids.station);
    let asset_canister_wasm = get_canister_wasm("assetstorage");
    env.install_canister(
        asset_canister,
        asset_canister_wasm,
        candid::encode_args(()).unwrap(),
        Some(canister_ids.station),
    );

    // As admin: Setup the prepare permission in the asset canister for the dfx user
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        station_api::RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id: asset_canister,
                    method_name: String::from("grant_permission"),
                },
                arg: Some(StationAgent::request_prepare_permission_payload(dfx_principal).unwrap()),
                execution_method_cycles: None,
            },
        ),
    )
    .unwrap();

    // As admin: Grant the user the call permission, set auto-approval for external calls
    permit_call_operation(&env, &canister_ids);
    set_auto_approve(&env, &canister_ids);

    // Setup a tmpdir, and store two assets in it
    // We generate the assets dyniamically, since we want to make sure we are not
    // fetching old assets
    // NOTE: Currently, the local asset computation skips hidden files while the
    // remote version does not. This creates an issue if we just used tempdir(), as that
    // uses `.` prefix.
    let asset_dir = Builder::new().prefix("asset").tempdir().unwrap();
    let asset_a = format!(
        "This is the current time: {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let asset_b = format!("This is a random number: {}", thread_rng().gen::<u64>());

    std::fs::create_dir_all(asset_dir.path().join("subdir")).unwrap();
    std::fs::write(asset_dir.path().join("system_time"), &asset_a).unwrap();
    std::fs::write(
        asset_dir.path().join("subdir").join("random_number"),
        &asset_b,
    )
    .unwrap();

    dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // As dfx user: Request to upload new files to the asset canister
        let upload_request = dfx_orbit
            .upload_assets(
                asset_canister.to_string(),
                vec![asset_dir.path().to_str().unwrap().to_string()],
            )
            .await
            .unwrap();

        //  As dfx user: Request commitment of the batch
        let _result = dfx_orbit
            .station
            .request(CreateRequestInput {
                operation: station_api::RequestOperationInput::CallExternalCanister(
                    CallExternalCanisterOperationInput {
                        validation_method: None,
                        execution_method: CanisterMethodDTO {
                            canister_id: asset_canister,
                            method_name: String::from("commit_proposed_batch"),
                        },
                        arg: Some(
                            StationAgent::commit_proposed_batch_payload(upload_request).unwrap(),
                        ),
                        execution_method_cycles: None,
                    },
                ),
                title: None,
                summary: None,
                execution_plan: None,
            })
            .await
            .unwrap();

        // NOTE: We need to wait until the certified state becomes available.
        // Since we are in live mode, we can not simply advance pocketIC by some
        // ticks, but actially need to wait.
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Check that the new files are being served by the asset canister
        let req = fetch_asset(asset_canister, "/system_time").await;
        let test_asset_a = String::from_utf8_lossy(&req);
        assert_eq!(asset_a, test_asset_a);

        let req = fetch_asset(asset_canister, "/subdir/random_number").await;
        let test_asset_b = String::from_utf8_lossy(&req);
        assert_eq!(asset_b, test_asset_b);
    });
}

/// Set four eyes principle for canister calls
pub(crate) fn set_auto_approve(env: &PocketIc, canister_ids: &CanisterIds) {
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::CallExternalCanister(
                CallExternalCanisterResourceTargetDTO {
                    validation_method: ValidationMethodResourceTargetDTO::No,
                    execution_method: ExecutionMethodResourceTargetDTO::Any,
                },
            ),
            rule: RequestPolicyRuleDTO::AutoApproved,
        });
    execute_request(
        env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_request_policy,
    )
    .unwrap();
}
