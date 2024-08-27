mod console_error_panic_hook;

use schemars::gen::SchemaSettings;
use std::str::FromStr;
use stellar_xdr::curr::{Limits, Type, TypeVariant, WriteXdr};
use wasm_bindgen::prelude::*;

/// Returns a list of XDR types.
#[wasm_bindgen]
pub fn types() -> Vec<String> {
    TypeVariant::VARIANTS_STR
        .iter()
        .map(ToString::to_string)
        .collect()
}

/// Returns the JSON Schema for an XDR type.
///
/// JSON Schema version Draft 7 is returned.
#[wasm_bindgen]
pub fn schema(type_variant: &str) -> Result<String, String> {
    let t = TypeVariant::from_str(type_variant).map_err(|_| "unknown type".to_string())?;
    let settings = SchemaSettings::draft07();
    let generator = settings.into_generator();
    let schema = t.json_schema(generator);
    serde_json::to_string(&schema).map_err(|e| format!("{e}"))
}

/// Identifies which XDR types the given XDR can decode to completely.
///
/// Supports single XDR values only, not arrays, streams, or framed streams.
#[wasm_bindgen]
pub fn guess(xdr_base64: String) -> Vec<String> {
    // Base64 when decoded will have a length at or under this len.
    // Ref: https://datatracker.ietf.org/doc/html/rfc4648#page-5
    let decoded_max_len = xdr_base64.len() / 4 * 3;
    // Limit decoding attempts to within the maximum length of the known input.
    let limits = Limits::len(decoded_max_len);

    TypeVariant::VARIANTS
        .iter()
        .filter(|v| Type::from_xdr_base64(**v, &xdr_base64, limits.clone()).is_ok())
        .map(|v| v.name().to_string())
        .collect()
}

/// Decodes the XDR into JSON.
///
/// Accepts a XDR base64 string.
///
/// Returns a JSON string.
///
/// Unstable: The API of this function is unstable and will likely be changed to
/// return a JsValue instead of a JSON string.
#[wasm_bindgen]
pub fn decode(type_variant: String, xdr_base64: String) -> Result<String, String> {
    let type_variant = TypeVariant::from_str(&type_variant).map_err(|e| format!("{e}"))?;

    // Base64 when decoded will have a length at or under this len.
    // Ref: https://datatracker.ietf.org/doc/html/rfc4648#page-5
    let decoded_max_len = xdr_base64.len() / 4 * 3;
    // Limit decoding attempts to within the maximum length of the known input.
    let limits = Limits::len(decoded_max_len);

    let value =
        Type::from_xdr_base64(type_variant, xdr_base64, limits).map_err(|e| format!("{e}"))?;
    // TODO: Return a native JS value.
    // let js = serde_wasm_bindgen::to_value(&value).map_err(|e| format!("{e}"))?;
    let json = serde_json::to_string(&value).map_err(|e| format!("{e}"))?;
    Ok(json)
}

/// Encodes to XDR from JSON.
///
/// Accepts a JSON string.
///
/// Returns an XDR base64 string.
///
/// Unstable: The API of this function is unstable and will likely be changed to
/// accept a JsValue instead of a JSON string.
#[wasm_bindgen]
pub fn encode(type_variant: String, json: String) -> Result<String, String> {
    let type_variant = TypeVariant::from_str(&type_variant).map_err(|e| format!("{e}"))?;
    // TODO: Return a native JS value.
    // let t: Type = serde_wasm_bindgen::from_value(js).map_err(|e| format!("{e}"))?;
    let t = Type::read_json(type_variant, json.as_bytes()).map_err(|e| format!("{e}"))?;
    let b64 = t
        .to_xdr_base64(Limits::none())
        .map_err(|e| format!("{e}"))?;
    Ok(b64)
}
