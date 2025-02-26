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
pub struct Address {
    #[serde(rename = "street_1")]
    pub street_1: String,
    #[serde(rename = "street_2")]
    pub street_2: String,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "zip")]
    pub zip: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "state")]
    pub state: String,
}

impl Address {
    pub fn new(
        street_1: String,
        street_2: String,
        city: String,
        zip: String,
        country: String,
        state: String,
    ) -> Address {
        Address {
            street_1,
            street_2,
            city,
            zip,
            country,
            state,
        }
    }
}
