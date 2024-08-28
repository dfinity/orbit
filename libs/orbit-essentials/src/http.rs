use crate::api::{HeaderField, HttpResponse};
use crate::cdk::api::data_certificate;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use ic_certification::{labeled, leaf, HashTree};
use ic_http_certification::DefaultCelBuilder;
use ic_representation_independent_hash::hash;
use serde::Serialize;

// Certify that frontend asset certification is skipped for this canister.

const IC_CERTIFICATE_HEADER: &str = "IC-Certificate";
const IC_CERTIFICATE_EXPRESSION_HEADER: &str = "IC-CertificateExpression";

fn skip_certification_cel_expr() -> String {
    DefaultCelBuilder::skip_certification().to_string()
}

fn skip_certification_asset_tree() -> HashTree {
    let cel_expr_hash = hash(skip_certification_cel_expr().as_bytes());
    labeled(
        "http_expr",
        labeled("<*>", labeled(cel_expr_hash, leaf(vec![]))),
    )
}

pub fn add_skip_certification_headers(response: &mut HttpResponse) {
    if let Some(certified_data) = data_certificate() {
        let witness = cbor_encode(&skip_certification_asset_tree());
        let expr_path = ["http_expr", "<*>"];
        let expr_path = cbor_encode(&expr_path);

        response.headers.push(HeaderField(
            IC_CERTIFICATE_EXPRESSION_HEADER.to_string(),
            skip_certification_cel_expr(),
        ));
        response.headers.push(HeaderField(
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

pub fn certified_data_for_skip_certification() -> [u8; 32] {
    skip_certification_asset_tree().digest()
}
