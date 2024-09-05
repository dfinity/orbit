use crate::core::ic_cdk::api::{
    canister_balance, data_certificate, print, set_certified_data, time, trap,
};
use crate::SERVICE_NAME;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use ic_asset_certification::{Asset, AssetConfig, AssetEncoding, AssetRouter};
use ic_cdk_macros::query;
use ic_certification::HashTree;
use ic_http_certification::{HeaderField, HttpCertificationTree, HttpRequest, HttpResponse};
use orbit_essentials::metrics::with_metrics_registry;
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};

#[query(decoding_quota = 10000)]
fn http_request(req: HttpRequest) -> HttpResponse {
    serve_asset(&req)
}

thread_local! {
    static HTTP_TREE: Rc<RefCell<HttpCertificationTree>> = Default::default();
    static ASSET_ROUTER: RefCell<AssetRouter<'static>> = RefCell::new(AssetRouter::with_tree(HTTP_TREE.with(|tree| tree.clone())));
}

// Certification
pub fn certify_metrics() {
    // 1. Define the asset certification configurations.
    let encodings = vec![
        AssetEncoding::Brotli.default_config(),
        AssetEncoding::Gzip.default_config(),
    ];

    let asset_configs = vec![AssetConfig::File {
        path: "metrics".to_string(),
        content_type: Some("text/plain".to_string()),
        headers: get_asset_headers(vec![(
            "cache-control".to_string(),
            "public, no-cache, no-store".to_string(),
        )]),
        fallback_for: vec![],
        aliased_by: vec![],
        encodings: encodings.clone(),
    }];

    // 2. Collect all assets from the frontend build directory.
    let mut assets = Vec::new();
    with_metrics_registry(SERVICE_NAME, |registry| {
        registry
            .gauge_mut(
                "canister_cycles_balance",
                "cycles balance available to the canister",
            )
            .set(canister_balance() as f64);
    });
    with_metrics_registry(SERVICE_NAME, |registry| {
        registry
            .gauge_mut(
                "metrics_timestamp",
                "UNIX timestamp in nanoseconds when the metrics were exported",
            )
            .set(time() as f64);
    });
    let metrics_contents =
        with_metrics_registry(SERVICE_NAME, |registry| registry.export_metrics());
    assets.push(Asset::new(
        "/metrics",
        metrics_contents.unwrap_or_else(|e| e.to_string().as_bytes().to_vec()),
    ));

    ASSET_ROUTER.with_borrow_mut(|asset_router| {
        // 3. Certify the assets using the `certify_assets` function from the `ic-asset-certification` crate.
        if let Err(err) = asset_router.certify_assets(assets, asset_configs) {
            print(format!("Failed to certify assets: {}", err));
        } else {
            // 4. Set the canister's certified data.
            set_certified_data(&asset_router.root_hash());
        }
    });
}

// Handlers
fn serve_asset(req: &HttpRequest) -> HttpResponse<'static> {
    ASSET_ROUTER.with_borrow(|asset_router| {
        if let Ok((mut response, witness, expr_path)) = asset_router.serve_asset(req) {
            add_certificate_header(&mut response, &witness, &expr_path);

            response
        } else {
            trap("Failed to serve asset");
        }
    })
}

fn get_asset_headers(additional_headers: Vec<HeaderField>) -> Vec<HeaderField> {
    // set up the default headers and include additional headers provided by the caller
    let mut headers = vec![
        ("strict-transport-security".to_string(), "max-age=31536000; includeSubDomains".to_string()),
        ("x-frame-options".to_string(), "DENY".to_string()),
        ("x-content-type-options".to_string(), "nosniff".to_string()),
        ("content-security-policy".to_string(), "default-src 'self'; form-action 'self'; object-src 'none'; frame-ancestors 'none'; upgrade-insecure-requests; block-all-mixed-content".to_string()),
        ("referrer-policy".to_string(), "no-referrer".to_string()),
        ("permissions-policy".to_string(), "accelerometer=(),ambient-light-sensor=(),autoplay=(),battery=(),camera=(),display-capture=(),document-domain=(),encrypted-media=(),fullscreen=(),gamepad=(),geolocation=(),gyroscope=(),layout-animations=(self),legacy-image-formats=(self),magnetometer=(),microphone=(),midi=(),oversized-images=(self),payment=(),picture-in-picture=(),publickey-credentials-get=(),speaker-selection=(),sync-xhr=(self),unoptimized-images=(self),unsized-media=(self),usb=(),screen-wake-lock=(),web-share=(),xr-spatial-tracking=()".to_string()),
        ("cross-origin-embedder-policy".to_string(), "require-corp".to_string()),
        ("cross-origin-opener-policy".to_string(), "same-origin".to_string()),
    ];
    headers.extend(additional_headers);

    headers
}

const IC_CERTIFICATE_HEADER: &str = "IC-Certificate";
fn add_certificate_header(response: &mut HttpResponse, witness: &HashTree, expr_path: &[String]) {
    if let Some(certified_data) = data_certificate() {
        let witness = cbor_encode(witness);
        let expr_path = cbor_encode(&expr_path);

        response.add_header((
            IC_CERTIFICATE_HEADER.to_string(),
            format!(
                "certificate=:{}:, tree=:{}:, expr_path=:{}:, version=2",
                BASE64.encode(certified_data),
                BASE64.encode(witness),
                BASE64.encode(expr_path)
            ),
        ));
    }
}

// Encoding
fn cbor_encode(value: &impl Serialize) -> Vec<u8> {
    let mut serializer = serde_cbor::Serializer::new(Vec::new());
    serializer
        .self_describe()
        .expect("Failed to self describe CBOR");
    value
        .serialize(&mut serializer)
        .expect("Failed to serialize value");
    serializer.into_inner()
}
