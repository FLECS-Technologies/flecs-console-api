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
pub struct ReviewRequestData {
    #[serde(rename = "product_id")]
    pub product_id: i32,
    #[serde(rename = "review")]
    pub review: String,
    #[serde(rename = "reviewer")]
    pub reviewer: String,
    #[serde(rename = "reviewer_email")]
    pub reviewer_email: String,
    #[serde(rename = "rating")]
    pub rating: f64,
}

impl ReviewRequestData {
    pub fn new(
        product_id: i32,
        review: String,
        reviewer: String,
        reviewer_email: String,
        rating: f64,
    ) -> ReviewRequestData {
        ReviewRequestData {
            product_id,
            review,
            reviewer,
            reviewer_email,
            rating,
        }
    }
}
