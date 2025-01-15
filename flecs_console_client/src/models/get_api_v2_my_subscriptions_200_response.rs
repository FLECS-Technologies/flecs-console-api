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
pub struct GetApiV2MySubscriptions200Response {
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    #[serde(rename = "statusText", skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
    #[serde(rename = "data")]
    pub data: Box<models::Subscriptions>,
}

impl GetApiV2MySubscriptions200Response {
    pub fn new(
        status_code: i32,
        data: models::Subscriptions,
    ) -> GetApiV2MySubscriptions200Response {
        GetApiV2MySubscriptions200Response {
            status_code,
            status_text: None,
            data: Box::new(data),
        }
    }
}
