/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfSaslAuthenticateRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfSaslAuthenticateRequest {
    /// The SASL authentication bytes from the client, as defined by the SASL mechanism.
    pub auth_bytes: Vec<u8>,
}

// -----------------------------------
// KfSaslAuthenticateResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfSaslAuthenticateResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: ErrorCode,

    /// The error message, or null if there was no error.
    pub error_message: Option<String>,

    /// The SASL authentication bytes from the server, as defined by the SASL mechanism.
    pub auth_bytes: Vec<u8>,

    /// The SASL authentication bytes from the server, as defined by the SASL mechanism.
    #[fluvio_kf(min_version = 1)]
    pub session_lifetime_ms: i64,
}

// -----------------------------------
// Implementation - KfSaslAuthenticateRequest
// -----------------------------------

impl Request for KfSaslAuthenticateRequest {
    const API_KEY: u16 = 36;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 1;
    const DEFAULT_API_VERSION: i16 = 1;

    type Response = KfSaslAuthenticateResponse;
}
