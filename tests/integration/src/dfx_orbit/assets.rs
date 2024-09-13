use super::setup::{dfx_orbit_test, setup_dfx_user, DfxOrbitTestConfig};
use crate::{
    dfx_orbit::{canister_call::permit_call_operation, setup::setup_dfx_orbit, util::fetch_asset},
    setup::{create_canister, get_canister_wasm, setup_new_env, WALLET_ADMIN_USER},
    utils::execute_request,
    CanisterIds, TestEnv,
};
use dfx_orbit::DfxOrbit;
use pocket_ic::PocketIc;
use rand::{thread_rng, Rng};
use station_api::{
    AddRequestPolicyOperationInput, CallExternalCanisterResourceTargetDTO, CreateRequestInput,
    ExecutionMethodResourceTargetDTO, GetRequestInput, RequestOperationInput, RequestPolicyRuleDTO,
    RequestSpecifierDTO, ValidationMethodResourceTargetDTO,
};
use std::{
    collections::BTreeMap,
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::Builder;

#[test]
fn assets_upload() {
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

    let mut asset_canisters = BTreeMap::new();
    asset_canisters.insert(
        String::from("test_asset_upload"),
        vec![asset_dir.path().to_str().unwrap().to_string()],
    );
    let config = DfxOrbitTestConfig {
        asset_canisters,
        ..Default::default()
    };

    dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // As dfx user: Request to have Prepare permission for asset_canister
        let _response = dfx_orbit
            .station
            .request(CreateRequestInput {
                operation: DfxOrbit::grant_permission_request(asset_canister, dfx_principal)
                    .unwrap(),
                title: None,
                summary: None,
                execution_plan: None,
            })
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;

        // Test that we can retreive the sources from `dfx.json`
        let sources = dfx_orbit.as_path_bufs("test_asset_upload", &[]).unwrap();
        let sources_path = sources
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect::<Vec<&Path>>();

        // As dfx user: Request to upload new files to the asset canister
        let (batch_id, evidence) = dfx_orbit
            .upload(asset_canister, &sources_path, false)
            .await
            .unwrap();

        let response = dfx_orbit
            .station
            .request(CreateRequestInput {
                operation: DfxOrbit::commit_batch_input(
                    asset_canister,
                    batch_id.clone(),
                    evidence.clone(),
                )
                .unwrap(),
                title: None,
                summary: None,
                execution_plan: None,
            })
            .await
            .unwrap();

        // Check whether the request passes the asset check
        let response = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: response.request.id,
            })
            .await
            .unwrap();
        DfxOrbit::check_evidence(&response, asset_canister, batch_id, hex::encode(evidence))
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
