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
pub struct User {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "user_email", skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(rename = "user_login", skip_serializing_if = "Option::is_none")]
    pub user_login: Option<String>,
}

impl User {
    pub fn new() -> User {
        User {
            id: None,
            display_name: None,
            user_email: None,
            user_login: None,
        }
    }
}
