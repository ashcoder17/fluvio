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
// KfListGroupsRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfListGroupsRequest {}

// -----------------------------------
// KfListGroupsResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfListGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    #[fluvio_kf(min_version = 1, ignorable)]
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: ErrorCode,

    /// Each group in the response.
    pub groups: Vec<ListedGroup>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct ListedGroup {
    /// The group ID.
    pub group_id: String,

    /// The group protocol type.
    pub protocol_type: String,
}

// -----------------------------------
// Implementation - KfListGroupsRequest
// -----------------------------------

impl Request for KfListGroupsRequest {
    const API_KEY: u16 = 16;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 2;
    const DEFAULT_API_VERSION: i16 = 2;

    type Response = KfListGroupsResponse;
}
