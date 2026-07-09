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
use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use station_api::{GetRequestInput, RequestApprovalStatusDTO, RequestStatusDTO};
use std::io::{Read, Write};
use tempfile::NamedTempFile;

fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Appends a custom section to a Wasm module. The IC decides whether a module
/// supports Enhanced Orthogonal Persistence by the presence of the
/// `icp:private enhanced-orthogonal-persistence` custom section, so appending
/// it turns the plain test canister into an EOP module from the IC's
/// perspective.
fn append_wasm_custom_section(module: &mut Vec<u8>, name: &str, payload: &[u8]) {
    fn write_leb128(mut value: usize, out: &mut Vec<u8>) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            out.push(byte);
            if value == 0 {
                break;
            }
        }
    }

    let mut contents = Vec::new();
    write_leb128(name.len(), &mut contents);
    contents.extend_from_slice(name.as_bytes());
    contents.extend_from_slice(payload);

    module.push(0); // custom section id
    write_leb128(contents.len(), module);
    module.extend_from_slice(&contents);
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
        wasm_memory_persistence: None,
        skip_pre_upgrade: false,
        wasm: wasm.path().as_os_str().to_str().unwrap().to_string(),
        argument: None,
        arg_file: None,
        asset_canister: asset_canister.map(|p| p.to_text()),
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

/// Test that `--wasm-memory-persistence` and `--skip-pre-upgrade` are plumbed
/// through into the upgrade request, survive the round-trip through the
/// station (so `verify` accepts a matching request and rejects a mismatched
/// one), and are forwarded to the IC when the approved upgrade executes.
///
/// The upgrade requests `wasm_memory_persistence = replace`: the test canister
/// is not built with Enhanced Orthogonal Persistence, and the IC rejects
/// `keep` for such modules.
#[test]
fn canister_upgrade_options_round_trip() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let (_dfx_user, _) = setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    // Create and install the test canister, so that it can be upgraded later.
    let test_canister = create_canister(&env, canister_ids.station);
    let module_bytes = get_canister_wasm("test_canister");
    let module_hash = hash(&module_bytes);
    env.install_canister(
        test_canister,
        module_bytes.clone(),
        vec![],
        Some(canister_ids.station),
    );

    permit_change_operation(&env, &canister_ids);
    set_four_eyes_on_change(&env, &canister_ids);

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("test"), test_canister)],
        ..Default::default()
    };

    let mut wasm = NamedTempFile::new().unwrap();
    wasm.write_all(&module_bytes).unwrap();

    let install_args = RequestCanisterInstallArgs {
        canister: String::from("test"),
        mode: CanisterInstallModeArgs::Upgrade,
        wasm_memory_persistence: Some(WasmMemoryPersistenceArgs::Replace),
        skip_pre_upgrade: true,
        wasm: wasm.path().as_os_str().to_str().unwrap().to_string(),
        argument: None,
        arg_file: None,
        asset_canister: None,
    };

    let request = dfx_orbit_test(&mut env, config, async {
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::Install(install_args.clone()),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();

        let response = dfx_orbit.station.request(request.clone()).await.unwrap();

        let review_response = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: response.request.id.clone(),
                with_full_info: Some(false),
            })
            .await
            .unwrap();

        // The stored upgrade options match, so verification succeeds.
        VerifyArgs {
            request_id: response.request.id.clone(),
            and_approve: false,
            or_reject: false,
            action: VerifyArgsAction::Canister(VerifyCanisterArgs {
                action: VerifyCanisterActionArgs::Install(install_args.clone()),
            }),
        }
        .verify(&dfx_orbit, &review_response)
        .await
        .unwrap();

        // A mismatched `wasm_memory_persistence` must fail verification.
        let mut mismatched_args = install_args.clone();
        mismatched_args.wasm_memory_persistence = Some(WasmMemoryPersistenceArgs::Keep);
        VerifyArgs {
            request_id: response.request.id.clone(),
            and_approve: false,
            or_reject: false,
            action: VerifyArgsAction::Canister(VerifyCanisterArgs {
                action: VerifyCanisterActionArgs::Install(mismatched_args),
            }),
        }
        .verify(&dfx_orbit, &review_response)
        .await
        .expect_err("verification must reject a mismatched wasm_memory_persistence");

        response.request
    });

    // The other user approves the request; the upgrade carrying the upgrade
    // options executes successfully (a failure to forward the options to the
    // IC would leave the request failed and `wait_for_request` erroring).
    submit_request_approval(
        &env,
        other_user,
        canister_ids.station,
        request.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    wait_for_request(&env, other_user, canister_ids.station, request).unwrap();

    // The canister still runs the same module after the upgrade.
    let status = canister_status(&env, Some(canister_ids.station), test_canister);
    assert_eq!(status.module_hash, Some(module_hash));
}

/// Test upgrading a canister that runs a module declaring Enhanced Orthogonal
/// Persistence — the actual use case motivating `--wasm-memory-persistence`.
/// The IC refuses to upgrade such canisters unless
/// `wasm_memory_persistence = keep` is set (a safety check against
/// accidentally dropping their main memory), so the same upgrade request must
/// fail without the flag and execute successfully with it.
#[test]
fn canister_upgrade_with_wasm_memory_persistence_keep() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let (_dfx_user, _) = setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    // Mark the test module as supporting Enhanced Orthogonal Persistence and
    // install it, so that upgrades require `wasm_memory_persistence = keep`.
    // The bundled module is gzipped and must be decompressed before appending
    // the custom section: the IC reads the uncompressed size of gzipped
    // modules from the trailing bytes of the gzip stream, so bytes appended
    // after the stream would corrupt the module.
    let test_canister = create_canister(&env, canister_ids.station);
    let mut module_bytes = Vec::new();
    GzDecoder::new(get_canister_wasm("test_canister").as_slice())
        .read_to_end(&mut module_bytes)
        .unwrap();
    append_wasm_custom_section(
        &mut module_bytes,
        "icp:private enhanced-orthogonal-persistence",
        &[],
    );
    let module_hash = hash(&module_bytes);
    env.install_canister(
        test_canister,
        module_bytes.clone(),
        vec![],
        Some(canister_ids.station),
    );

    permit_change_operation(&env, &canister_ids);
    set_four_eyes_on_change(&env, &canister_ids);

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("test"), test_canister)],
        ..Default::default()
    };

    let mut wasm = NamedTempFile::new().unwrap();
    wasm.write_all(&module_bytes).unwrap();

    let install_args = |wasm_memory_persistence| RequestCanisterInstallArgs {
        canister: String::from("test"),
        mode: CanisterInstallModeArgs::Upgrade,
        wasm_memory_persistence,
        skip_pre_upgrade: false,
        wasm: wasm.path().as_os_str().to_str().unwrap().to_string(),
        argument: None,
        arg_file: None,
        asset_canister: None,
    };
    let args_without_keep = install_args(None);
    let args_with_keep = install_args(Some(WasmMemoryPersistenceArgs::Keep));

    let (request_without_keep, request_with_keep) = dfx_orbit_test(&mut env, config, async {
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;
        let dfx_orbit = &dfx_orbit;

        let submit_upgrade = move |args: RequestCanisterInstallArgs| async move {
            let request = RequestArgs {
                title: None,
                summary: None,
                action: RequestArgsActions::Canister(RequestCanisterArgs {
                    action: RequestCanisterActionArgs::Install(args),
                }),
            }
            .into_request(dfx_orbit)
            .await
            .unwrap();

            dfx_orbit.station.request(request).await.unwrap().request
        };

        let request_without_keep = submit_upgrade(args_without_keep).await;
        let request_with_keep = submit_upgrade(args_with_keep).await;
        (request_without_keep, request_with_keep)
    });

    // Without `keep`, the IC refuses to drop the main memory of a canister
    // running an Enhanced Orthogonal Persistence module, so the upgrade fails
    // during execution. Confirm it failed for that reason — the `install_code`
    // call was rejected — rather than being rejected/cancelled at an earlier
    // stage or timing out.
    submit_request_approval(
        &env,
        other_user,
        canister_ids.station,
        request_without_keep.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    let failed_status =
        wait_for_request(&env, other_user, canister_ids.station, request_without_keep)
            .expect_err("the IC must reject an EOP upgrade without wasm_memory_persistence = keep")
            .expect("the request must reach a terminal failed state rather than time out");
    match failed_status {
        RequestStatusDTO::Failed { reason } => {
            let reason = reason.expect("a failed upgrade must carry a reason");
            // The station wraps the management-canister error in this prefix
            // (see ChangeExternalCanisterRequestExecute), so its presence
            // confirms the failure came from the rejected `install_code` call.
            // The IC's own wording is surfaced in the panic message rather than
            // asserted on, to avoid coupling the test to replica error strings.
            assert!(
                reason.contains("failed to install external canister"),
                "upgrade failed for an unexpected reason: {reason}"
            );
        }
        other => panic!("expected the upgrade to fail, got {other:?}"),
    }

    // With `keep`, the same upgrade executes successfully.
    submit_request_approval(
        &env,
        other_user,
        canister_ids.station,
        request_with_keep.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    wait_for_request(&env, other_user, canister_ids.station, request_with_keep).unwrap();

    // The canister still runs the (EOP-marked) module after the upgrade.
    let status = canister_status(&env, Some(canister_ids.station), test_canister);
    assert_eq!(status.module_hash, Some(module_hash));
}
