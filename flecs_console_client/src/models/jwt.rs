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
pub struct Jwt {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "token_expires", skip_serializing_if = "Option::is_none")]
    pub token_expires: Option<i32>,
}

impl Jwt {
    pub fn new() -> Jwt {
        Jwt {
            token: None,
            token_expires: None,
        }
    }
}