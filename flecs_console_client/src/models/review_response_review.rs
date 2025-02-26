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
pub struct ReviewResponseReview {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "date_created")]
    pub date_created: String,
    #[serde(rename = "date_created_gmt")]
    pub date_created_gmt: String,
    #[serde(rename = "product_id")]
    pub product_id: i32,
    #[serde(rename = "product_name")]
    pub product_name: String,
    #[serde(rename = "product_permalink")]
    pub product_permalink: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "reviewer")]
    pub reviewer: String,
    #[serde(rename = "reviewer_email")]
    pub reviewer_email: String,
    #[serde(rename = "review")]
    pub review: String,
    #[serde(rename = "rating")]
    pub rating: f64,
    #[serde(rename = "verified")]
    pub verified: bool,
}

impl ReviewResponseReview {
    pub fn new(
        id: i32,
        date_created: String,
        date_created_gmt: String,
        product_id: i32,
        product_name: String,
        product_permalink: String,
        status: String,
        reviewer: String,
        reviewer_email: String,
        review: String,
        rating: f64,
        verified: bool,
    ) -> ReviewResponseReview {
        ReviewResponseReview {
            id,
            date_created,
            date_created_gmt,
            product_id,
            product_name,
            product_permalink,
            status,
            reviewer,
            reviewer_email,
            review,
            rating,
            verified,
        }
    }
}
