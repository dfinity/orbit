use dfx_orbit::{args::canister::UploadAssetsArgs, StationAgent};
use pocket_ic::PocketIc;
use rand::{thread_rng, Rng};
use station_api::{
    AddRequestPolicyOperationInput, CallExternalCanisterOperationInput,
    CallExternalCanisterResourceTargetDTO, CanisterMethodDTO, CreateRequestInput,
    ExecutionMethodResourceTargetDTO, RequestOperationInput, RequestPolicyRuleDTO,
    RequestSpecifierDTO, ValidationMethodResourceTargetDTO,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

use crate::{
    cli::{canister_call::permit_call_operation, dfx_orbit_test, setup_agent, setup_dfx_user},
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
    let asset_dir = tempdir().unwrap();
    let asset_a = format!(
        "This is the current time: {}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let asset_b = format!("This is a random number: {}", thread_rng().gen::<u64>());

    std::fs::create_dir_all(asset_dir.path().join("subdir")).unwrap();
    std::fs::write(asset_dir.path().join("system_time"), asset_a).unwrap();
    std::fs::write(
        asset_dir.path().join("subdir").join("random_number"),
        asset_b,
    )
    .unwrap();

    let _response = dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut station_agent = setup_agent(canister_ids.station).await;

        // As dfx user: Request to upload new files to the asset canister
        let upload_request = station_agent
            .upload_assets(UploadAssetsArgs {
                canister: asset_canister.to_string(),
                source: vec![asset_dir.path().to_str().unwrap().to_string()],
                verbose: false,
            })
            .await
            .unwrap();

        // TODO: As dfx user: Request commitment of the batch
        let result = station_agent
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

        dbg!(&result);
        // TODO: As anon: Check that the new files are being served by the asset canister
    });

    todo!()
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
