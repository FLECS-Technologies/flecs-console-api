/*
 * FLECS Console API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostApiV2DeviceLicenseActivateRequest {
    #[serde(rename = "licenseKey", skip_serializing_if = "Option::is_none")]
    pub license_key: Option<String>,
}

impl PostApiV2DeviceLicenseActivateRequest {
    pub fn new() -> PostApiV2DeviceLicenseActivateRequest {
        PostApiV2DeviceLicenseActivateRequest { license_key: None }
    }
}