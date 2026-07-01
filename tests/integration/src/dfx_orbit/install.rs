use crate::{
    dfx_orbit::{
        setup::{dfx_orbit_test, setup_dfx_orbit, setup_dfx_user, DfxOrbitTestConfig},
        util::{permit_change_operation, set_four_eyes_on_change},
    },
    setup::{create_canister, get_canister_wasm, setup_new_env},
    utils::{add_user, canister_status, submit_request_approval, user_test_id, wait_for_request},
    TestEnv,
};
use candid::Encode;
use dfx_orbit::canister::{
    CanisterInstallModeArgs, RequestCanisterInstallArgs, WasmMemoryPersistenceArgs,
};
use dfx_orbit::{
    args::{RequestArgs, RequestArgsActions, VerifyArgs, VerifyArgsAction},
    canister::{
        RequestCanisterActionArgs, RequestCanisterArgs, VerifyCanisterActionArgs,
        VerifyCanisterArgs,
    },
};
use sha2::{Digest, Sha256};
use station_api::{
    CanisterInstallMode, GetRequestInput, RequestApprovalStatusDTO, RequestOperationDTO,
    WasmMemoryPersistence,
};
use std::io::Write;
use tempfile::NamedTempFile;

fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Test installing a canister through orbit using the station agent
#[test]
fn canister_install_no_chunks() {
    canister_install(false);
}

/// Test installing a canister using chunks through orbit using the station agent
#[test]
fn canister_install_chunks() {
    canister_install(true);
}

fn canister_install(use_chunks: bool) {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let (dfx_user, _) = setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    // create the test canister
    let test_canister = create_canister(&env, canister_ids.station);

    // create and install the asset canister to hold module chunks
    let asset_canister = if use_chunks {
        let asset_canister_id = create_canister(&env, dfx_user);
        env.install_canister(
            asset_canister_id,
            get_canister_wasm("assetstorage"),
            Encode!(&()).unwrap(),
            Some(dfx_user),
        );
        Some(asset_canister_id)
    } else {
        None
    };

    permit_change_operation(&env, &canister_ids);
    set_four_eyes_on_change(&env, &canister_ids);

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("test"), test_canister)],
        ..Default::default()
    };

    let mut wasm = NamedTempFile::new().unwrap();
    let module_bytes = get_canister_wasm("test_canister");
    let max_chunk_len = module_bytes.len() / 2;
    assert!(max_chunk_len > 0);
    let module_hash = hash(&module_bytes);
    wasm.write_all(&module_bytes).unwrap();

    let inner_args = RequestCanisterInstallArgs {
        canister: String::from("test"),
        mode: CanisterInstallModeArgs::Install,
        wasm: wasm.path().as_os_str().to_str().unwrap().to_string(),
        argument: None,
        arg_file: None,
        asset_canister: asset_canister.map(|p| p.to_text()),
        wasm_memory_persistence: None,
        skip_pre_upgrade: false,
    };

    let request = dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // Call the test canister
        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::Install(inner_args.clone()),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();

        let request = dfx_orbit.station.request(request.clone()).await.unwrap();

        // Check that the request verifies
        let req_response = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: request.request.id.clone(),
                with_full_info: Some(false),
            })
            .await
            .unwrap();

        VerifyArgs {
            request_id: request.request.id.clone(),
            and_approve: false,
            or_reject: false,
            action: VerifyArgsAction::Canister(VerifyCanisterArgs {
                action: VerifyCanisterActionArgs::Install(inner_args),
            }),
        }
        .verify(&dfx_orbit, &req_response)
        .await
        .unwrap();

        request.request
    });

    // Check that the canister is still empty.
    let status = canister_status(&env, Some(canister_ids.station), test_canister);
    assert_eq!(status.module_hash, None);

    // The other user approves the request
    submit_request_approval(
        &env,
        other_user,
        canister_ids.station,
        request.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    wait_for_request(&env, other_user, canister_ids.station, request).unwrap();

    // Check that the canister is installed now.
    let status = canister_status(&env, Some(canister_ids.station), test_canister);
    assert_eq!(status.module_hash, Some(module_hash));
}

/// Test that an upgrade request created with `--wasm-memory-persistence keep`
/// carries the option through the station round-trip and that `verify`
/// accepts it.
#[test]
fn canister_upgrade_wasm_memory_persistence() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let (_dfx_user, _) = setup_dfx_user(&env, &canister_ids);

    // create the test canister
    let test_canister = create_canister(&env, canister_ids.station);

    permit_change_operation(&env, &canister_ids);
    set_four_eyes_on_change(&env, &canister_ids);

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("test"), test_canister)],
        ..Default::default()
    };

    let mut wasm = NamedTempFile::new().unwrap();
    let module_bytes = get_canister_wasm("test_canister");
    wasm.write_all(&module_bytes).unwrap();

    let inner_args = RequestCanisterInstallArgs {
        canister: String::from("test"),
        mode: CanisterInstallModeArgs::Upgrade,
        wasm: wasm.path().as_os_str().to_str().unwrap().to_string(),
        argument: None,
        arg_file: None,
        asset_canister: None,
        wasm_memory_persistence: Some(WasmMemoryPersistenceArgs::Keep),
        skip_pre_upgrade: false,
    };

    dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::Install(inner_args.clone()),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();

        let request = dfx_orbit.station.request(request.clone()).await.unwrap();

        let req_response = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: request.request.id.clone(),
                with_full_info: Some(false),
            })
            .await
            .unwrap();

        // The stored operation must retain the `keep` persistence option.
        let RequestOperationDTO::ChangeExternalCanister(op) = &req_response.request.operation
        else {
            panic!("expected a change external canister operation");
        };
        assert!(matches!(
            &op.mode,
            CanisterInstallMode::Upgrade(Some(opts))
                if opts.wasm_memory_persistence == Some(WasmMemoryPersistence::Keep)
        ));

        // Verifying with the same options must succeed.
        VerifyArgs {
            request_id: request.request.id.clone(),
            and_approve: false,
            or_reject: false,
            action: VerifyArgsAction::Canister(VerifyCanisterArgs {
                action: VerifyCanisterActionArgs::Install(inner_args),
            }),
        }
        .verify(&dfx_orbit, &req_response)
        .await
        .unwrap();

        request.request
    });
}
